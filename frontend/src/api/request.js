/**
 * 统一请求封装
 *
 * - 基于 axios 封装
 * - baseURL 统一管理（优先读取 VITE_API_BASE_URL，回退 /api/v1）
 * - 请求拦截器：自动注入 Authorization token
 * - 响应拦截器：统一解包 data 层，集中处理 HTTP 错误码
 * - 401 处理：清除 token 并跳转登录（后端未实现 /auth/refresh）
 */

import axios from "axios";

const request = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || "/api/v1",
  timeout: 15000,
  headers: { "Content-Type": "application/json" },
});

// -------- 请求拦截器 --------
request.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem("token");
    if (token) config.headers.Authorization = `Bearer ${token}`;
    return config;
  },
  (error) => Promise.reject(error)
);

// -------- 响应拦截器 --------
request.interceptors.response.use(
  // 成功分支：解包后端统一响应结构 { code, message, data }
  (response) => {
    const res = response.data;

    // 业务成功（code === 0），直接返回 data 层
    if (res.code === 0) {
      return res.data;
    }

    // 业务错误（code !== 0），打印并拒绝
    console.error(`[API Error ${res.code}] ${res.message}`);
    const err = new Error(res.message || "请求失败");
    err.code = res.code;
    return Promise.reject(err);
  },
  // 错误分支：处理 HTTP 状态码
  (error) => {
    if (error.response) {
      const { status, data } = error.response;
      const message = data?.message || "";

      // 401：Token 无效或过期，直接 logout（后端未实现 /auth/refresh）
      if (status === 401) {
        localStorage.removeItem("token");
        localStorage.removeItem("refreshToken");
        console.error("未登录或 Token 无效");
        window.location.href = "/login";
        return Promise.reject(error);
      }

      switch (status) {
        case 403:
          console.error("权限不足");
          break;
        case 404:
          console.error("资源不存在");
          break;
        case 409:
          console.error("资源冲突：" + message);
          break;
        case 413:
          console.error("文件过大");
          break;
        case 422:
          console.error("业务校验失败：" + message);
          break;
        case 423:
          console.error("账号已锁定或禁用");
          break;
        case 500:
          console.error("服务器错误");
          break;
        default:
          console.error(`HTTP ${status}: ${message || "未知错误"}`);
      }
    }

    return Promise.reject(error);
  }
);

export default request;
