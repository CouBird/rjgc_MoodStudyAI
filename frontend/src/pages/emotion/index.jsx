/**
 * 情绪分析页面
 *
 * 展示：
 * - 当前情绪状态
 * - 情绪趋势变化
 * - AI 分析反馈
 * - 历史情绪记录
 */

import React, { useEffect, useState, useCallback } from "react";
import EmotionCard from "../../components/emotion/EmotionCard";
import EmotionTrend from "../../components/emotion/EmotionTrend";
import AiFeedbackCard from "../../components/emotion/AiFeedbackCard";
import { emotionApi } from "../../api/emotion";

const PERIOD_OPTIONS = [
  { label: "本周", value: "week" },
  { label: "本月", value: "month" },
  { label: "本年", value: "year" },
];

export default function EmotionPage({ setCurrentPage }) {
  const [activePeriod, setActivePeriod] = useState("week");
  const [trendData, setTrendData] = useState(null);
  const [trendLoading, setTrendLoading] = useState(false);
  const [trendError, setTrendError] = useState(null);
  const [latestRecord, setLatestRecord] = useState(null);
  const [latestLoading, setLatestLoading] = useState(false);
  const [aiFeedback, setAiFeedback] = useState(null);

  // 获取趋势数据
  const loadTrend = useCallback(async (period) => {
    setTrendLoading(true);
    setTrendError(null);
    try {
      const data = await emotionApi.getEmotionTrend({ period });
      setTrendData(data);
    } catch (err) {
      setTrendError(err.message || "获取情绪趋势失败");
    } finally {
      setTrendLoading(false);
    }
  }, []);

  // 获取最新情绪记录
  const loadLatestRecord = useCallback(async () => {
    setLatestLoading(true);
    try {
      // 获取最近一周的记录，取最新的
      const data = await emotionApi.getEmotionTrend({ period: "week" });
      if (data && data.items && data.items.length > 0) {
        const latest = data.items[data.items.length - 1];
        setLatestRecord({
          emotionTag: latest.emotionTag,
          createdAt: latest.date ? new Date(latest.date).toISOString() : null,
        });
      }
    } catch {
      // 静默处理
    } finally {
      setLatestLoading(false);
    }
  }, []);

  useEffect(() => {
    loadTrend(activePeriod);
  }, [activePeriod, loadTrend]);

  useEffect(() => {
    loadLatestRecord();
  }, [loadLatestRecord]);

  const handlePeriodChange = (period) => {
    setActivePeriod(period);
  };

  const handleAiFeedback = (feedback) => {
    // 后续接入后端反馈接口时替换
    // emotionApi.submitAiFeedback — 后端未实现，功能已禁用
  };

  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      {/* 页面标题 */}
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-gray-800 mb-2">情绪分析</h1>
        <p className="text-gray-500">了解你的学习情绪变化，获得 AI 温暖反馈</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        {/* 当前情绪 */}
        <EmotionCard emotionRecord={latestRecord} loading={latestLoading} />

        {/* 情绪趋势 */}
        <div>
          {/* 周期切换 */}
          <div className="flex gap-2 mb-4">
            {PERIOD_OPTIONS.map((opt) => (
              <button
                key={opt.value}
                onClick={() => handlePeriodChange(opt.value)}
                className={"px-4 py-2 rounded-lg text-sm font-medium transition-colors " +
                  (activePeriod === opt.value
                    ? "bg-primary text-white"
                    : "bg-white text-gray-600 hover:bg-gray-100 border border-gray-200")
                }
              >
                {opt.label}
              </button>
            ))}
          </div>
          <EmotionTrend
            trendData={trendData}
            loading={trendLoading}
            error={trendError}
          />
        </div>
      </div>

      {/* AI 反馈展示 */}
      {aiFeedback && (
        <div className="mb-8">
          <AiFeedbackCard
            aiFeedback={aiFeedback}
            onFeedback={handleAiFeedback}
          />
        </div>
      )}

      {/* 无数据时的引导 */}
      {!trendData && !trendLoading && !trendError && (
        <div className="text-center py-16">
          <div className="text-6xl mb-4">📊</div>
          <h2 className="text-xl font-bold text-gray-700 mb-2">开始记录你的情绪</h2>
          <p className="text-gray-400 mb-6">完成学习后记录心情，AI 会为你提供温暖的反馈</p>
          <button
            onClick={() => setCurrentPage("study-rooms")}
            className="px-6 py-3 bg-primary text-white rounded-xl hover:bg-primary/90 transition-colors"
          >
            去自习室学习
          </button>
        </div>
      )}
    </div>
  );
}


