import React from "react";
import { useUserVM } from "../../viewmodels";
import { useUser } from "../../store/userContext";

export default function ProfilePage({ setCurrentPage }) {
  const userVM = useUserVM();
  const { clearUser } = useUser();

  const formatPhone = (phone) => {
    if (!phone) return null;
    if (phone.length === 11) {
      return phone.slice(0, 3) + "****" + phone.slice(7);
    }
    return phone;
  };

  const formatDate = (isoStr) => {
    if (!isoStr) return null;
    try {
      const d = new Date(isoStr);
      return d.toLocaleDateString("zh-CN");
    } catch {
      return null;
    }
  };

  const handleLogout = () => {
    localStorage.removeItem("token");
    clearUser();
    setCurrentPage("login");
  };

  const avatarSrc = userVM.avatarUrl || "https://picsum.photos/id/64/200/200";

  return (
    <div className="container mx-auto px-4 pb-16 min-h-screen">
      <div className="max-w-4xl mx-auto">
        <div className="mb-8">
          <h2 className="text-2xl font-bold text-gray-800 mb-2">个人中心</h2>
          <p className="text-gray-500">管理你的个人信息和学习记录</p>
        </div>

        <div className="bg-white rounded-xl shadow-md p-6 mb-6">
          <div className="flex flex-col md:flex-row items-center md:items-start gap-6">
            <div className="relative">
              <img
                src={avatarSrc}
                alt="用户头像"
                className="w-24 h-24 rounded-full object-cover border-2 border-gray-200"
              />
            </div>
            <div className="flex-1 text-center md:text-left">
              <h3 className="text-xl font-bold text-gray-800 mb-2">{userVM.displayName || "用户"}</h3>
              <div className="flex flex-wrap justify-center md:justify-start gap-2">
                {userVM.phone && (
                  <span className="px-3 py-1 bg-primary/10 text-primary text-xs rounded-full">
                    <i className="fa fa-phone mr-1"></i>{formatPhone(userVM.phone)}
                  </span>
                )}
                {userVM.createdAt && (
                  <span className="px-3 py-1 bg-gray-100 text-gray-500 text-xs rounded-full">
                    <i className="fa fa-calendar mr-1"></i>注册时间：{formatDate(userVM.createdAt)}
                  </span>
                )}
              </div>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-xl shadow-md p-6 mb-6">
          <h3 className="text-lg font-semibold text-gray-800 mb-4">学习统计</h3>
          <p className="text-sm text-gray-400 mb-4">详细统计数据请在「学习统计」页面查看</p>
          <button
            onClick={() => setCurrentPage("statistics")}
            className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors text-sm"
          >
            <i className="fa fa-bar-chart mr-1"></i>查看学习统计
          </button>
        </div>

        <div className="bg-white rounded-xl shadow-md p-6 mb-6">
          <h3 className="text-lg font-semibold text-gray-800 mb-4">账号设置</h3>
          <div className="space-y-3">
            <button disabled className="w-full text-left px-4 py-3 rounded-lg bg-gray-100 text-gray-400 cursor-not-allowed flex items-center justify-between">
              <span><i className="fa fa-user mr-2"></i>编辑个人资料</span><span className="text-xs">即将上线</span>
            </button>
            <button disabled className="w-full text-left px-4 py-3 rounded-lg bg-gray-100 text-gray-400 cursor-not-allowed flex items-center justify-between">
              <span><i className="fa fa-key mr-2"></i>修改密码</span><span className="text-xs">即将上线</span>
            </button>
          </div>
        </div>

        <div className="text-center pb-8">
          <button onClick={handleLogout} className="text-red-500 hover:text-red-600 text-sm">
            <i className="fa fa-sign-out mr-1"></i>退出登录
          </button>
        </div>
      </div>
    </div>
  );
}
