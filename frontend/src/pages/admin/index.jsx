import React, { useCallback, useEffect, useMemo, useState } from "react";
import { adminApi } from "../../api/admin";
import { EMOTION_EMOJI_MAP, EMOTION_LABELS } from "../../constants/emotions";
import { resolveAvatarUrl } from "../../utils";

const PAGE_SIZE = 10;

const tabs = [
  { key: "dashboard", label: "数据概览", icon: "fa-dashboard" },
  { key: "users", label: "用户管理", icon: "fa-users" },
  { key: "rooms", label: "自习室管理", icon: "fa-book" },
  { key: "logs", label: "日志审计", icon: "fa-file-text" },
];

const colorMap = {
  primary: { bg: "bg-primary/10", text: "text-primary" },
  secondary: { bg: "bg-secondary/10", text: "text-secondary" },
  success: { bg: "bg-success/10", text: "text-success" },
  warning: { bg: "bg-warning/10", text: "text-warning" },
};

const actionLabels = {
  admin_login: "管理员登录",
  user_disable: "禁用用户",
  user_enable: "启用用户",
  room_close: "关闭自习室",
  room_open: "开放自习室",
};

const targetLabels = {
  admin: "管理员",
  user: "用户",
  room: "自习室",
};

function cleanParams(params) {
  return Object.fromEntries(
    Object.entries(params).filter(([, value]) => value !== undefined && value !== null && value !== "")
  );
}

