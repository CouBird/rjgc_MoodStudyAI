import React, { useState } from "react";

export default function AdminPage({ setCurrentPage }) {
  const [tab, setTab] = useState("dashboard");

  const tabs = [
    { key: "dashboard", label: "数据概览", icon: "fa-dashboard" },
    { key: "users", label: "用户管理", icon: "fa-users" },
    { key: "rooms", label: "自习室管理", icon: "fa-book" },
    { key: "logs", label: "日志审计", icon: "fa-file-text" },
  ];

  // =========== Dashboard Stats Placeholder ===========
  const statsCards = [
    {
      label: "今日总在线",
      value: "--",
      icon: "fa-users",
      color: "primary",
      change: null,
    },
    {
      label: "今日新增用户",
      value: "--",
      icon: "fa-user-plus",
      color: "secondary",
      change: null,
    },
    {
      label: "今日有效打卡",
      value: "--",
      icon: "fa-check-circle",
      color: "success",
      change: null,
    },
    {
      label: "房间总数",
      value: "--",
      icon: "fa-home",
      color: "warning",
      change: null,
    },
  ];

  const colorMap = {
    primary: { bg: "bg-primary/10", text: "text-primary" },
    secondary: { bg: "bg-secondary/10", text: "text-secondary" },
    success: { bg: "bg-success/10", text: "text-success" },
    warning: { bg: "bg-warning/10", text: "text-warning" },
  };

  const renderDashboard = () => (
    <div>
      <h2 className="text-2xl font-bold mb-6 text-dark">数据概览</h2>
      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        {statsCards.map((card) => {
          const c = colorMap[card.color] || colorMap.primary;
          return (
            <div key={card.label} className="bg-white rounded-xl shadow-md p-6">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-sm font-medium text-gray-500">{card.label}</h3>
                <div className={"w-10 h-10 rounded-full " + c.bg + " flex items-center justify-center " + c.text}>
                  <i className={"fa " + card.icon}></i>
                </div>
              </div>
              <p className="text-3xl font-bold text-dark">{card.value}</p>
              <p className="text-xs text-gray-400 mt-2">等待后端 API (GET /admin/dashboard) 实现</p>
            </div>
          );
        })}
      </div>
      {/* Charts Placeholder */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
        <div className="bg-white rounded-xl shadow-md p-6">
          <h3 className="text-lg font-semibold mb-4">学习趋势</h3>
          <div className="h-64 flex items-center justify-center bg-gray-50 rounded-lg">
            <div className="text-center text-gray-400">
              <i className="fa fa-bar-chart text-4xl mb-2"></i>
              <p className="text-sm">图表等待后端 API 实现</p>
            </div>
          </div>
        </div>
        <div className="bg-white rounded-xl shadow-md p-6">
          <h3 className="text-lg font-semibold mb-4">情绪分布</h3>
          <div className="h-64 flex items-center justify-center bg-gray-50 rounded-lg">
            <div className="text-center text-gray-400">
              <i className="fa fa-pie-chart text-4xl mb-2"></i>
              <p className="text-sm">图表等待后端 API 实现</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );

  // =========== Users Management Placeholder ===========
  const renderUsers = () => (
    <div>
      <h2 className="text-2xl font-bold mb-6 text-dark">用户管理</h2>
      {/* Filters */}
      <div className="bg-white rounded-xl shadow-md p-4 mb-6 flex flex-wrap gap-4 items-center">
        <div className="relative flex-1 min-w-[200px]">
          <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-400">
            <i className="fa fa-search"></i>
          </span>
          <input
            type="text"
            placeholder="搜索手机号或昵称..."
            disabled
            className="w-full pl-10 pr-4 py-2 border border-gray-200 rounded-lg bg-gray-50 text-gray-400 cursor-not-allowed outline-none"
          />
        </div>
        <select disabled className="px-4 py-2 border border-gray-200 rounded-lg bg-gray-50 text-gray-400 cursor-not-allowed outline-none">
          <option>全部状态</option>
        </select>
        <span className="text-xs text-gray-400">等待后端 API (GET /admin/users) 实现</span>
      </div>
      {/* Table */}
      <div className="bg-white rounded-xl shadow-md overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">用户</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">手机号</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">注册时间</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">状态</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr className="border-b border-gray-100">
              <td colSpan={5} className="py-12 text-center text-gray-400">
                <i className="fa fa-users text-3xl mb-2 block"></i>
                <p className="text-sm">用户列表功能等待后端 API 实现</p>
                <p className="text-xs text-gray-300 mt-1">GET /admin/users, PATCH /admin/users/{"{userId}"}/status</p>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  );

  // =========== Rooms Management Placeholder ===========
  const renderRooms = () => (
    <div>
      <h2 className="text-2xl font-bold mb-6 text-dark">自习室管理</h2>
      {/* Filters */}
      <div className="bg-white rounded-xl shadow-md p-4 mb-6 flex flex-wrap gap-4 items-center">
        <div className="relative flex-1 min-w-[200px]">
          <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-400">
            <i className="fa fa-search"></i>
          </span>
          <input
            type="text"
            placeholder="搜索房间名称..."
            disabled
            className="w-full pl-10 pr-4 py-2 border border-gray-200 rounded-lg bg-gray-50 text-gray-400 cursor-not-allowed outline-none"
          />
        </div>
        <span className="text-xs text-gray-400">等待后端 API (GET /admin/rooms) 实现</span>
      </div>
      {/* Table */}
      <div className="bg-white rounded-xl shadow-md overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">房间名称</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">房主</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">人数/容量</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">状态</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr className="border-b border-gray-100">
              <td colSpan={5} className="py-12 text-center text-gray-400">
                <i className="fa fa-book text-3xl mb-2 block"></i>
                <p className="text-sm">自习室列表功能等待后端 API 实现</p>
                <p className="text-xs text-gray-300 mt-1">GET /admin/rooms, PATCH /admin/rooms/{"{roomId}"}/status</p>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  );

  // =========== Audit Logs Placeholder ===========
  const renderLogs = () => (
    <div>
      <h2 className="text-2xl font-bold mb-6 text-dark">日志审计</h2>
      {/* Filters */}
      <div className="bg-white rounded-xl shadow-md p-4 mb-6 flex flex-wrap gap-4 items-center">
        <select disabled className="px-4 py-2 border border-gray-200 rounded-lg bg-gray-50 text-gray-400 cursor-not-allowed outline-none">
          <option>全部类型</option>
        </select>
        <input
          type="date"
          disabled
          className="px-4 py-2 border border-gray-200 rounded-lg bg-gray-50 text-gray-400 cursor-not-allowed outline-none"
        />
        <span className="text-xs text-gray-400">等待后端 API (GET /admin/audit-logs) 实现</span>
      </div>
      {/* Table */}
      <div className="bg-white rounded-xl shadow-md overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">时间</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">操作者</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">类型</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">对象</th>
              <th className="text-left py-3 px-4 text-sm font-medium text-gray-500">详情</th>
            </tr>
          </thead>
          <tbody>
            <tr className="border-b border-gray-100">
              <td colSpan={5} className="py-12 text-center text-gray-400">
                <i className="fa fa-file-text text-3xl mb-2 block"></i>
                <p className="text-sm">审计日志功能等待后端 API 实现</p>
                <p className="text-xs text-gray-300 mt-1">GET /admin/audit-logs</p>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  );

  return (
    <div className="min-h-screen bg-gradient-to-br from-calm via-light to-purple-50">
      <div className="flex min-h-screen">
        {/* Sidebar */}
        <div className="w-64 bg-dark text-white">
          <div className="p-6 border-b border-gray-700">
            <h2 className="text-xl font-bold">管理后台</h2>
            <p className="text-xs text-gray-400 mt-1">AI情绪治愈自习打卡空间</p>
          </div>
          <nav className="flex-1 px-4 py-4 space-y-2">
            {tabs.map((t) => (
              <button key={t.key} onClick={() => setTab(t.key)}
                className={"w-full text-left px-4 py-3 rounded-lg transition-colors flex items-center gap-3 " +
                  (tab === t.key ? "bg-white/10 text-white" : "text-gray-300 hover:bg-white/10")}>
                <i className={"fa " + t.icon + " w-5 text-center"}></i>
                {t.label}
              </button>
            ))}
          </nav>
          <div className="p-4 border-t border-gray-700">
            <button onClick={() => { localStorage.removeItem("token"); setCurrentPage("login"); }}
              className="w-full text-left px-4 py-2 text-red-400 hover:bg-white/5 rounded-lg transition-colors flex items-center gap-3">
              <i className="fa fa-sign-out"></i> 退出登录
            </button>
          </div>
        </div>

        {/* Main Content */}
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


