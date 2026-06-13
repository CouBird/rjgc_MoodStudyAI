/**
 * 用户模块 API
 *
 * 后端实际实现（backend handler + docs 对照）：
 * - GET /users/me    获取当前用户信息（已实现）
 *
 * 后端缺失（docs #4-#6 已定义但尚未实现）：
 * - PATCH  /users/me              修改用户资料  ❌ 缺失
 * - POST   /users/me/avatar       上传头像       ❌ 缺失
 * - PATCH  /users/me/password     修改密码       ❌ 缺失
 * 以上功能前端已禁用，等待后端实现
 */

import request from "./request";

export const userApi = {
  /** 获取当前用户信息 */
  getProfile: () => request.get("/users/me"),
};
