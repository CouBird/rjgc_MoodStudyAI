/**
 * 认证模块 API
 *
 * 对应文档第 1-2 号接口
 * - POST /auth/register    用户注册
 * - POST /auth/login       用户登录
 */

import request from "./request";

export const authApi = {
  /** 用户注册 */
  register: (data) => request.post("/auth/register", data),

  /** 用户登录 */
  login: (data) => request.post("/auth/login", data),
};
