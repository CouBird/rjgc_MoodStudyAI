/**
 * UserContext — 全局用户状态
 *
 * 唯一数据源：App.jsx 挂载时执行 GET /users/me
 * 禁止页面各自重复调用
 *
 * 流程：
 * 1. 检测 localStorage token
 * 2. 存在 → 调用一次 userApi.getProfile()
 * 3. 存储 userInfo / loading / error
 * 4. 注册/登录成功 → 通过 setUser() 更新
 * 5. 退出 → 通过 clearUser() 清除
 *
 * 依赖：userApi
 */

import React, { createContext, useContext, useState, useEffect, useCallback } from "react";
import { userApi } from "../api/user";

const UserContext = createContext(null);

export function UserProvider({ children, onUserReady, setUserRole }) {
  const [userInfo, setUserInfo] = useState(null);
  const [loading, setLoading] = useState(!!localStorage.getItem("token"));
  const [error, setError] = useState(null);

  // 挂载时：如果有 token，请求一次用户信息
  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) {
      setLoading(false);
      return;
    }

    setLoading(true);
    setError(null);

    userApi.getProfile()
      .then((data) => {
        setUserInfo(data);
        if (setUserRole && data.role) {
          setUserRole(data.role);
        }
        if (onUserReady) {
          onUserReady(data);
        }
      })
      .catch((err) => {
        console.error("[UserContext] 获取用户信息失败:", err?.message || err);
        setError(err?.message || "获取用户信息失败");
        // token 无效时清除
        if (err?.response?.status === 401) {
          localStorage.removeItem("token");
        }
      })
      .finally(() => setLoading(false));
  }, []); // 仅挂载执行一次

  const setUser = useCallback((userData) => {
    setUserInfo(userData);
    if (setUserRole && userData.role) {
      setUserRole(userData.role);
    }
  }, [setUserRole]);

  const clearUser = useCallback(() => {
    setUserInfo(null);
    setLoading(false);
    setError(null);
  }, []);

  const refreshUser = useCallback(async () => {
    const token = localStorage.getItem("token");
    if (!token) {
      clearUser();
      return null;
    }
    setLoading(true);
    try {
      const data = await userApi.getProfile();
      setUserInfo(data);
      if (setUserRole && data.role) {
        setUserRole(data.role);
      }
      return data;
    } catch (err) {
      console.error("[UserContext] 刷新用户信息失败:", err?.message || err);
      clearUser();
      return null;
    } finally {
      setLoading(false);
    }
  }, [clearUser, setUserRole]);

  const value = {
    userInfo,
    loading,
    error,
    setUser,
    clearUser,
    refreshUser,
    isAuthenticated: !!userInfo,
  };

  return (
    <UserContext.Provider value={value}>
      {children}
    </UserContext.Provider>
  );
}

export function useUser() {
  const ctx = useContext(UserContext);
  if (!ctx) {
    throw new Error("useUser must be used within a UserProvider");
  }
  return ctx;
}

export default UserContext;
