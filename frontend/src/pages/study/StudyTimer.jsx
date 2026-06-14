import React, { useState, useEffect } from "react";
import { useTimer } from "../../hooks/useTimer";
import { useStudy } from "../../store/studyContext";

import { StudySessionStatus } from "../../constants/studySessionStatus";
// 10分钟
const MIN_SESSION_SECONDS = 600;

import { EMOTION_OPTIONS } from '../../constants/emotions';



export default function StudyTimer({ setCurrentPage, setIsStudying, setActiveRoomId, selectedSeat, selectedSeatCode, activeRoomId }) {
  const study = useStudy();
  const { timerMode, setTimerMode, totalSessionTime, pomodoroTime, pomoRound, isActive, setIsActive, isPaused, togglePause, isBreak, setIsBreak, breakTime, setBreakTime, formatTime } = useTimer();
  const [studyContent, setStudyContent] = useState("");
  const [showPomoModal, setShowPomoModal] = useState(false);
  const [showShortModal, setShowShortModal] = useState(false);
  const [showEmotion, setShowEmotion] = useState(false);
  const [showAiFeedback, setShowAiFeedback] = useState(false);
  const [showBreakModal, setShowBreakModal] = useState(false);
  const [selectedEmotion, setSelectedEmotion] = useState("平静");
  const [emotionNote, setEmotionNote] = useState("");
  const [aiFeedbackData, setAiFeedbackData] = useState(null);
  const [currentRoomName, setCurrentRoomName] = useState("");
  const [actionLoading, setActionLoading] = useState(false);

  useEffect(() => {
    setIsStudying(true); setActiveRoomId(activeRoomId || "1");
    return () => { setIsStudying(false); setActiveRoomId(null); };
  }, []);

  useEffect(() => {
    if (isBreak) { setShowBreakModal(true); }
    else setShowBreakModal(false);
  }, [isBreak]);

  const handleStart = async () => {
    if (study.sessionStatus === StudySessionStatus.STUDYING || study.sessionStatus === StudySessionStatus.PAUSED) return;
    setActionLoading(true);
    try {
      await study.startStudy({ roomId: activeRoomId || "1", seatId: selectedSeat, mode: "normal" });
      setIsActive(true);
    } catch (err) {
      console.error("开始学习失败:", err);
    } finally {
      setActionLoading(false);
    }
  };

  const handlePause = () => {
    togglePause();
    if (isActive && !isPaused) {
      study.pauseStudy().catch(() => {});
    }
  };

  const handleResume = () => {
    togglePause();
    study.resumeStudy().catch(() => {});
  };

  const handleTakeBreak = () => {
    if (isBreak) return;
    setIsBreak(true);
  };

  const handleEnd = () => {
    if (isBreak) return;
    if (totalSessionTime < MIN_SESSION_SECONDS) {
      togglePause();
      setShowShortModal(true);
      return;
    }
    finishSession();
  };

  const finishSession = async () => {
    // Check minimum session duration (works for both normal & pomodoro modes)
    const effectiveStudyTime = timerMode === "pomodoro"
      ? pomoRound * 25 * 60 + (25 * 60 - pomodoroTime)
      : totalSessionTime;
    if (effectiveStudyTime < MIN_SESSION_SECONDS && !isBreak) {
      togglePause();
      setShowShortModal(true);
      return;
    }
    setIsActive(false);
    setIsStudying(false);
    setActiveRoomId(null);
    try {
      await study.endStudy({ finalDuration: totalSessionTime });
    } catch (err) {
      console.error("结束学习失败:", err);
    }
    setShowEmotion(true);
  };

  const endShortSession = () => {
    setShowShortModal(false);
    setIsActive(false);
    setIsStudying(false);
    setActiveRoomId(null);
    setCurrentPage("home");
  };

  const handleEmotionSubmit = async (e) => {
    e.preventDefault();
    if (!selectedEmotion) { alert("请选择您的情绪标签"); return; }
    setActionLoading(true);
    try {
      const result = await study.submitEmotion({
        emotionTag: selectedEmotion,
        userNote: emotionNote || undefined,
      });
      if (result) {
        setAiFeedbackData(result.aiFeedback);
      }
    } catch (err) {
      console.error("提交情绪失败:", err);
    } finally {
      setActionLoading(false);
    }
    setShowEmotion(false);
    setShowAiFeedback(true);
  };

  const handleAiDone = () => { setShowAiFeedback(false); setCurrentPage("home"); };
  const handleModeSwitch = () => { if (timerMode === "pomodoro") setShowPomoModal(true); else setTimerMode("pomodoro"); };
  const confirmSwitch = () => { setTimerMode(timerMode === "pomodoro" ? "normal" : "pomodoro"); setShowPomoModal(false); };
  const handleExtendBreak = () => setBreakTime(prev => prev + 300);

  const endBreak = () => {
    setShowBreakModal(false);
    setIsBreak(false);
  };

  const seatLabel = selectedSeatCode || '--';

  const circumference = 2 * Math.PI * 45;
  const pomodoroElapsed = 25 * 60 - pomodoroTime;
  const pomodoroProgress = Math.min(pomodoroElapsed / (25 * 60), 1);
  const dashoffset = circumference * pomodoroProgress;

  const emotionOptions = EMOTION_OPTIONS;

  return (
    <div className="min-h-screen bg-white flex items-center justify-center">
      <div className="w-full max-w-2xl mx-auto px-4 py-6">
        <div className="bg-white rounded-2xl shadow-xl overflow-hidden">
          {/* Gradient Header */}
          <div className="bg-gradient-to-r from-primary to-secondary p-6 text-white text-center">
            <h2 className="text-2xl font-bold mb-2">{currentRoomName}</h2>
            <p className="text-white/80">座位号：{seatLabel}</p>
          </div>

          <div className="p-8 text-center">
            {/* Mode Switch */}
            <div className="flex flex-col sm:flex-row justify-between items-center mb-6 gap-4">
              <div className="flex gap-3">
                <button
                  onClick={() => timerMode === "pomodoro" ? setShowPomoModal(true) : null}
                  className={"px-4 py-2 rounded-lg text-sm font-medium transition-colors " + (timerMode === "normal" ? "bg-primary text-white" : "bg-gray-200 text-gray-700 hover:bg-gray-300")}
                >
                  <i className="fa fa-clock-o mr-1"></i>正向计时
                </button>
                <button
                  onClick={handleModeSwitch}
                  className={"px-4 py-2 rounded-lg text-sm font-medium transition-colors " + (timerMode === "pomodoro" ? "bg-primary text-white" : "bg-gray-200 text-gray-700 hover:bg-gray-300")}
                >
                  <i className="fa fa-tomato mr-1"></i>番茄钟
                </button>
              </div>
              {timerMode === "pomodoro" && (
                <span className="bg-primary/10 text-primary text-sm px-3 py-1 rounded-full">
                  <i className="fa fa-check-circle mr-1"></i>已完成 {pomoRound} 个番茄
                </span>
              )}
            </div>

            {timerMode === "normal" ? (
              <>
                <div className="mb-8">
                  <div className="text-6xl font-bold text-dark tracking-wider mb-4">{formatTime(totalSessionTime)}</div>
                  <p className="text-gray-500 text-sm">累计学习时间</p>
                </div>

                <div className="mb-6">
                  <input type="text" value={studyContent} onChange={(e) => setStudyContent(e.target.value)}
                    placeholder="记录一下你正在学习的内容..." maxLength={100}
                    className="w-full max-w-md px-4 py-3 border border-gray-300 rounded-lg text-center focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none transition-colors" />
                </div>

                {!isActive ? (
                  <button onClick={handleStart} disabled={actionLoading}
                    className="px-12 py-3 bg-primary text-white rounded-xl font-semibold text-lg hover:bg-primary/90 transition-all shadow-lg shadow-primary/20 flex items-center justify-center mx-auto gap-2 disabled:opacity-60">
                    <i className="fa fa-play"></i>{actionLoading ? "启动中..." : "开始学习"}
                  </button>
                ) : (
                  <div className="flex justify-center gap-4">
                    <button onClick={handleEnd}
                      className="px-8 py-3 bg-danger text-white rounded-xl font-semibold hover:bg-danger/90 transition-all flex items-center gap-2">
                      <i className="fa fa-stop"></i>结束学习
                    </button>
                    <button onClick={handleTakeBreak}
                      className="px-8 py-3 bg-warning text-white rounded-xl font-semibold hover:bg-warning/90 transition-all flex items-center gap-2">
                      <i className="fa fa-pause"></i>休息一下
                    </button>
                    {isActive && (
                      <button onClick={isPaused ? handleResume : handlePause}
                        className="px-8 py-3 bg-gray-200 text-gray-700 rounded-xl font-semibold hover:bg-gray-300 transition-all flex items-center gap-2">
                        <i className={"fa " + (isPaused ? "fa-play" : "fa-pause")}></i>{isPaused ? "继续" : "暂停"}
                      </button>
                    )}
                  </div>
                )}
              </>
            ) : (
              <>
                <div className="mb-6">
                  <input type="text" value={studyContent} onChange={(e) => setStudyContent(e.target.value)}
                    placeholder="记录一下你正在学习的内容..." maxLength={100}
                    className="w-full px-4 py-3 border border-gray-300 rounded-lg text-center focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none transition-colors" />
                </div>
                <div className="mb-8 relative">
                  <svg className="w-48 h-48 mx-auto transform -rotate-90" viewBox="0 0 100 100">
                    <circle cx="50" cy="50" r="45" fill="none" stroke="#e5e7eb" strokeWidth="6" />
                    <circle cx="50" cy="50" r="45" fill="none" stroke="#6366f1" strokeWidth="6"
                      strokeDasharray={circumference} strokeDashoffset={dashoffset} strokeLinecap="round"
                      className="transition-all duration-1000 ease-linear" />
                  </svg>
                  <div className="absolute inset-0 flex items-center justify-center">
                    <div>
                      <div className="text-5xl font-bold text-dark">{formatTime(pomodoroTime)}</div>
                      <p className="text-gray-500 text-sm mt-2">番茄 {pomoRound + 1}</p>
                    </div>
                  </div>
                </div>

                <div className="flex justify-center gap-4">
                  {!isActive ? (
                    <button onClick={() => setIsActive(true)}
                      className="px-12 py-3 bg-primary text-white rounded-xl font-semibold text-lg hover:bg-primary/90 transition-all shadow-lg shadow-primary/20 flex items-center gap-2">
                      <i className="fa fa-play"></i>开始番茄
                    </button>
                  ) : (
                    <>
                      <button onClick={finishSession}
                        className="px-8 py-3 bg-danger text-white rounded-xl font-semibold hover:bg-danger/90 transition-all flex items-center gap-2">
                        <i className="fa fa-stop"></i>结束
                      </button>
                      <button onClick={handleTakeBreak}
                        className="px-8 py-3 bg-warning text-white rounded-xl font-semibold hover:bg-warning/90 transition-all flex items-center gap-2">
                        <i className="fa fa-pause"></i>休息一下
                      </button>
                      <button onClick={isPaused ? (() => { togglePause(); }) : togglePause}
                        className="px-8 py-3 bg-gray-200 text-gray-700 rounded-xl font-semibold hover:bg-gray-300 transition-all flex items-center gap-2">
                        <i className={"fa " + (isPaused ? "fa-play" : "fa-pause")}></i>{isPaused ? "继续" : "暂停"}
                      </button>
                    </>
                  )}
                </div>
              </>
            )}
          </div>
        </div>
      </div>

      {/* 会话时长过短 */}
      {showShortModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white rounded-2xl shadow-2xl w-full max-w-sm mx-4">
            <div className="bg-gradient-to-r from-warning to-orange-400 p-6 text-white text-center rounded-t-2xl">
              <div className="w-16 h-16 rounded-full bg-white/20 mx-auto flex items-center justify-center mb-4">
                <i className="fa fa-clock-o text-3xl"></i>
              </div>
              <h3 className="text-xl font-bold mb-2">学习时间太短了</h3>
            </div>
            <div className="p-6 text-center">
              <p className="text-gray-600 mb-6">当前学习时长不足10分钟，无法生成有效的学习记录，建议学习至少10分钟哦！</p>
              <div className="flex gap-3">
                <button onClick={() => { setShowShortModal(false); if (isPaused) togglePause(); }} className="flex-1 bg-primary text-white py-2 rounded-lg hover:bg-primary/90 transition-colors">继续学习</button>
                <button onClick={endShortSession} className="flex-1 bg-gray-200 text-gray-700 py-2 rounded-lg hover:bg-gray-300 transition-colors">结束</button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* 番茄钟切换 */}
      {showPomoModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white rounded-2xl shadow-2xl w-full max-w-sm mx-4">
            <div className="bg-gradient-to-r from-primary to-secondary p-6 text-white text-center rounded-t-2xl">
              <h3 className="text-xl font-bold">切换模式</h3>
            </div>
            <div className="p-6 text-center">
              <p className="text-gray-600 mb-6">当前有正在进行的番茄钟，切换模式将重置当前番茄钟，是否确认？</p>
              <div className="flex gap-3">
                <button onClick={confirmSwitch} className="flex-1 bg-primary text-white py-2 rounded-lg hover:bg-primary/90 transition-colors">确认切换</button>
                <button onClick={() => setShowPomoModal(false)} className="flex-1 bg-gray-200 text-gray-700 py-2 rounded-lg hover:bg-gray-300 transition-colors">取消</button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* 休息 */}
      {showBreakModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white rounded-2xl shadow-2xl w-full max-w-sm mx-4">
            <div className="bg-gradient-to-r from-success to-teal-400 p-6 text-white text-center rounded-t-2xl">
              <div className="w-16 h-16 rounded-full bg-white/20 mx-auto flex items-center justify-center mb-4">
                <i className="fa fa-coffee text-3xl"></i>
              </div>
              <h3 className="text-xl font-bold mb-2">休息一下</h3>
            </div>
            <div className="p-6 text-center">
              <div className="text-4xl font-bold text-dark mb-2">{formatTime(breakTime)}</div>
              <p className="text-gray-500 mb-6">休息小贴士：站起来活动一下身体，看看远处的绿色植物，让眼睛和大脑都放松一下。</p>
              <div className="flex gap-4">
                <button onClick={endBreak} className="flex-1 bg-primary text-white py-2 rounded-lg hover:bg-primary/90 transition-colors">结束休息</button>
                <button onClick={handleExtendBreak} className="flex-1 bg-gray-200 text-gray-700 py-2 rounded-lg hover:bg-gray-300 transition-colors">延长5分钟</button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* 情绪 */}
      {showEmotion && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white rounded-2xl shadow-2xl w-full max-w-md mx-4">
            <div className="bg-gradient-to-r from-primary to-secondary p-6 text-white text-center rounded-t-2xl">
              <h3 className="text-xl font-bold mb-2">记录你的心情</h3>
              <p className="text-white/80">学习结束了，今天感觉怎么样？</p>
            </div>
            <form onSubmit={handleEmotionSubmit} className="p-6">
              <div className="grid grid-cols-4 gap-4 mb-6">
                {emotionOptions.map((e) => (
                  <button key={e.val} type="button" onClick={() => setSelectedEmotion(e.val)}
                    className={"flex flex-col items-center p-3 rounded-lg transition-all " + (selectedEmotion === e.val ? "bg-primary/10 border-2 border-primary" : "hover:bg-gray-100")}>
                    <span className="text-3xl mb-2">{e.emoji}</span>
                    <span className="text-sm text-gray-600">{e.label}</span>
                  </button>
                ))}
              </div>
              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-700 mb-2">备注（可选）</label>
                <textarea value={emotionNote} onChange={(e) => setEmotionNote(e.target.value)} rows={2} placeholder="写下你现在的感受..."
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none resize-none" />
              </div>
              <button type="submit" disabled={actionLoading} className="w-full bg-primary text-white py-2 rounded-lg hover:bg-primary/90 transition-colors disabled:opacity-60">
                {actionLoading ? "提交中..." : "提交"}
              </button>
            </form>
          </div>
        </div>
      )}

      {/* AI反馈 */}
      {showAiFeedback && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white rounded-2xl shadow-2xl w-full max-w-md mx-4">
            <div className="bg-gradient-to-r from-primary to-secondary p-6 text-white text-center rounded-t-2xl">
              <div className="w-16 h-16 rounded-full bg-white/20 mx-auto flex items-center justify-center mb-4">
                <i className="fa fa-robot text-3xl"></i>
              </div>
              <h3 className="text-xl font-bold mb-2">AI 情绪助手</h3>
            </div>
            <div className="p-6">
              <div className="bg-calm rounded-lg p-4 mb-6">
                <p className="text-gray-700">
                  {aiFeedbackData?.comfortText}
                </p>
              </div>
              <div className="space-y-3 mb-6">
                <div className="flex items-start">
                  <div className="w-8 h-8 rounded-full bg-success/10 flex items-center justify-center text-success mr-3 flex-shrink-0">
                    <i className="fa fa-lightbulb-o"></i>
                  </div>
                  <div>
                    <h4 className="font-medium text-sm mb-1">学习建议</h4>
                    <p className="text-sm text-gray-600">{aiFeedbackData?.studyAdvice}</p>
                  </div>
                </div>
                <div className="flex items-start">
                  <div className="w-8 h-8 rounded-full bg-warning/10 flex items-center justify-center text-warning mr-3 flex-shrink-0">
                    <i className="fa fa-heart"></i>
                  </div>
                  <div>
                    <h4 className="font-medium text-sm mb-1">放松建议</h4>
                    <p className="text-sm text-gray-600">{aiFeedbackData?.relaxAdvice}</p>
                  </div>
                </div>
              </div>
              <button onClick={handleAiDone} className="w-full bg-primary text-white py-2 rounded-lg hover:bg-primary/90 transition-colors">知道了</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}


