/**
 * AiFeedbackCard — AI 情绪反馈展示卡片
 *
 * 展示 AI 分析结果：
 * - 安慰文本 (comfortText)
 * - 学习建议 (studyAdvice)
 * - 放松建议 (relaxAdvice)
 *
 * 支持用户评分反馈
 */

import React, { useState } from "react";

export default function AiFeedbackCard({ aiFeedback, loading, onFeedback }) {
  const [userRating, setUserRating] = useState(null); // 'helpful' | 'unhelpful'
  const [feedbackSubmitted, setFeedbackSubmitted] = useState(false);

  if (loading) {
    return (
      <div className="bg-white rounded-2xl shadow-md p-6 animate-pulse">
        <div className="h-4 bg-gray-200 rounded w-1/4 mb-6"></div>
        <div className="h-20 bg-gray-200 rounded mb-4"></div>
        <div className="h-12 bg-gray-200 rounded mb-2"></div>
        <div className="h-12 bg-gray-200 rounded"></div>
      </div>
    );
  }

  if (!aiFeedback) {
    return null;
  }

  const handleFeedback = (rating) => {
    setUserRating(rating);
    setFeedbackSubmitted(true);
    if (onFeedback) {
      onFeedback({ rating, comfortText: aiFeedback.comfortText });
    }
  };

  return (
    <div className="bg-white rounded-2xl shadow-md p-6">
      <div className="flex items-center gap-3 mb-4">
        <div className="w-10 h-10 rounded-full bg-primary/10 flex items-center justify-center text-primary">
          <i className="fa fa-robot text-lg"></i>
        </div>
        <h3 className="text-lg font-bold text-gray-800">AI 情绪分析</h3>
      </div>

      {/* 安慰文本 */}
      <div className="bg-calm rounded-xl p-4 mb-4">
        <p className="text-gray-700 leading-relaxed">{aiFeedback.comfortText}</p>
      </div>

      {/* 建议列表 */}
      <div className="space-y-3 mb-4">
        <div className="flex items-start">
          <div className="w-8 h-8 rounded-full bg-success/10 flex items-center justify-center text-success mr-3 flex-shrink-0">
            <i className="fa fa-lightbulb-o"></i>
          </div>
          <div>
            <h4 className="font-medium text-sm text-gray-700 mb-1">学习建议</h4>
            <p className="text-sm text-gray-500">{aiFeedback.studyAdvice}</p>
          </div>
        </div>
        <div className="flex items-start">
          <div className="w-8 h-8 rounded-full bg-warning/10 flex items-center justify-center text-warning mr-3 flex-shrink-0">
            <i className="fa fa-heart"></i>
          </div>
          <div>
            <h4 className="font-medium text-sm text-gray-700 mb-1">放松建议</h4>
            <p className="text-sm text-gray-500">{aiFeedback.relaxAdvice}</p>
          </div>
        </div>
      </div>

      {/* 用户反馈 */}
      <div className="border-t border-gray-100 pt-4">
        <p className="text-xs text-gray-400 mb-2">这个分析对你有帮助吗？</p>
        <div className="flex gap-3">
          <button
            onClick={() => handleFeedback("helpful")}
            disabled={feedbackSubmitted}
            className={"flex items-center gap-1 px-4 py-2 rounded-lg text-sm transition-colors " +
              (userRating === "helpful"
                ? "bg-success/10 text-success border border-success/30"
                : feedbackSubmitted
                  ? "bg-gray-100 text-gray-400 cursor-not-allowed"
                  : "bg-gray-100 text-gray-600 hover:bg-gray-200")
            }
          >
            <i className="fa fa-thumbs-up"></i>
            <span>有帮助</span>
          </button>
          <button
            onClick={() => handleFeedback("unhelpful")}
            disabled={feedbackSubmitted}
            className={"flex items-center gap-1 px-4 py-2 rounded-lg text-sm transition-colors " +
              (userRating === "unhelpful"
                ? "bg-danger/10 text-danger border border-danger/30"
                : feedbackSubmitted
                  ? "bg-gray-100 text-gray-400 cursor-not-allowed"
                  : "bg-gray-100 text-gray-600 hover:bg-gray-200")
            }
          >
            <i className="fa fa-thumbs-down"></i>
            <span>待改进</span>
          </button>
        </div>
        {feedbackSubmitted && (
          <p className="text-xs text-success mt-2">感谢你的反馈！</p>
        )}
      </div>
    </div>
  );
}
