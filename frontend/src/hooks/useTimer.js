import { formatTime } from '../utils';
import { useState, useEffect, useRef } from "react";

export function useTimer() {
  const [timerMode, setTimerMode] = useState("normal"); // "normal" | "pomodoro"
  const [totalSessionTime, setTotalSessionTime] = useState(0);
  const [pomodoroTime, setPomodoroTime] = useState(25 * 60);
  const [pomoRound, setPomoRound] = useState(0);
  const [isActive, setIsActive] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const [isBreak, setIsBreak] = useState(false);
  const [breakTime, setBreakTime] = useState(5 * 60);

  const timerRef = useRef(null);
  const breakTimerRef = useRef(null);

  // 主计时器：正向计时 + 番茄倒计时
  useEffect(() => {
    if (isActive && !isBreak && !isPaused) {
      timerRef.current = setInterval(() => {
        if (timerMode === "normal") {
          setTotalSessionTime((prev) => prev + 1);
        } else {
          setPomodoroTime((prev) => {
            if (prev <= 1) {
              clearInterval(timerRef.current);
              setIsActive(false);
              setPomoRound((r) => r + 1);
              setIsBreak(true);
              return 25 * 60;
            }
            return prev - 1;
          });
        }
      }, 1000);
    } else {
      clearInterval(timerRef.current);
    }
    return () => clearInterval(timerRef.current);
  }, [isActive, timerMode, isBreak, isPaused]);

  // 休息倒计时
  useEffect(() => {
    if (isBreak) {
      breakTimerRef.current = setInterval(() => {
        setBreakTime((prev) => {
          if (prev <= 1) {
            clearInterval(breakTimerRef.current);
            setIsBreak(false);
            setIsActive(true);
            return 5 * 60;
          }
          return prev - 1;
        });
      }, 1000);
    } else {
      clearInterval(breakTimerRef.current);
    }
    return () => clearInterval(breakTimerRef.current);
  }, [isBreak]);

  const togglePause = () => {
    setIsPaused((prev) => !prev);
  };

  const resetTimer = () => {
    setTotalSessionTime(0);
    setPomodoroTime(25 * 60);
    setPomoRound(0);
    setIsActive(false);
    setIsPaused(false);
    setIsBreak(false);
    setBreakTime(5 * 60);
  };

  // 切换模式时重置番茄状态
  const switchMode = (newMode) => {
    setTimerMode(newMode);
    if (newMode === "normal") {
      setPomodoroTime(25 * 60);
      setPomoRound(0);
      setIsBreak(false);
    }
  };

  // 格式化秒数为 MM:SS 或 HH:MM:SS


  return {
    timerMode,
    setTimerMode: switchMode,
    isPaused,
    setIsPaused,
    togglePause,
    resetTimer,
    totalSessionTime,
    setTotalSessionTime,
    pomodoroTime,
    setPomodoroTime,
    pomoRound,
    setPomoRound,
    isActive,
    setIsActive,
    isBreak,
    setIsBreak,
    breakTime,
    setBreakTime,
    formatTime,
  };
}

