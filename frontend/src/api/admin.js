/**
 * 管理后台模块 API
 *
 * 后端实际实现（backend handler + docs 对照）：
 * - POST /admin/auth/login    管理员登录（已实现）
 *
 * 后端缺失（docs #24-#29 已定义但尚未实现）：
 * - GET   /admin/users                   ❌ 缺失
 * - PATCH /admin/users/{userId}/status   ❌ 缺失
 * - GET   /admin/rooms                   ❌ 缺失
 * - PATCH /admin/rooms/{roomId}/status   ❌ 缺失
 * - GET   /admin/dashboard               ❌ 缺失
 * - GET   /admin/audit-logs              ❌ 缺失
 * 以上功能前端已禁用，等待后端实现
 */

import request from "./request";

export const adminApi = {
  /** 管理员登录 */
  login: (data) => request.post("/admin/auth/login", data),
};
