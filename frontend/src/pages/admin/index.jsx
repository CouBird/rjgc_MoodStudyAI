import React, { useState } from "react";

export default function AdminPage({ setCurrentPage }) {
  const [tab, setTab] = useState("dashboard");

  const tabs = [
    { key: "dashboard", label: "仪表盘", icon: "fa-dashboard" },
    { key: "users", label: "用户管理", icon: "fa-users" },
    { key: "rooms", label: "房间管理", icon: "fa-building" },
    { key: "logs", label: "审计日志", icon: "fa-history" },
  ];

  const renderFallback = (title) => (
    <div className="text-center py-20">
      <div className="text-gray-300 text-5xl mb-4"><i className="fa fa-tools"></i></div>
      <h3 className="text-lg font-semibold text-gray-700 mb-2">{title}</h3>
      <p className="text-gray-400 text-sm">该模块功能正在开发中，敬请期待</p>
      <p className="text-gray-400 text-xs mt-2">[TODO] 后端 API 尚未实现</p>
    </div>
  );

  return (
    <div className="min-h-screen bg-gradient-to-br from-calm via-light to-purple-50">
      <div className="flex min-h-screen">
        <div className="w-64 bg-white border-r border-gray-200 pt-8">
          <div className="px-6 py-4 border-b border-gray-100">
            <h2 className="font-bold text-gray-800">管理后台</h2>
          </div>
          <nav className="p-4 space-y-1">
            {tabs.map((t) => (
              <button key={t.key} onClick={() => setTab(t.key)}
                className={"w-full flex items-center gap-3 px-4 py-2.5 rounded-xl text-sm font-medium transition-all " +
                  (tab === t.key ? "bg-primary/10 text-primary" : "text-gray-600 hover:bg-gray-50")}>
                <i className={"fa " + t.icon + " w-5 text-center"}></i>{t.label}
              </button>
            ))}
          </nav>
          <div className="px-6 pt-4 border-t border-gray-100">
            <button onClick={() => { localStorage.removeItem("token"); setCurrentPage("login"); }}
              className="flex items-center gap-3 text-sm text-gray-500 hover:text-red-600 transition-colors">
              <i className="fa fa-sign-out"></i> 退出管理
            </button>
          </div>
        </div>

        <div className="flex-1 p-8 pt-24 overflow-auto">
          {tab === "dashboard" && <div><h2 className="text-xl font-bold text-gray-800 mb-6">仪表盘</h2>{renderFallback("仪表盘数据加载")}</div>}
          {tab === "users" && <div><h2 className="text-xl font-bold text-gray-800 mb-6">用户管理</h2>{renderFallback("用户列表管理")}</div>}
          {tab === "rooms" && <div><h2 className="text-xl font-bold text-gray-800 mb-6">房间管理</h2>{renderFallback("自习室管理")}</div>}
          {tab === "logs" && <div><h2 className="text-xl font-bold text-gray-800 mb-6">审计日志</h2>{renderFallback("操作审计日志")}</div>}
        </div>
      </div>
    </div>
  );
}
