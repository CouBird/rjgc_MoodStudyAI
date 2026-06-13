/**
 * 打卡模块 API
 *
 * 对应文档第 17-19 号接口
 * - GET  /checkins          获取打卡日历
 * - GET  /checkins/{date}   获取某日打卡详情
 * - POST /checkins          补卡
 */

import request from "./request";

export const checkinApi = {
  /** 获取打卡日历 */
  getCalendar: (params) => request.get("/checkins", { params }),

  /** 获取某日打卡详情 */
  getCheckinDetail: (date) => request.get(`/checkins/${date}`),

  /** 补卡 */
  createCheckin: (data) => request.post("/checkins", data),
};
