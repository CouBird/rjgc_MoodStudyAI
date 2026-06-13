/**
 * EmotionCard — 当前情绪状态展示卡片
 *
 * 展示当前/最新情绪标签、情绪评分、记录时间
 * 数据来自 API response 的 emotionRecord 字段
 */

import React from "react";

import { EMOTION_EMOJI_MAP } from "../../constants/emotions";

export default function EmotionCard({ emotionRecord, loading }) {
  if (loading) {
    return (
      <div className="bg-white rounded-2xl shadow-md p-6 animate-pulse">
        <div className="h-4 bg-gray-200 rounded w-1/3 mb-4"></div>
        <div className="h-8 bg-gray-200 rounded w-1/2 mb-2"></div>
        <div className="h-4 bg-gray-200 rounded w-2/3"></div>
      </div>
    );
  }

  if (!emotionRecord) {
    return (
      <div className="bg-white rounded-2xl shadow-md p-6 text-center">
        <div className="text-5xl mb-4">💭</div>
        <h3 className="text-lg font-bold text-gray-800 mb-2">暂无情绪记录</h3>
        <p className="text-gray-500">完成学习后记录你的心情吧</p>
      </div>
    );
  }

  const emotionTag = emotionRecord.emotionTag || emotionRecord.emotion_tag || "";
  const emoji = EMOTION_EMOJI_MAP[emotionTag] || "💭";
  const note = emotionRecord.userNote ?? emotionRecord.emotionNote ?? emotionRecord.note ?? "";
  const timeStr = emotionRecord.createdAt || emotionRecord.created_at
    ? new Date(emotionRecord.createdAt || emotionRecord.created_at).toLocaleString("zh-CN")
    : "";

  return (
    <div className="bg-white rounded-2xl shadow-md p-6">
      <h3 className="text-sm font-medium text-gray-500 mb-3">当前情绪</h3>
      <div className="flex items-center gap-4">
        <span className="text-5xl">{emoji}</span>
        <div>
          <p className="text-xl font-bold text-gray-800">{emotionTag}</p>
          {note && (
            <p className="text-sm text-gray-500 mt-1">{note}</p>
          )}
          {timeStr && <p className="text-xs text-gray-400 mt-1">{timeStr}</p>}
        </div>
      </div>
    </div>
  );
}

