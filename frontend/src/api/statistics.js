/**
 * 统计模块 API
 *
 * 对应文档第 7、22 号接口
 * - GET /users/me/stats/today 获取今日学习概览
 * - GET /users/me/stats       获取学习统计
 */

import request from "./request";

export const statisticsApi = {
  /** 获取今日学习概览 */
  getTodayStats: () => request.get("/users/me/stats/today"),

  /** 获取学习统计（支持 period / startDate / endDate 等查询参数） */
  getStudyStats: (params) => request.get("/users/me/stats", { params }),
};
