/**
 * 用户模块 API
 */

import request from "./request";

export const userApi = {
  /** 获取当前用户信息 */
  getProfile: () => request.get("/users/me"),

  /** 修改用户资料 */
  updateProfile: (data) => request.patch("/users/me", data),

  /** 上传头像（multipart/form-data） */
  uploadAvatar: (formData) => request.post("/users/me/avatar", formData),

  /** 修改密码 */
  changePassword: (data) => request.patch("/users/me/password", data),
};
