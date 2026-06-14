/**
 * 学习会话状态上下文
 *
 * 统一管理：
 * - 会话生命周期（开始/暂停/恢复/结束）
 * - 与后端 API 同步
 * - 页面刷新恢复
 * - 登录状态联动
 *
 */

import React, { createContext, useContext, useState, useEffect, useCallback, useRef } from "react";
import { studyApi } from "../api/study";
import { emotionApi } from "../api/emotion";
import { StudySessionStatus } from '../constants/studySessionStatus';
import { toEmotionPayload, toAiFeedbackVM } from '../viewmodels/emotionVM';
import { formatTime } from '../utils';

const StudyContext = createContext(null);

const SESSION_STORAGE_KEY = "active_study_session";

function saveToStorage(data) {
  if (data) {
    localStorage.setItem(SESSION_STORAGE_KEY, JSON.stringify(data));
  } else {
    localStorage.removeItem(SESSION_STORAGE_KEY);
  }
}

function loadFromStorage() {
  try {
    const saved = localStorage.getItem(SESSION_STORAGE_KEY);
    return saved ? JSON.parse(saved) : null;
  } catch {
    return null;
  }
}

export function StudyProvider({ children }) {
  const [sessionId, setSessionId] = useState(null);
  const [sessionStatus, setSessionStatus] = useState(StudySessionStatus.IDLE);
  // idle | active | paused | completed

  const [activeSession, setActiveSession] = useState(null);

  const [startedAt, setStartedAt] = useState(null);

  const [totalDuration, setTotalDuration] = useState(0);

  const [timerMode, setTimerMode] = useState("normal");
  const [isChecking, setIsChecking] = useState(true);

  const intervalRef = useRef(null);
  const heartbeatRef = useRef(null);
  const lastSessionIdRef = useRef(null);


  useEffect(() => {
    if (sessionStatus === StudySessionStatus.STUDYING && startedAt) {
      const startMs = new Date(startedAt).getTime();
      intervalRef.current = setInterval(() => {
        setTotalDuration(Math.floor((Date.now() - startMs) / 1000));
      }, 1000);
    } else {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    }
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    };
  }, [sessionStatus, startedAt]);

  // ---------- Heartbeat (每30s) ----------

  useEffect(() => {
    if (sessionStatus === StudySessionStatus.STUDYING && sessionId) {
      heartbeatRef.current = setInterval(() => {
        studyApi.sendHeartbeat(sessionId).catch(() => {});
      }, 30000);
    } else {
      if (heartbeatRef.current) {
        clearInterval(heartbeatRef.current);
        heartbeatRef.current = null;
      }
    }
    return () => {
      if (heartbeatRef.current) {
        clearInterval(heartbeatRef.current);
        heartbeatRef.current = null;
      }
    };
  }, [sessionStatus, sessionId]);

  // 

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) {
      clearSession();
      setIsChecking(false);
      return;
    }

    syncActiveSession().finally(() => {});
  }, []);

  const restoreSession = useCallback((data) => {
    const id = data.sessionId ?? data.id;
    const status = data.status === StudySessionStatus.PAUSED ? StudySessionStatus.PAUSED : StudySessionStatus.STUDYING;
    const roomId = data.roomId ?? data.room_id ?? null;
    const roomName = data.roomName ?? data.room_name ?? null;
    const seatId = data.seatId ?? data.seat_id ?? null;
    const seatCode = data.seatCode ?? data.seat_code ?? null;
    const startTime = data.startTime ?? data.start_time ?? null;
    const durationMinutes = data.durationMinutes ?? data.duration_minutes ?? 0;
    const mode = data.mode ?? "normal";
    const studyContent = data.studyContent ?? data.study_content ?? null;

    setSessionId(id);
    setSessionStatus(status);
    setStartedAt(startTime);
    setTimerMode(mode);
    setTotalDuration(durationMinutes ? durationMinutes * 60 : 0);
    setActiveSession({
      sessionId: id,
      roomId,
      roomName,
      seatId,
      seatCode,
      status,
      mode,
      startTime,
      durationMinutes,
      studyContent,
    });
    saveToStorage({
      sessionId: id,
      status: status,
      roomId,
      roomName,
      seatId,
      seatCode,
      startTime,
      durationMinutes,
      mode,
    });
  }, []);

  const clearSession = useCallback(() => {
    setSessionId(null);
    setSessionStatus(StudySessionStatus.IDLE);
    setStartedAt(null);
    setTotalDuration(0);
    setTimerMode("normal");
    setActiveSession(null);
    saveToStorage(null);
  }, []);

  const syncActiveSession = useCallback(async () => {
    const token = localStorage.getItem("token");
    if (!token) {
      clearSession();
      setIsChecking(false);
      return null;
    }

    setIsChecking(true);
    const saved = loadFromStorage();

    try {
      const data = await studyApi.getActiveSession();
      if (data && data.sessionId) {
        restoreSession(data);
        return data;
      }

      clearSession();
      return null;
    } catch (error) {
      if (saved && saved.sessionId) {
        restoreSession(saved);
        return saved;
      }

      clearSession();
      return null;
    } finally {
      setIsChecking(false);
    }
  }, [clearSession, restoreSession]);

  // 

  const startStudy = useCallback(async ({ roomId, seatId, mode, studyContent } = {}) => {
    const token = localStorage.getItem("token");
    if (!token) {
      throw new Error("请先登录");
    }

    if (!roomId || !seatId) {
      throw new Error("请先选择自习室和座位");
    }

    const modeVal = mode || "normal";
    const payload = {
      roomId: roomId,
      seatId: seatId,
      mode: modeVal,
    };
    const content = studyContent?.trim();
    if (content) {
      payload.studyContent = content;
    }

    const data = await studyApi.startSession(payload);
    const id = data.sessionId ?? data.id;

    lastSessionIdRef.current = null;
    restoreSession({
      ...data,
      sessionId: id,
      roomId,
      seatId,
      mode: modeVal,
      studyContent: content || null,
      durationMinutes: 0,
      status: StudySessionStatus.STUDYING,
    });

    return id;
  }, [restoreSession]);

  const pauseStudy = useCallback(async () => {
    if (!sessionId || sessionStatus !== StudySessionStatus.STUDYING) return;

    await studyApi.updateSession(sessionId, {
      status: StudySessionStatus.PAUSED,
    });

    setSessionStatus(StudySessionStatus.PAUSED);
    setActiveSession((prev) => prev ? { ...prev, status: StudySessionStatus.PAUSED } : prev);

    saveToStorage({
      sessionId,
      status: StudySessionStatus.PAUSED,
      roomId: activeSession?.roomId ?? null,
      roomName: activeSession?.roomName ?? null,
      seatId: activeSession?.seatId ?? null,
      seatCode: activeSession?.seatCode ?? null,
      startTime: startedAt,
      durationMinutes: Math.floor(totalDuration / 60),
      mode: timerMode,
    });
  }, [sessionId, sessionStatus, startedAt, timerMode, activeSession, totalDuration]);

  const resumeStudy = useCallback(async () => {
    if (!sessionId || sessionStatus !== StudySessionStatus.PAUSED) return;

    await studyApi.updateSession(sessionId, {
      status: StudySessionStatus.STUDYING,
    });

    setSessionStatus(StudySessionStatus.STUDYING);
    setActiveSession((prev) => prev ? { ...prev, status: StudySessionStatus.STUDYING } : prev);

    saveToStorage({
      sessionId,
      status: StudySessionStatus.STUDYING,
      roomId: activeSession?.roomId ?? null,
      roomName: activeSession?.roomName ?? null,
      seatId: activeSession?.seatId ?? null,
      seatCode: activeSession?.seatCode ?? null,
      startTime: startedAt,
      durationMinutes: Math.floor(totalDuration / 60),
      mode: timerMode,
    });
  }, [sessionId, sessionStatus, timerMode, activeSession, totalDuration, startedAt]);

  const endStudy = useCallback(async ({ studyContent } = {}) => {
    if (!sessionId) {
      throw new Error("没有正在进行的学习会话");
    }

    if (sessionStatus !== StudySessionStatus.STUDYING && sessionStatus !== StudySessionStatus.PAUSED) {
      throw new Error("当前学习会话状态不允许结束");
    }

    const payload = {
      status: StudySessionStatus.ENDED,
    };
    const content = studyContent?.trim();
    if (content) {
      payload.studyContent = content;
    }

    await studyApi.updateSession(sessionId, payload);

    const endedSessionId = sessionId;
    lastSessionIdRef.current = sessionId;
    clearSession();
    return endedSessionId;
  }, [sessionId, sessionStatus]);

  const submitEmotion = useCallback(
    async (data) => {
      const sid = sessionId || lastSessionIdRef.current;
      if (!sid) {
        throw new Error("没有可提交情绪的学习会话");
      }
      const payload = toEmotionPayload(data);
      if (!payload) {
        throw new Error("情绪信息不完整");
      }
      const raw = await emotionApi.submitEmotion(sid, payload);
      if (raw) {
        raw.aiFeedback = toAiFeedbackVM(raw, payload.emotionTag);
      }
      return raw;
  }, [sessionId]);








  const resetStudy = useCallback(() => {
    lastSessionIdRef.current = null;
    clearSession();
  }, [clearSession]);

  const value = {
    sessionId,
    sessionStatus,
    activeSession,
    startedAt,
    totalDuration,
    timerMode,
    isChecking,
    startStudy,
    pauseStudy,
    resumeStudy,
    endStudy,
    submitEmotion,
    syncActiveSession,
    sendHeartbeat: () => {
      if (sessionId && sessionStatus === StudySessionStatus.STUDYING) {
        studyApi.sendHeartbeat(sessionId).catch(() => {});
      }
    },
    resetStudy,
    setTimerMode,
    formatTime,
  };

  return <StudyContext.Provider value={value}>{children}</StudyContext.Provider>;
}

export function useStudy() {
  const ctx = useContext(StudyContext);
  if (!ctx) {
    throw new Error("useStudy must be used within a StudyProvider");
  }
  return ctx;
}

export default StudyContext;






