import React, { createContext, useContext, useState, useEffect, useCallback } from "react";
import { userApi } from "../api/user";

const UserContext = createContext(null);

function decodeJwtRole(token) {
  if (!token) return null;
  try {
    const payload = token.split(".")[1];
    if (!payload) return null;
    const base64 = payload.replace(/-/g, "+").replace(/_/g, "/");
    const padded = base64 + "=".repeat((4 - (base64.length % 4)) % 4);
    const decoded = atob(padded);
    const claims = JSON.parse(decoded);
    return claims?.role || null;
  } catch {
    return null;
  }
}

function readStoredRole() {
  if (typeof window === "undefined") return null;
  const token = localStorage.getItem("token");
  return decodeJwtRole(token) || localStorage.getItem("role");
}

export function UserProvider({ children, onUserReady, setUserRole }) {
  const [userInfo, setUserInfo] = useState(null);
  const [loading, setLoading] = useState(() => {
    if (typeof window === "undefined") return false;
    const token = localStorage.getItem("token");
    return !!token && readStoredRole() !== "admin";
  });
  const [error, setError] = useState(null);

  useEffect(() => {
    const token = localStorage.getItem("token");
    const role = readStoredRole();
    if (!token) {
      setLoading(false);
      return;
    }

    if (role === "admin") {
      const adminUser = { role: "admin" };
      setUserInfo(adminUser);
      if (setUserRole) setUserRole("admin");
      if (onUserReady) onUserReady(adminUser);
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
          localStorage.setItem("role", data.role);
        }
        if (onUserReady) {
          onUserReady(data);
        }
      })
      .catch((err) => {
        console.error("[UserContext] 获取用户信息失败:", err?.message || err);
        setError(err?.message || "获取用户信息失败");
        if (err?.response?.status === 401) {
          localStorage.removeItem("token");
          localStorage.removeItem("role");
          setUserInfo(null);
        }
      })
      .finally(() => setLoading(false));
  }, []);

  const setUser = useCallback((userData) => {
    setUserInfo(userData);
    if (setUserRole && userData.role) {
      setUserRole(userData.role);
      localStorage.setItem("role", userData.role);
    }
  }, [setUserRole]);

  const clearUser = useCallback(() => {
    setUserInfo(null);
    setLoading(false);
    setError(null);
    localStorage.removeItem("role");
  }, []);

  const refreshUser = useCallback(async () => {
    const token = localStorage.getItem("token");
    const role = readStoredRole();
    if (!token) {
      clearUser();
      return null;
    }

    if (role === "admin") {
      const adminUser = { role: "admin" };
      setUserInfo(adminUser);
      if (setUserRole) setUserRole("admin");
      return adminUser;
    }

    setLoading(true);
    try {
      const data = await userApi.getProfile();
      setUserInfo(data);
      if (setUserRole && data.role) {
        setUserRole(data.role);
        localStorage.setItem("role", data.role);
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