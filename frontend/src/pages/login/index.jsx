import React, { useState } from "react";
import DisabledAccountModal from "../../components/feedback/DisabledAccountModal";
import TermsModal from "../../components/feedback/TermsModal";
import PrivacyModal from "../../components/feedback/PrivacyModal";
import SensitiveWordModal from "../../components/feedback/SensitiveWordModal";
import { useUser } from "../../store/userContext";
import { authApi } from "../../api/auth";
import { adminApi } from "../../api/admin";

export default function LoginPage({ setCurrentPage, setUserRole }) {
  const { setUser, refreshUser } = useUser();
  const [tab, setTab] = useState("login");
  const [showDisabled, setShowDisabled] = useState(false);
  const [showTerms, setShowTerms] = useState(false);
  const [showPrivacy, setShowPrivacy] = useState(false);
  const [showSensitive, setShowSensitive] = useState(false);
  const [phone, setPhone] = useState("");
  const [password, setPassword] = useState("");
  const [rememberMe, setRememberMe] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [rPhone, setRPhone] = useState("");
  const [rNick, setRNick] = useState("");
  const [rPwd, setRPwd] = useState("");
  const [rConfirm, setRConfirm] = useState("");
  const [agree, setAgree] = useState(false);
  const [adminAcc, setAdminAcc] = useState("");
  const [adminPwd, setAdminPwd] = useState("");
  const [loginError, setLoginError] = useState("");
  const [registerError, setRegisterError] = useState("");
  const [adminError, setAdminError] = useState("");
  const [loginLoading, setLoginLoading] = useState(false);
  const [registerLoading, setRegisterLoading] = useState(false);
  const [adminLoading, setAdminLoading] = useState(false);

  const validatePassword = (pwd) => {
    if (pwd.length < 8) return false;
    const hasLetter = /[a-zA-Z]/.test(pwd);
    const hasDigit = /\d/.test(pwd);
    return hasLetter && hasDigit;
  };

  const handleLogin = (e) => {
    e.preventDefault();
    setLoginError("");
    if (phone === "11111111111") { setShowDisabled(true); return; }
    if (phone.length !== 11 || !validatePassword(password)) { alert("手机号需11位，密码至少8位且包含字母和数字"); return; }
    setLoginLoading(true);
    authApi.login({ phone, password })
      .then((res) => {
        const token = res?.token || res?.adminToken;
        if (token) localStorage.setItem("token", token);
        refreshUser();
        setCurrentPage("home");
      })
      .catch((err) => {
        setLoginError(err?.message || "登录失败，请检查账号和密码");
      })
      .finally(() => setLoginLoading(false));
  };

  const handleRegister = (e) => {
    e.preventDefault();
    setRegisterError("");
    if (!rPhone || !rNick || !rPwd || !rConfirm) { alert("请填写所有注册字段"); return; }
    if (rPwd !== rConfirm) { alert("两次密码不一致"); return; }
    if (!validatePassword(rPwd)) { alert("密码至少8位，包含字母和数字"); return; }
    if (!agree) { alert("请阅读并同意用户协议"); return; }
    const sensitiveWords = ["cnm"]; const lower = rNick.toLowerCase();
    if (sensitiveWords.some((w) => lower.includes(w))) { setShowSensitive(true); return; }
    setRegisterLoading(true);
    authApi.register({ phone: rPhone, nickname: rNick, password: rPwd, confirmPassword: rConfirm, agreeTerms: agree, agreePrivacy: agree })
      .then(() => {
        alert("注册成功，请登录！");
        setRPhone(""); setRNick(""); setRPwd(""); setRConfirm(""); setAgree(false);
        setTab("login");
        refreshUser();
      })
      .catch((err) => {
        setRegisterError(err?.message || "注册失败，请稍后重试");
      })
      .finally(() => setRegisterLoading(false));
  };

  const handleAdminLogin = (e) => {
    e.preventDefault();
    setAdminError("");
    setAdminLoading(true);
    adminApi.login({ account: adminAcc, password: adminPwd })
      .then((res) => {
        const token = res?.token || res?.adminToken;
        if (token) localStorage.setItem("token", token);
        setUserRole("admin");
        setCurrentPage("admin");
        refreshUser();
      })
      .catch((err) => {
        setAdminError(err?.message || "管理员登录失败");
      })
      .finally(() => setAdminLoading(false));
  };

  return (
    <div className="container mx-auto px-4 pb-16">
      <div className="w-full max-w-md mx-auto">
        <div className="bg-white rounded-2xl shadow-xl overflow-hidden">
          <div className="bg-gradient-to-r from-primary to-secondary p-6 text-white text-center">
            <div className="w-16 h-16 rounded-full bg-white/20 mx-auto flex items-center justify-center mb-4">
              <i className={"fa " + (tab === "register" ? "fa-user-plus" : tab === "admin" ? "fa-user-shield" : "fa-book") + " text-3xl"}></i>
            </div>
            <h2 className="text-2xl font-bold mb-2">{tab === "register" ? "创建账号" : tab === "admin" ? "管理后台入口" : "欢迎回来"}</h2>
            <p className="text-white/80">{tab === "register" ? "加入我们，开启高效学习新模式" : tab === "admin" ? "" : "登录开始你的高效学习之旅"}</p>
          </div>

          <div className="p-8">
            {tab === "login" && (
              <form id="login-form" onSubmit={handleLogin}>
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-2">手机号</label>
                  <div className="relative">
                    <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-500"><i className="fa fa-phone"></i></span>
                    <input id="phone" type="tel" placeholder="请输入手机号(测试禁用填11111111111)" maxLength="11" value={phone} onChange={(e) => setPhone(e.target.value)} required
                      className="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none transition-colors" />
                  </div>
                </div>
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-2">密码</label>
                  <div className="relative">
                    <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-500"><i className="fa fa-lock"></i></span>
                    <input id="password" type={showPassword ? "text" : "password"} placeholder="请输入密码（至少8位，包含字母和数字）" value={password} onChange={(e) => setPassword(e.target.value)} required minLength={8}
                      className="w-full pl-10 pr-10 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none transition-colors" />
                    <button type="button" onClick={() => setShowPassword(!showPassword)} className="absolute inset-y-0 right-0 flex items-center pr-3 text-gray-500 hover:text-gray-700">
                      <i className={"fa " + (showPassword ? "fa-eye" : "fa-eye-slash")}></i>
                    </button>
                  </div>
                </div>
                <div className="flex items-center mb-6">
                  <input id="remember-me" type="checkbox" checked={rememberMe} onChange={(e) => setRememberMe(e.target.checked)} className="h-4 w-4 text-primary border-gray-300 rounded" />
                  <label htmlFor="remember-me" className="ml-2 block text-sm text-gray-700">记住我</label>
                </div>
                <button type="submit" disabled={loginLoading} className="w-full bg-gradient-to-r from-primary to-secondary text-white py-3 rounded-lg font-medium hover:opacity-90 transition-opacity shadow-lg shadow-primary/20 disabled:opacity-60">
                  {loginLoading ? "登录中..." : "登录"}
                </button>
                {loginError && <p className="mt-4 text-sm text-red-500 bg-red-50 rounded-lg p-3 text-center">{loginError}</p>}
                <div className="mt-4 text-center flex justify-center space-x-4">
                  <button type="button" onClick={() => setTab("register")} className="text-sm text-primary hover:underline">没有账号？立即注册</button>
                  <button type="button" onClick={() => setTab("admin")} className="text-sm text-gray-500 hover:text-primary transition-colors"><i className="fa fa-user-shield mr-1"></i>管理员</button>
                </div>
              </form>
            )}

            {tab === "register" && (
              <form onSubmit={handleRegister}>
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-2">手机号</label>
                  <div className="relative">
                    <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-500"><i className="fa fa-phone"></i></span>
                    <input id="reg-phone" type="tel" placeholder="请输入11位手机号" value={rPhone} onChange={(e) => setRPhone(e.target.value)} maxLength="11" required
                      className="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
                  </div>
                </div>
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-2">昵称</label>
                  <div className="relative">
                    <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-500"><i className="fa fa-user"></i></span>
                    <input id="reg-nickname" type="text" placeholder="请输入昵称" value={rNick} onChange={(e) => setRNick(e.target.value)} maxLength="20" required
                      className="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
                  </div>
                </div>
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-2">密码</label>
                  <div className="relative">
                    <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-500"><i className="fa fa-lock"></i></span>
                    <input id="reg-password" type="password" placeholder="至少8位，包含字母和数字" value={rPwd} onChange={(e) => setRPwd(e.target.value)} required minLength={8}
                      className="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
                  </div>
                  <p className="mt-1 text-xs text-gray-500">密码长度不少于8位，且必须同时包含字母与数字</p>
                </div>
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-2">确认密码</label>
                  <div className="relative">
                    <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-500"><i className="fa fa-lock"></i></span>
                    <input id="reg-confirm-password" type="password" placeholder="请再次输入密码" value={rConfirm} onChange={(e) => setRConfirm(e.target.value)} required
                      className="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
                  </div>
                </div>
                <div className="mb-6 flex items-center">
                  <input id="agree-terms" type="checkbox" checked={agree} onChange={(e) => setAgree(e.target.checked)} className="h-4 w-4 text-primary border-gray-300 rounded" />
                  <label htmlFor="agree-terms" className="ml-2 text-sm text-gray-700">我已阅读并同意
                    <button type="button" onClick={() => setShowTerms(true)} className="text-primary hover:underline mx-0.5">用户协议</button>和
                    <button type="button" onClick={() => setShowPrivacy(true)} className="text-primary hover:underline mx-0.5">隐私政策</button>
                  </label>
                </div>
                <button type="submit" disabled={registerLoading} className="w-full bg-gradient-to-r from-primary to-secondary text-white py-3 rounded-lg font-medium hover:opacity-90 transition-opacity shadow-lg shadow-primary/20 disabled:opacity-60">
                  {registerLoading ? "注册中..." : "注册"}
                </button>
                {registerError && <p className="mt-4 text-sm text-red-500 bg-red-50 rounded-lg p-3 text-center">{registerError}</p>}
                <div className="mt-4 text-center">
                  <button type="button" onClick={() => setTab("login")} className="text-sm text-primary hover:underline">已有账号？立即登录</button>
                </div>
              </form>
            )}

            {tab === "admin" && (
              <form id="admin-login-form" onSubmit={handleAdminLogin}>
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-2">管理员账号</label>
                  <div className="relative">
                    <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-500"><i className="fa fa-user-shield"></i></span>
                    <input id="admin-account" type="text" placeholder="请输入管理员账号" value={adminAcc} onChange={(e) => setAdminAcc(e.target.value)} required
                      className="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
                  </div>
                  {/* <p className="mt-1 text-xs text-gray-500">测试账号：admin / admin1234</p> */}
                </div>
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-2">密码</label>
                  <div className="relative">
                    <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-500"><i className="fa fa-lock"></i></span>
                    <input id="admin-password" type="password" placeholder="请输入密码" value={adminPwd} onChange={(e) => setAdminPwd(e.target.value)} required
                      className="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
                  </div>
                </div>
                <button type="submit" disabled={adminLoading} className="w-full bg-gradient-to-r from-dark to-gray-700 text-white py-3 rounded-lg font-medium hover:opacity-90 transition-opacity shadow-lg disabled:opacity-60">
                  {adminLoading ? "登录中..." : "管理员登录"}
                </button>
                {adminError && <p className="mt-4 text-sm text-red-500 bg-red-50 rounded-lg p-3 text-center">{adminError}</p>}
                <div className="mt-4 text-center">
                  <button type="button" onClick={() => setTab("login")} className="text-sm text-gray-500 hover:text-primary transition-colors">返回用户登录</button>
                </div>
              </form>
            )}
          </div>
        </div>
      </div>
      <DisabledAccountModal isOpen={showDisabled} onClose={() => setShowDisabled(false)} />
      <TermsModal isOpen={showTerms} onClose={() => setShowTerms(false)} />
      <PrivacyModal isOpen={showPrivacy} onClose={() => setShowPrivacy(false)} />
      <SensitiveWordModal isOpen={showSensitive} onClose={() => setShowSensitive(false)} />
    </div>
  );
}