function formatDate(value) {
  if (!value) return "-";
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return "-";
  return date.toLocaleString("zh-CN", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function getErrorMessage(error, fallback) {
  return error?.response?.data?.message || error?.message || fallback;
}

function StatusBadge({ status }) {
  const isGood = status === "active" || status === "open";
  const text = status === "active" ? "正常" : status === "disabled" ? "禁用" : status === "open" ? "开放" : "关闭";
  return (
    <span className={`inline-flex items-center px-2.5 py-1 rounded-full text-xs font-medium ${isGood ? "bg-green-50 text-green-700" : "bg-red-50 text-red-700"}`}>
      {text}
    </span>
  );
}

function Pagination({ page, pageSize, total, onPageChange }) {
  const totalPages = Math.max(1, Math.ceil((total || 0) / (pageSize || PAGE_SIZE)));
  return (
    <div className="flex items-center justify-between px-4 py-3 border-t border-gray-100 text-sm text-gray-600">
      <span>共 {total || 0} 条，第 {page || 1} / {totalPages} 页</span>
      <div className="flex gap-2">
        <button
          type="button"
          disabled={(page || 1) <= 1}
          onClick={() => onPageChange((page || 1) - 1)}
          className="px-3 py-1.5 border border-gray-200 rounded-lg disabled:opacity-40 disabled:cursor-not-allowed hover:bg-gray-50"
        >
          上一页
        </button>
        <button
          type="button"
          disabled={(page || 1) >= totalPages}
          onClick={() => onPageChange((page || 1) + 1)}
          className="px-3 py-1.5 border border-gray-200 rounded-lg disabled:opacity-40 disabled:cursor-not-allowed hover:bg-gray-50"
        >
          下一页
        </button>
      </div>
    </div>
  );
}

function EmptyRow({ colSpan, icon, text }) {
  return (
    <tr>
      <td colSpan={colSpan} className="py-12 text-center text-gray-400">
        <i className={`fa ${icon} text-3xl mb-2 block`}></i>
        <p className="text-sm">{text}</p>
      </td>
    </tr>
  );
}

function Avatar({ src, name }) {
  return (
    <img
      src={resolveAvatarUrl(src, name)}
      alt={name || "avatar"}
      className="w-9 h-9 rounded-full object-cover bg-gray-100 shrink-0"
      onError={(event) => { event.currentTarget.src = resolveAvatarUrl("", name); }}
    />
  );
}

export default function AdminPage({ setCurrentPage, setUserRole }) {
  const [tab, setTab] = useState("dashboard");
  const [dashboard, setDashboard] = useState(null);
  const [dashboardLoading, setDashboardLoading] = useState(false);
  const [dashboardError, setDashboardError] = useState("");

  const [userDraft, setUserDraft] = useState({ keyword: "", status: "" });
  const [userQuery, setUserQuery] = useState({ keyword: "", status: "", page: 1, pageSize: PAGE_SIZE });
  const [users, setUsers] = useState({ items: [], total: 0, page: 1, pageSize: PAGE_SIZE });
  const [usersLoading, setUsersLoading] = useState(false);
  const [usersError, setUsersError] = useState("");

  const [roomDraft, setRoomDraft] = useState({ keyword: "", status: "" });
  const [roomQuery, setRoomQuery] = useState({ keyword: "", status: "", page: 1, pageSize: PAGE_SIZE });
  const [rooms, setRooms] = useState({ items: [], total: 0, page: 1, pageSize: PAGE_SIZE });
  const [roomsLoading, setRoomsLoading] = useState(false);
  const [roomsError, setRoomsError] = useState("");

  const [logDraft, setLogDraft] = useState({ type: "", startDate: "", endDate: "" });
  const [logQuery, setLogQuery] = useState({ type: "", startDate: "", endDate: "", page: 1, pageSize: PAGE_SIZE });
  const [logs, setLogs] = useState({ items: [], total: 0, page: 1, pageSize: PAGE_SIZE });
  const [logsLoading, setLogsLoading] = useState(false);
  const [logsError, setLogsError] = useState("");

  const [actionLoading, setActionLoading] = useState("");

  const loadDashboard = useCallback(async () => {
    setDashboardLoading(true);
    setDashboardError("");
    try {
      const data = await adminApi.getDashboard();
      setDashboard(data || null);
    } catch (error) {
      setDashboardError(getErrorMessage(error, "后台概览加载失败"));
    } finally {
      setDashboardLoading(false);
    }
  }, []);

  const loadUsers = useCallback(async () => {
    setUsersLoading(true);
    setUsersError("");
    try {
      const data = await adminApi.getUsers(cleanParams(userQuery));
      setUsers(data || { items: [], total: 0, page: 1, pageSize: PAGE_SIZE });
    } catch (error) {
      setUsersError(getErrorMessage(error, "用户列表加载失败"));
    } finally {
      setUsersLoading(false);
    }
  }, [userQuery]);

  const loadRooms = useCallback(async () => {
    setRoomsLoading(true);
    setRoomsError("");
    try {
      const data = await adminApi.getRooms(cleanParams(roomQuery));
      setRooms(data || { items: [], total: 0, page: 1, pageSize: PAGE_SIZE });
    } catch (error) {
      setRoomsError(getErrorMessage(error, "自习室列表加载失败"));
    } finally {
      setRoomsLoading(false);
    }
  }, [roomQuery]);

  const loadLogs = useCallback(async () => {
    setLogsLoading(true);
    setLogsError("");
    try {
      const data = await adminApi.getAuditLogs(cleanParams(logQuery));
      setLogs(data || { items: [], total: 0, page: 1, pageSize: PAGE_SIZE });
    } catch (error) {
      setLogsError(getErrorMessage(error, "审计日志加载失败"));
    } finally {
      setLogsLoading(false);
    }
  }, [logQuery]);

  useEffect(() => {
    if (tab === "dashboard") loadDashboard();
    if (tab === "users") loadUsers();
    if (tab === "rooms") loadRooms();
    if (tab === "logs") loadLogs();
  }, [tab, loadDashboard, loadUsers, loadRooms, loadLogs]);

  const dashboardCards = useMemo(() => [
    { label: "当前在线", value: dashboard?.currentOnlineUsers ?? 0, unit: "人", icon: "fa-users", color: "primary" },
    { label: "活跃用户", value: dashboard?.activeUsers ?? 0, unit: "人", icon: "fa-user-plus", color: "secondary" },
    { label: "今日打卡", value: dashboard?.todayCheckins ?? 0, unit: "次", icon: "fa-check-circle", color: "success" },
    { label: "房间总数", value: dashboard?.totalRooms ?? 0, unit: "间", icon: "fa-home", color: "warning" },
  ], [dashboard]);

  const emotionTotal = useMemo(() => {
    return (dashboard?.emotionDistribution || []).reduce((sum, item) => sum + (Number(item.count) || 0), 0);
  }, [dashboard]);

  const handleLogout = () => {
    localStorage.removeItem("token");
    localStorage.removeItem("role");
    if (setUserRole) setUserRole("user");
    setCurrentPage("login");
  };

  const submitUserFilters = (event) => {
    event.preventDefault();
    setUserQuery({ ...userDraft, page: 1, pageSize: PAGE_SIZE });
  };

  const submitRoomFilters = (event) => {
    event.preventDefault();
    setRoomQuery({ ...roomDraft, page: 1, pageSize: PAGE_SIZE });
  };

  const submitLogFilters = (event) => {
    event.preventDefault();
    setLogQuery({ ...logDraft, page: 1, pageSize: PAGE_SIZE });
  };

  const updateUserStatus = async (user) => {
    const nextStatus = user.status === "active" ? "disabled" : "active";
    const label = nextStatus === "disabled" ? "禁用" : "启用";
    if (!window.confirm(`确定要${label}用户「${user.nickname}」吗？`)) return;
    const reason = window.prompt(`请输入${label}原因，可留空：`, "");
    if (reason === null) return;

    setActionLoading(`user-${user.userId}`);
    try {
      await adminApi.updateUserStatus(user.userId, { status: nextStatus, reason: reason.trim() || undefined });
      await Promise.all([loadUsers(), loadDashboard()]);
    } catch (error) {
      alert(getErrorMessage(error, `${label}用户失败`));
    } finally {
      setActionLoading("");
    }
  };

  const updateRoomStatus = async (room) => {
    const nextStatus = room.status === "open" ? "closed" : "open";
    const label = nextStatus === "closed" ? "关闭" : "开放";
    if (!window.confirm(`确定要${label}自习室「${room.name}」吗？`)) return;
    const reason = window.prompt(`请输入${label}原因，可留空：`, "");
    if (reason === null) return;

    setActionLoading(`room-${room.roomId}`);
    try {
      await adminApi.updateRoomStatus(room.roomId, { status: nextStatus, reason: reason.trim() || undefined });
      await Promise.all([loadRooms(), loadDashboard()]);
    } catch (error) {
      alert(getErrorMessage(error, `${label}自习室失败`));
    } finally {
      setActionLoading("");
    }
  };

  const renderDashboard = () => (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold text-dark">数据概览</h2>
        <button type="button" onClick={loadDashboard} disabled={dashboardLoading} className="px-4 py-2 rounded-lg bg-white border border-gray-200 text-sm hover:bg-gray-50 disabled:opacity-50">
          <i className="fa fa-refresh mr-2"></i>{dashboardLoading ? "刷新中" : "刷新"}
        </button>
      </div>

      {dashboardError && <div className="mb-4 rounded-lg bg-red-50 text-red-600 px-4 py-3 text-sm">{dashboardError}</div>}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        {dashboardCards.map((card) => {
          const color = colorMap[card.color] || colorMap.primary;
          return (
            <div key={card.label} className="bg-white rounded-xl shadow-md p-6">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-sm font-medium text-gray-500">{card.label}</h3>
                <div className={`w-10 h-10 rounded-full ${color.bg} flex items-center justify-center ${color.text}`}>
                  <i className={`fa ${card.icon}`}></i>
                </div>
              </div>
              <p className="text-3xl font-bold text-dark">
                {dashboardLoading ? "--" : card.value}<span className="text-base font-normal text-gray-500 ml-1">{card.unit}</span>
              </p>
            </div>
          );
        })}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
        <div className="bg-white rounded-xl shadow-md p-6">
          <h3 className="text-lg font-semibold mb-5">今日学习概览</h3>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div className="rounded-lg bg-gray-50 p-4">
              <p className="text-gray-500 mb-1">今日学习时长</p>
              <p className="text-2xl font-bold text-dark">{dashboard?.todayStudyHours ?? 0}<span className="text-base font-normal text-gray-500 ml-1">小时</span></p>
            </div>
            <div className="rounded-lg bg-gray-50 p-4">
              <p className="text-gray-500 mb-1">今日学习分钟</p>
              <p className="text-2xl font-bold text-dark">{dashboard?.todayStudyMinutes ?? 0}<span className="text-base font-normal text-gray-500 ml-1">分钟</span></p>
            </div>
            <div className="rounded-lg bg-gray-50 p-4">
              <p className="text-gray-500 mb-1">用户总数</p>
              <p className="text-2xl font-bold text-dark">{dashboard?.totalUsers ?? 0}<span className="text-base font-normal text-gray-500 ml-1">人</span></p>
            </div>
            <div className="rounded-lg bg-gray-50 p-4">
              <p className="text-gray-500 mb-1">禁用用户</p>
              <p className="text-2xl font-bold text-dark">{dashboard?.disabledUsers ?? 0}<span className="text-base font-normal text-gray-500 ml-1">人</span></p>
            </div>
            <div className="rounded-lg bg-gray-50 p-4">
              <p className="text-gray-500 mb-1">开放房间</p>
              <p className="text-2xl font-bold text-dark">{dashboard?.openRooms ?? 0}<span className="text-base font-normal text-gray-500 ml-1">间</span></p>
            </div>
            <div className="rounded-lg bg-gray-50 p-4">
              <p className="text-gray-500 mb-1">关闭房间</p>
              <p className="text-2xl font-bold text-dark">{dashboard?.closedRooms ?? 0}<span className="text-base font-normal text-gray-500 ml-1">间</span></p>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-xl shadow-md p-6">
          <h3 className="text-lg font-semibold mb-5">情绪分布</h3>
          {emotionTotal === 0 ? (
            <div className="h-64 flex items-center justify-center bg-gray-50 rounded-lg text-gray-400">
              <div className="text-center">
                <i className="fa fa-pie-chart text-4xl mb-2"></i>
                <p className="text-sm">今日暂无情绪记录</p>
              </div>
            </div>
          ) : (
            <div className="space-y-4">
              {EMOTION_LABELS.map((label) => {
                const item = (dashboard?.emotionDistribution || []).find((row) => row.emotionTag === label);
                const count = Number(item?.count) || 0;
                const percent = emotionTotal ? Math.round((count / emotionTotal) * 100) : 0;
                return (
                  <div key={label}>
                    <div className="flex items-center justify-between mb-1 text-sm">
                      <span className="text-gray-700"><span className="mr-2">{EMOTION_EMOJI_MAP[label] || "💭"}</span>{label}</span>
                      <span className="text-gray-500">{count} 次 · {percent}%</span>
                    </div>
                    <div className="h-2 rounded-full bg-gray-100 overflow-hidden">
                      <div className="h-full rounded-full bg-primary" style={{ width: `${percent}%` }}></div>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      </div>
    </div>
  );

  const renderUsers = () => (
    <div>
      <h2 className="text-2xl font-bold mb-6 text-dark">用户管理</h2>
      <form onSubmit={submitUserFilters} className="bg-white rounded-xl shadow-md p-4 mb-6 flex flex-wrap gap-4 items-center">
        <div className="relative flex-1 min-w-[220px]">
          <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-400"><i className="fa fa-search"></i></span>
          <input
            type="text"
            placeholder="搜索手机号或昵称..."
            value={userDraft.keyword}
            onChange={(event) => setUserDraft((prev) => ({ ...prev, keyword: event.target.value }))}
            className="w-full pl-10 pr-4 py-2 border border-gray-200 rounded-lg bg-white outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary"
          />
        </div>
        <select
          value={userDraft.status}
          onChange={(event) => setUserDraft((prev) => ({ ...prev, status: event.target.value }))}
          className="px-4 py-2 border border-gray-200 rounded-lg bg-white outline-none"
        >
          <option value="">全部状态</option>
          <option value="active">正常</option>
          <option value="disabled">禁用</option>
        </select>
        <button type="submit" className="px-4 py-2 rounded-lg bg-primary text-white hover:opacity-90">查询</button>
        <button type="button" onClick={loadUsers} disabled={usersLoading} className="px-4 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 disabled:opacity-50">刷新</button>
      </form>

      {usersError && <div className="mb-4 rounded-lg bg-red-50 text-red-600 px-4 py-3 text-sm">{usersError}</div>}

      <div className="bg-white rounded-xl shadow-md overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">用户</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">手机号</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">注册时间</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">连续打卡</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">状态</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">操作</th>
            </tr>
          </thead>
          <tbody>
            {usersLoading ? <EmptyRow colSpan={6} icon="fa-spinner fa-spin" text="正在加载用户列表" /> : users.items.length === 0 ? <EmptyRow colSpan={6} icon="fa-users" text="暂无用户数据" /> : users.items.map((user) => (
              <tr key={user.userId} className="border-b border-gray-100 hover:bg-gray-50/60">
                <td className="py-3 px-4">
                  <div className="flex items-center gap-3">
                    <Avatar src={user.avatarUrl} name={user.nickname} />
                    <div>
                      <p className="font-medium text-gray-800">{user.nickname || "未命名用户"}</p>
                      <p className="text-xs text-gray-400">ID: {user.userId}</p>
                    </div>
                  </div>
                </td>
                <td className="py-3 px-4 text-sm text-gray-600">{user.phone}</td>
                <td className="py-3 px-4 text-sm text-gray-600">{formatDate(user.createdAt)}</td>
                <td className="py-3 px-4 text-sm text-gray-600">{user.streakDays || 0} 天</td>
                <td className="py-3 px-4"><StatusBadge status={user.status} /></td>
                <td className="py-3 px-4">
                  <button
                    type="button"
                    disabled={actionLoading === `user-${user.userId}`}
                    onClick={() => updateUserStatus(user)}
                    className={`px-3 py-1.5 rounded-lg text-sm ${user.status === "active" ? "bg-red-50 text-red-600 hover:bg-red-100" : "bg-green-50 text-green-700 hover:bg-green-100"} disabled:opacity-50`}
                  >
                    {actionLoading === `user-${user.userId}` ? "处理中" : user.status === "active" ? "禁用" : "启用"}
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        <Pagination page={users.page} pageSize={users.pageSize} total={users.total} onPageChange={(page) => setUserQuery((prev) => ({ ...prev, page }))} />
      </div>
    </div>
  );

  const renderRooms = () => (
    <div>
      <h2 className="text-2xl font-bold mb-6 text-dark">自习室管理</h2>
      <form onSubmit={submitRoomFilters} className="bg-white rounded-xl shadow-md p-4 mb-6 flex flex-wrap gap-4 items-center">
        <div className="relative flex-1 min-w-[220px]">
          <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-400"><i className="fa fa-search"></i></span>
          <input
            type="text"
            placeholder="搜索房间名称..."
            value={roomDraft.keyword}
            onChange={(event) => setRoomDraft((prev) => ({ ...prev, keyword: event.target.value }))}
            className="w-full pl-10 pr-4 py-2 border border-gray-200 rounded-lg bg-white outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary"
          />
        </div>
        <select
          value={roomDraft.status}
          onChange={(event) => setRoomDraft((prev) => ({ ...prev, status: event.target.value }))}
          className="px-4 py-2 border border-gray-200 rounded-lg bg-white outline-none"
        >
          <option value="">全部状态</option>
          <option value="open">开放</option>
          <option value="closed">关闭</option>
        </select>
        <button type="submit" className="px-4 py-2 rounded-lg bg-primary text-white hover:opacity-90">查询</button>
        <button type="button" onClick={loadRooms} disabled={roomsLoading} className="px-4 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 disabled:opacity-50">刷新</button>
      </form>

      {roomsError && <div className="mb-4 rounded-lg bg-red-50 text-red-600 px-4 py-3 text-sm">{roomsError}</div>}

      <div className="bg-white rounded-xl shadow-md overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">房间名称</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">房主</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">人数/容量</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">开放时间</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">状态</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">操作</th>
            </tr>
          </thead>
          <tbody>
            {roomsLoading ? <EmptyRow colSpan={6} icon="fa-spinner fa-spin" text="正在加载自习室列表" /> : rooms.items.length === 0 ? <EmptyRow colSpan={6} icon="fa-book" text="暂无自习室数据" /> : rooms.items.map((room) => (
              <tr key={room.roomId} className="border-b border-gray-100 hover:bg-gray-50/60">
                <td className="py-3 px-4">
                  <p className="font-medium text-gray-800">{room.name}</p>
                  <p className="text-xs text-gray-400">ID: {room.roomId}{room.isPrivate ? " · 私密" : " · 公开"}</p>
                </td>
                <td className="py-3 px-4">
                  <div className="flex items-center gap-3">
                    <Avatar src={room.creator?.avatarUrl} name={room.creator?.nickname} />
                    <div>
                      <p className="text-sm text-gray-800">{room.creator?.nickname || "未知房主"}</p>
                      <p className="text-xs text-gray-400">ID: {room.creator?.userId || "-"}</p>
                    </div>
                  </div>
                </td>
                <td className="py-3 px-4 text-sm text-gray-600">{room.currentMembers || 0} / {room.capacity || 0}</td>
                <td className="py-3 px-4 text-sm text-gray-600">{formatDate(room.openAt)}</td>
                <td className="py-3 px-4"><StatusBadge status={room.status} /></td>
                <td className="py-3 px-4">
                  <button
                    type="button"
                    disabled={actionLoading === `room-${room.roomId}`}
                    onClick={() => updateRoomStatus(room)}
                    className={`px-3 py-1.5 rounded-lg text-sm ${room.status === "open" ? "bg-red-50 text-red-600 hover:bg-red-100" : "bg-green-50 text-green-700 hover:bg-green-100"} disabled:opacity-50`}
                  >
                    {actionLoading === `room-${room.roomId}` ? "处理中" : room.status === "open" ? "关闭" : "开放"}
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        <Pagination page={rooms.page} pageSize={rooms.pageSize} total={rooms.total} onPageChange={(page) => setRoomQuery((prev) => ({ ...prev, page }))} />
      </div>
    </div>
  );

  const renderLogs = () => (
    <div>
      <h2 className="text-2xl font-bold mb-6 text-dark">日志审计</h2>
      <form onSubmit={submitLogFilters} className="bg-white rounded-xl shadow-md p-4 mb-6 flex flex-wrap gap-4 items-center">
        <select
          value={logDraft.type}
          onChange={(event) => setLogDraft((prev) => ({ ...prev, type: event.target.value }))}
          className="px-4 py-2 border border-gray-200 rounded-lg bg-white outline-none"
        >
          <option value="">全部类型</option>
          <option value="login">管理员登录</option>
          <option value="disable">禁用用户</option>
          <option value="enable">启用用户</option>
          <option value="room_close">关闭自习室</option>
          <option value="room_open">开放自习室</option>
        </select>
        <input
          type="date"
          value={logDraft.startDate}
          onChange={(event) => setLogDraft((prev) => ({ ...prev, startDate: event.target.value }))}
          className="px-4 py-2 border border-gray-200 rounded-lg bg-white outline-none"
        />
        <input
          type="date"
          value={logDraft.endDate}
          onChange={(event) => setLogDraft((prev) => ({ ...prev, endDate: event.target.value }))}
          className="px-4 py-2 border border-gray-200 rounded-lg bg-white outline-none"
        />
        <button type="submit" className="px-4 py-2 rounded-lg bg-primary text-white hover:opacity-90">查询</button>
        <button type="button" onClick={loadLogs} disabled={logsLoading} className="px-4 py-2 rounded-lg border border-gray-200 hover:bg-gray-50 disabled:opacity-50">刷新</button>
      </form>

      {logsError && <div className="mb-4 rounded-lg bg-red-50 text-red-600 px-4 py-3 text-sm">{logsError}</div>}

      <div className="bg-white rounded-xl shadow-md overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">时间</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">操作者</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">类型</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">对象</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">原因</th>
            </tr>
          </thead>
          <tbody>
            {logsLoading ? <EmptyRow colSpan={5} icon="fa-spinner fa-spin" text="正在加载审计日志" /> : logs.items.length === 0 ? <EmptyRow colSpan={5} icon="fa-file-text" text="暂无审计日志" /> : logs.items.map((log) => (
              <tr key={log.logId} className="border-b border-gray-100 hover:bg-gray-50/60">
                <td className="py-3 px-4 text-sm text-gray-600">{formatDate(log.createdAt)}</td>
                <td className="py-3 px-4 text-sm text-gray-800">{log.adminName}<span className="text-xs text-gray-400 ml-2">ID: {log.adminId}</span></td>
                <td className="py-3 px-4 text-sm text-gray-600">{actionLabels[log.action] || log.action}</td>
                <td className="py-3 px-4 text-sm text-gray-600">{targetLabels[log.targetType] || log.targetType} #{log.targetId}</td>
                <td className="py-3 px-4 text-sm text-gray-600">{log.reason || "-"}</td>
              </tr>
            ))}
          </tbody>
        </table>
        <Pagination page={logs.page} pageSize={logs.pageSize} total={logs.total} onPageChange={(page) => setLogQuery((prev) => ({ ...prev, page }))} />
      </div>
    </div>
  );

  return (
    <div className="min-h-screen bg-gradient-to-br from-calm via-light to-purple-50">
      <div className="flex min-h-screen">
        <div className="w-64 bg-dark text-white">
          <div className="p-6 border-b border-gray-700">
            <h2 className="text-xl font-bold">管理后台</h2>
            <p className="text-xs text-gray-400 mt-1">AI情绪治愈自习打卡空间</p>
          </div>
          <nav className="flex-1 px-4 py-4 space-y-2">
            {tabs.map((item) => (
              <button
                key={item.key}
                type="button"
                onClick={() => setTab(item.key)}
                className={`w-full text-left px-4 py-3 rounded-lg transition-colors flex items-center gap-3 ${tab === item.key ? "bg-white/10 text-white" : "text-gray-300 hover:bg-white/10"}`}
              >
                <i className={`fa ${item.icon} w-5 text-center`}></i>
                {item.label}
              </button>
            ))}
          </nav>
          <div className="p-4 border-t border-gray-700">
            <button
              type="button"
              onClick={handleLogout}
              className="w-full text-left px-4 py-2 text-red-400 hover:bg-white/5 rounded-lg transition-colors flex items-center gap-3"
            >
              <i className="fa fa-sign-out"></i> 退出登录
            </button>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto p-8">
          {tab === "dashboard" && renderDashboard()}
          {tab === "users" && renderUsers()}
          {tab === "rooms" && renderRooms()}
          {tab === "logs" && renderLogs()}
        </div>
      </div>
    </div>
  );
}