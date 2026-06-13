import React, { useState, useRef } from "react";
import { useUserVM } from "../../viewmodels";
import { useUser } from "../../store/userContext";
import { userApi } from "../../api/user";

export default function ProfilePage({ setCurrentPage }) {
  const userVM = useUserVM();
  const { clearUser, refreshUser } = useUser();
  const fileInputRef = useRef(null);

  // Modal states
  const [showEditProfile, setShowEditProfile] = useState(false);
  const [showChangePassword, setShowChangePassword] = useState(false);

  // Edit profile form
  const [editNickname, setEditNickname] = useState("");
  const [editProfile, setEditProfile] = useState("");
  const [editLoading, setEditLoading] = useState(false);
  const [editError, setEditError] = useState("");

  // Change password form
  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [pwdLoading, setPwdLoading] = useState(false);
  const [pwdError, setPwdError] = useState("");

  // Avatar upload
  const [avatarUploading, setAvatarUploading] = useState(false);

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

  // --- Edit Profile ---
  const openEditProfile = () => {
    setEditNickname(userVM.nickname || "");
    setEditProfile(userVM.profile || "");
    setEditError("");
    setShowEditProfile(true);
  };

  const handleEditProfile = async (e) => {
    e.preventDefault();
    setEditError("");
    if (!editNickname.trim()) {
      setEditError("昵称不能为空");
      return;
    }
    setEditLoading(true);
    try {
      await userApi.updateProfile({
        nickname: editNickname.trim(),
        profile: editProfile.trim() || undefined,
      });
      await refreshUser();
      setShowEditProfile(false);
    } catch (err) {
      setEditError(err?.message || "修改失败");
    } finally {
      setEditLoading(false);
    }
  };

  // --- Change Password ---
  const openChangePassword = () => {
    setCurrentPassword("");
    setNewPassword("");
    setConfirmPassword("");
    setPwdError("");
    setShowChangePassword(true);
  };

  const validatePassword = (pwd) => {
    if (pwd.length < 8) return false;
    const hasLetter = /[a-zA-Z]/.test(pwd);
    const hasDigit = /\d/.test(pwd);
    return hasLetter && hasDigit;
  };

  const handleChangePassword = async (e) => {
    e.preventDefault();
    setPwdError("");
    if (newPassword !== confirmPassword) {
      setPwdError("两次密码不一致");
      return;
    }
    if (!validatePassword(newPassword)) {
      setPwdError("密码至少8位，包含字母和数字");
      return;
    }
    setPwdLoading(true);
    try {
      await userApi.changePassword({
        currentPassword,
        newPassword,
        confirmPassword,
      });
      alert("密码修改成功");
      setShowChangePassword(false);
    } catch (err) {
      setPwdError(err?.message || "修改密码失败");
    } finally {
      setPwdLoading(false);
    }
  };

  // --- Avatar Upload ---
  const handleAvatarClick = () => {
    fileInputRef.current?.click();
  };

  const handleAvatarChange = async (e) => {
    const file = e.target.files?.[0];
    if (!file) return;
    if (file.size > 5 * 1024 * 1024) {
      alert("头像文件不能超过5MB");
      return;
    }
    const formData = new FormData();
    formData.append("file", file);
    setAvatarUploading(true);
    try {
      await userApi.uploadAvatar(formData);
      await refreshUser();
    } catch (err) {
      alert(err?.message || "头像上传失败");
    } finally {
      setAvatarUploading(false);
    }
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
            <div className="relative cursor-pointer group" onClick={handleAvatarClick}>
              <img
                src={avatarSrc}
                alt="用户头像"
                className="w-24 h-24 rounded-full object-cover border-2 border-gray-200"
              />
              <div className="absolute inset-0 rounded-full bg-black/40 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                <span className="text-white text-xs">{avatarUploading ? "上传中..." : "更换"}</span>
              </div>
              <input
                ref={fileInputRef}
                type="file"
                accept="image/*"
                className="hidden"
                onChange={handleAvatarChange}
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
            <button onClick={openEditProfile} className="w-full text-left px-4 py-3 rounded-lg bg-gray-50 text-gray-700 hover:bg-gray-100 transition-colors flex items-center justify-between">
              <span><i className="fa fa-user mr-2"></i>编辑个人资料</span><i className="fa fa-chevron-right text-gray-400 text-xs"></i>
            </button>
            <button onClick={openChangePassword} className="w-full text-left px-4 py-3 rounded-lg bg-gray-50 text-gray-700 hover:bg-gray-100 transition-colors flex items-center justify-between">
              <span><i className="fa fa-key mr-2"></i>修改密码</span><i className="fa fa-chevron-right text-gray-400 text-xs"></i>
            </button>
          </div>
        </div>

        <div className="text-center pb-8">
          <button onClick={handleLogout} className="text-red-500 hover:text-red-600 text-sm">
            <i className="fa fa-sign-out mr-1"></i>退出登录
          </button>
        </div>
      </div>

      {/* Edit Profile Modal */}
      {showEditProfile && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white rounded-2xl shadow-2xl w-full max-w-md mx-4">
            <div className="bg-gradient-to-r from-primary to-secondary p-6 text-white text-center rounded-t-2xl">
              <h3 className="text-xl font-bold mb-2">编辑个人资料</h3>
            </div>
            <form onSubmit={handleEditProfile} className="p-6 space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">昵称</label>
                <input type="text" value={editNickname} onChange={(e) => setEditNickname(e.target.value)} maxLength={20} required
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">个人简介（选填）</label>
                <textarea rows={3} maxLength={255} value={editProfile} onChange={(e) => setEditProfile(e.target.value)} placeholder="介绍一下自己..."
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none resize-none" />
              </div>
              {editError && <p className="text-sm text-red-500 bg-red-50 rounded-lg p-3">{editError}</p>}
              <div className="flex gap-3 pt-2">
                <button type="button" onClick={() => setShowEditProfile(false)} className="flex-1 bg-gray-100 text-gray-600 py-2.5 rounded-lg font-medium hover:bg-gray-200 transition-colors">取消</button>
                <button type="submit" disabled={editLoading} className="flex-1 bg-primary text-white py-2.5 rounded-lg font-medium hover:bg-primary/90 transition-colors disabled:opacity-60">
                  {editLoading ? "保存中..." : "保存"}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Change Password Modal */}
      {showChangePassword && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white rounded-2xl shadow-2xl w-full max-w-md mx-4">
            <div className="bg-gradient-to-r from-primary to-secondary p-6 text-white text-center rounded-t-2xl">
              <h3 className="text-xl font-bold mb-2">修改密码</h3>
            </div>
            <form onSubmit={handleChangePassword} className="p-6 space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">当前密码</label>
                <input type="password" value={currentPassword} onChange={(e) => setCurrentPassword(e.target.value)} required
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">新密码</label>
                <input type="password" value={newPassword} onChange={(e) => setNewPassword(e.target.value)} required
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">确认新密码</label>
                <input type="password" value={confirmPassword} onChange={(e) => setConfirmPassword(e.target.value)} required
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
              </div>
              {pwdError && <p className="text-sm text-red-500 bg-red-50 rounded-lg p-3">{pwdError}</p>}
              <div className="flex gap-3 pt-2">
                <button type="button" onClick={() => setShowChangePassword(false)} className="flex-1 bg-gray-100 text-gray-600 py-2.5 rounded-lg font-medium hover:bg-gray-200 transition-colors">取消</button>
                <button type="submit" disabled={pwdLoading} className="flex-1 bg-primary text-white py-2.5 rounded-lg font-medium hover:bg-primary/90 transition-colors disabled:opacity-60">
                  {pwdLoading ? "修改中..." : "确认修改"}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
