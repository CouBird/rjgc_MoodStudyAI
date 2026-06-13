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
    const saved = loadFromStorage();
    if (!saved || !saved.sessionId) {
      setIsChecking(false);
      return;
    }

    studyApi
      .getActiveSession()
      .then((data) => {
        if (data && data.sessionId) {
          restoreSession(data);
        } else {
          clearSession();
        }
      })
      .catch(() => {
        // Backend unreachable, restore from local
        if (saved) {
          restoreSession(saved);
        } else {
          clearSession();
        }
      })
      .finally(() => setIsChecking(false));
  }, [sessionId]);

  function restoreSession(data) {
    const id = data.sessionId ?? data.id;
    const status = data.status === StudySessionStatus.PAUSED ? StudySessionStatus.PAUSED : StudySessionStatus.STUDYING;
    setSessionId(id);
    setSessionStatus(status);
    // Use startTime (backend canonical field), fallback to startTime from localStorage
    setStartedAt(data.startTime ?? null);
    setTimerMode(data.mode ?? "normal");
    if (data.durationMinutes) {
      setTotalDuration(data.durationMinutes * 60);
    }
    saveToStorage({
      sessionId: id,
      status: status,
      startTime: data.startTime ?? null,
      mode: data.mode ?? "normal",
    });
  }

  function clearSession() {
    setSessionId(null);
    setSessionStatus(StudySessionStatus.IDLE);
    setStartedAt(null);
    setTotalDuration(0);
    setTimerMode("normal");
    saveToStorage(null);
  }

  // 

  const startStudy = useCallback(async ({ roomId, seatId, mode } = {}) => {
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

    const data = await studyApi.startSession(payload);
    const id = data.sessionId ?? data.id;

    setSessionId(id);
    setSessionStatus(StudySessionStatus.STUDYING);
    setStartedAt(data.startTime ?? new Date().toISOString());
    setTotalDuration(0);
    setTimerMode(modeVal);

    saveToStorage({
      sessionId: id,
      status: StudySessionStatus.STUDYING,
      startTime: data.startTime ?? new Date().toISOString(),
      mode: modeVal,
    });

    return id;
  }, []);

  const pauseStudy = useCallback(async () => {
    if (!sessionId || sessionStatus !== StudySessionStatus.STUDYING) return;

    await studyApi.updateSession(sessionId, {
      status: StudySessionStatus.PAUSED,
    });

    setSessionStatus(StudySessionStatus.PAUSED);

    saveToStorage({
      sessionId,
      status: StudySessionStatus.PAUSED,
      startTime: startedAt,
      mode: timerMode,
    });
  }, [sessionId, sessionStatus, startedAt, timerMode]);

  const resumeStudy = useCallback(async () => {
    if (!sessionId || sessionStatus !== StudySessionStatus.PAUSED) return;

    const now = new Date().toISOString();
    await studyApi.updateSession(sessionId, {
      status: StudySessionStatus.STUDYING,
    });

    setStartedAt(now);
    setSessionStatus(StudySessionStatus.STUDYING);

    saveToStorage({
      sessionId,
      status: StudySessionStatus.STUDYING,
      startTime: now,
      mode: timerMode,
    });
  }, [sessionId, sessionStatus, timerMode]);

  const endStudy = useCallback(async ({ finalDuration } = {}) => {
    if (!sessionId || (sessionStatus !== StudySessionStatus.STUDYING && sessionStatus !== StudySessionStatus.PAUSED)) {
      return null;
    }

    const duration = finalDuration || totalDuration;
    const now = new Date().toISOString();

    await studyApi.updateSession(sessionId, {
      status: StudySessionStatus.ENDED,
      endedAt: now,
    });

    const endedSessionId = sessionId;
    lastSessionIdRef.current = sessionId;
    clearSession();
    return endedSessionId;
  }, [sessionId, sessionStatus, totalDuration]);

  const submitEmotion = useCallback(
    async (data) => {
      const sid = sessionId || lastSessionIdRef.current;
      if (!sid) return null;
      const payload = toEmotionPayload(data);
      if (!payload) return null;
      const raw = await emotionApi.submitEmotion(sid, payload);
      if (raw && raw.aiFeedback) {
        raw.aiFeedback = toAiFeedbackVM(raw, payload.emotionTag);
      }
      return raw;
  }, [sessionId]);








  const resetStudy = useCallback(() => {
    clearSession();
  }, []);

  const value = {
    sessionId,
    sessionStatus,
    startedAt,
    totalDuration,
    timerMode,
    isChecking,
    startStudy,
    pauseStudy,
    resumeStudy,
    endStudy,
    submitEmotion,
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






