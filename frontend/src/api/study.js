/**
 * 学习会话模块 API
 *
 * 对应文档第 11-16 号接口
 * - POST   /study-sessions                   开始学习
 * - GET    /study-sessions/active            获取当前进行中的学习会话
 * - PATCH  /study-sessions/{sessionId}       更新学习会话
 * - POST   /study-sessions/{sessionId}/breaks 创建休息记录
 * - PATCH  /study-breaks/{breakId}           延长休息
 * - POST   /study-sessions/{sessionId}/heartbeats 学习心跳
 */

import request from "./request";

export const studyApi = {
  /** 开始学习（创建学习会话） */
  startSession: (data) => request.post("/study-sessions", data),

  /** 获取当前进行中的学习会话 */
  getActiveSession: () => request.get("/study-sessions/active"),

  /** 更新学习会话（例如结束学习） */
  updateSession: (sessionId, data) =>
    request.patch(`/study-sessions/${sessionId}`, data),

  /** 创建休息记录 */
  createBreak: (sessionId, data) =>
    request.post(`/study-sessions/${sessionId}/breaks`, data),

  /** 延长休息 */
  extendBreak: (breakId, data) =>
    request.patch(`/study-breaks/${breakId}`, data),

  /** 学习心跳 */
  sendHeartbeat: (sessionId) =>
    request.post(`/study-sessions/${sessionId}/heartbeats`),
};
