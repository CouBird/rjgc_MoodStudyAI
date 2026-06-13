/**
 * SessionVM — 统一学习会话视图模型
 */


export function toSessionVM(raw) {
  if (!raw) return null;

  return Object.freeze({
    sessionId: raw.sessionId ?? null,
    roomId: raw.roomId ?? null,
    roomName: raw.roomName ?? null,
    seatId: raw.seatId ?? null,
    seatCode: raw.seatCode ?? null,
    status: raw.status ?? null,
    mode: raw.mode ?? "normal",
    studyContent: raw.studyContent ?? null,
    startTime: raw.startTime ?? null,
    endTime: raw.endTime ?? null,
    durationMinutes: raw.durationMinutes ?? 0,
    isValid: raw.isValid ?? true,
    lastHeartbeatAt: raw.lastHeartbeatAt ?? null,
    // 前端计算
    durationSeconds: (raw.durationMinutes ?? 0) * 60,
  });
}

/**
 * 将 TodayStatsResponse 转为 TodayStatsVM
 */
export function toTodayStatsVM(raw) {
  if (!raw) return null;

  return Object.freeze({
    todayMinutes: raw.todayMinutes ?? 0,
    todayHours: (raw.todayHours ?? 0).toString(),
    streakDays: raw.streakDays ?? 0,
    todayCheckin: raw.todayCheckin ?? false,
    latestEmotion: raw.latestEmotion ?? null,
    mood: raw.mood ?? null,
  });
}

/**
 * 将 UserStatsResponse 转为 UserStatsVM
 */
export function toUserStatsVM(raw) {
  if (!raw) return null;

  return Object.freeze({
    period: raw.period ?? "week",
    totalHours: raw.totalHours ?? 0,
    averageDailyHours: raw.averageDailyHours ?? 0,
    streakDays: raw.streakDays ?? 0,
    validSessionCount: raw.validSessionCount ?? 0,
    checkinCount: raw.checkinCount ?? 0,
    studyDays: raw.studyDays ?? 0,
    averageDailyMinutes: raw.averageDailyMinutes ?? 0,
    trends: Array.isArray(raw.trends) ? raw.trends : [],
    mainEmotion: raw.mainEmotion ?? null,
    summary: raw.summary ?? null,
  });
}

export default toSessionVM;
