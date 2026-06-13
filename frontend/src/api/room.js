/**
 * 自习室模块 API
 *
 * 后端实际实现（backend handler + docs 对照）：
 * - GET   /rooms                   获取自习室列表（已实现）
 * - POST  /rooms                   创建自习室（已实现）
 * - GET   /rooms/{roomId}          获取自习室详情（已实现）
 * - GET   /rooms/{roomId}/seats    获取自习室座位列表（已实现）
 *
 * 加入/离开房间功能后端未提供，相关功能已从前端禁用
 */

import request from "./request";

export const roomApi = {
  getRoomList: (params) => request.get("/rooms", { params }),
  getRoomDetail: (roomId) => request.get(`/rooms/${roomId}`),
  createRoom: (data) => request.post("/rooms", data),
};
