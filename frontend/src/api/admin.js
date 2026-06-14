/**
 * 管理后台模块 API
 *
 * 后端统一挂载在 /api/v1 下，这里只写业务路径。
 */

import request from "./request";

export const adminApi = {
  /** 管理员登录 */
  login: (data) => request.post("/admin/auth/login", data),

  /** 后台概览 */
  getDashboard: () => request.get("/admin/dashboard"),

  /** 用户列表 */
  getUsers: (params) => request.get("/admin/users", { params }),

  /** 修改用户状态 */
  updateUserStatus: (userId, data) => request.patch(`/admin/users/${userId}/status`, data),

  /** 自习室列表 */
  getRooms: (params) => request.get("/admin/rooms", { params }),

  /** 修改自习室状态 */
  updateRoomStatus: (roomId, data) => request.patch(`/admin/rooms/${roomId}/status`, data),

  /** 管理员操作日志 */
  getAuditLogs: (params) => request.get("/admin/audit-logs", { params }),
};