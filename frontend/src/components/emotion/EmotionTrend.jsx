/**
 * EmotionTrend — 情绪趋势展示组件
 *
 * 展示情绪趋势数据，包括：
 * - 主导情绪概览
 * - 情绪变化列表
 * - 情绪总结
 *
 * 数据来自 emotionApi.getEmotionTrend 返回
 */

import React from "react";

import { EMOTION_EMOJI_MAP } from "../../constants/emotions";

export default function EmotionTrend({ trendData, loading, error }) {
  if (loading) {
    return (
      <div className="bg-white rounded-2xl shadow-md p-6 animate-pulse">
        <div className="h-4 bg-gray-200 rounded w-1/4 mb-6"></div>
        <div className="h-16 bg-gray-200 rounded mb-4"></div>
        <div className="h-8 bg-gray-200 rounded w-full mb-2"></div>
        <div className="h-8 bg-gray-200 rounded w-full"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-white rounded-2xl shadow-md p-6 text-center">
        <div className="text-4xl mb-3">⚠️</div>
        <p className="text-gray-500 text-sm">{error}</p>
      </div>
    );
  }

  if (!trendData) {
    return (
      <div className="bg-white rounded-2xl shadow-md p-6 text-center">
        <p className="text-gray-400">暂无趋势数据</p>
      </div>
    );
  }

  const mainEmoji = EMOTION_EMOJI_MAP[trendData.mainEmotion] || "💭";

  return (
    <div className="bg-white rounded-2xl shadow-md p-6">
      <h3 className="text-sm font-medium text-gray-500 mb-4">情绪趋势</h3>

      {/* 主导情绪 */}
      <div className="bg-calm rounded-xl p-4 mb-4">
        <div className="flex items-center gap-3">
          <span className="text-3xl">{mainEmoji}</span>
          <div>
            <p className="text-sm text-gray-500">主导情绪</p>
            <p className="text-lg font-bold text-gray-800">{trendData.mainEmotion}</p>
          </div>
        </div>
        {trendData.summary && (
          <p className="text-sm text-gray-600 mt-2">{trendData.summary}</p>
        )}
      </div>

      {/* 情绪变化列表 */}
      {trendData.items && trendData.items.length > 0 && (
        <div>
          <h4 className="text-xs font-medium text-gray-400 uppercase mb-2">近期记录</h4>
          <div className="space-y-2">
            {trendData.items.slice(0, 10).map((item, idx) => (
              <div key={idx} className="flex items-center justify-between py-2 border-b border-gray-100 last:border-0">
                <div className="flex items-center gap-2">
                  <span>{EMOTION_EMOJI_MAP[item.emotionTag] || "💭"}</span>
                  <span className="text-sm text-gray-700">{item.emotionTag}</span>
                </div>
                <span className="text-xs text-gray-400">{item.date}</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

