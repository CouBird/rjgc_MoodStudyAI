/**
 * 情绪模块 API — 纯传输层
 *
 * 职责仅限：发送请求、接收原始响应、返回原始 DTO。
 * 所有字段归一化、分数映射、响应转换移至 viewmodels/emotionVM.js。
 */

import request from "./request";

export const emotionApi = {
  /**
   * 提交情绪打卡（关联学习会话）
   * body: 已由 vm.toEmotionPayload 预处理的 payload
   * 返回: 原始 DTO { emotionRecord, aiFeedback }
   */
  submitEmotion: (sessionId, payload) =>
    request.post(`/study-sessions/${sessionId}/emotion-records`, payload),

  /**
   * 获取情绪趋势
   * params: { period?: "week"|"month"|"year", date?: "YYYY-MM-DD" }
   * 返回: 原始 DTO { period, emotionMap, items, mainEmotion, summary, trends, tagDistribution }
   */
  getEmotionTrend: (params) =>
    request.get("/users/me/emotion-trends", { params }),

  /** 获取情绪记录列表（复用趋势接口） */
  getEmotionList: (params) =>
    request.get("/users/me/emotion-trends", { params }),
};

