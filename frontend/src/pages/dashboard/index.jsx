import React, { useState, useEffect, useRef, useMemo } from "react";
import { Chart, registerables } from "chart.js";
import { statisticsApi } from "../../api/statistics";
import { emotionApi } from "../../api/emotion";
import { checkinApi } from "../../api/checkin";
import { toUserStatsVM, toEmotionTrendVM } from '../../viewmodels';

Chart.register(...registerables);

import { EMOTION_LABELS } from "../../constants/emotions";

// 情绪标签 → 图表纵轴值（仅 Chart.js 渲染用，与提交评分 EMOTION_SCORE_MAP 不同）
const CHART_EMOTION_SCORES = Object.freeze({
  "平静": 4, "自豪": 5, "满足": 4, "快乐": 5,
  "疲惫": 2, "焦虑": 2, "难过": 2,
});

const CARD_COLORS = {
  primary: { bg: "bg-primary/10", text: "text-primary" },
  secondary: { bg: "bg-secondary/10", text: "text-secondary" },
  success: { bg: "bg-success/10", text: "text-success" },
  warning: { bg: "bg-warning/10", text: "text-warning" },
};

function getEmotionPointColors(data) {
  return data.map((val) => (val >= 4 ? "#f59e0b" : "#10b981"));
}

function buildCalendarDays(year, month, checkedDates) {
  const today = new Date();
  const firstWeekday = new Date(year, month, 1).getDay();
  const daysInMonth = new Date(year, month + 1, 0).getDate();
  const currentDate = today.getDate();
  const cells = [];

  ["日", "一", "二", "三", "四", "五", "六"].forEach((name, i) => cells.push({ type: "header", label: name, key: `h-${i}` }));
  for (let i = 0; i < firstWeekday; i++) cells.push({ type: "empty", key: `e-${i}` });
  for (let d = 1; d <= daysInMonth; d++) {
    let status = "none";
    if (d === currentDate) status = "today";
    else if (checkedDates && checkedDates.has(d)) status = "checked";
    cells.push({ type: "day", day: d, status, key: `d-${d}` });
  }
  return cells;
}

function initStudyChart(canvas, labels, data) {
  if (!canvas || !labels || labels.length === 0) return null;
  const ctx = canvas.getContext("2d");
  return new Chart(ctx, {
    type: "line",
    data: {
      labels,
      datasets: [{
        label: "学习时长(小时)",
        data,
        borderColor: "#6366f1",
        backgroundColor: "rgba(99, 102, 241, 0.1)",
        tension: 0.4,
        fill: true,
      }],
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: { y: { beginAtZero: true, ticks: { callback: (v) => v + "h" } } },
    },
  });
}

function initEmotionChart(canvas, labels, data) {
  if (!canvas || !labels || labels.length === 0) return null;
  const ctx = canvas.getContext("2d");
  return new Chart(ctx, {
    type: "line",
    data: {
      labels,
      datasets: [{
        label: "情绪指数",
        data,
        borderColor: "#8b5cf6",
        backgroundColor: "rgba(139, 92, 246, 0.1)",
        tension: 0.4,
        fill: true,
        pointBackgroundColor: getEmotionPointColors(data),
        pointRadius: 6,
      }],
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: { y: { beginAtZero: true, max: 6, ticks: { callback: (v) => EMOTION_LABELS[v] || "" } } },
    },
  });
}

export default function Dashboard({ setCurrentPage }) {
  const [period, setPeriod] = useState("week");
  const [loading, setLoading] = useState(true);
  const [statsData, setStatsData] = useState(null);
  const [emotionTrend, setEmotionTrend] = useState(null);
  const [checkinData, setCheckinData] = useState(null);

  const studyRef = useRef(null);
  const emotionRef = useRef(null);
  const studyChartRef = useRef(null);
  const emotionChartRef = useRef(null);

  const now = new Date();
  const year = now.getFullYear();
  const month = now.getMonth();
  const calMonthStr = `${year}-${String(month + 1).padStart(2, "0")}`;

  const checkedDates = useMemo(() => {
    if (checkinData && Array.isArray(checkinData.days)) {
      return new Set(checkinData.days.filter((d) => d.checkedIn).map((d) => new Date(d.date + "T00:00:00").getDate()));
    }
    return null;
  }, [checkinData]);

  const calendarDays = useMemo(() => buildCalendarDays(year, month, checkedDates), [year, month, checkedDates]);

  useEffect(() => {
    setLoading(true);
    Promise.all([
      statisticsApi.getStudyStats({ period }).catch(() => null),
      emotionApi.getEmotionTrend({ period }).catch(() => null),
      checkinApi.getCalendar({ month: calMonthStr }).catch(() => null),
    ])
      .then(([stats, emotion, checkin]) => {
        setStatsData(toUserStatsVM(stats));
        setEmotionTrend(toEmotionTrendVM(emotion));
        setCheckinData(checkin);
      })
      .catch(() => {})
      .finally(() => setLoading(false));
  }, [period]);

  useEffect(() => {
    if (studyChartRef.current) studyChartRef.current.destroy();
    if (emotionChartRef.current) emotionChartRef.current.destroy();
    const chartLabels = statsData?.trends?.map((t) => {
      const d = new Date(t.date + "T00:00:00");
      return (d.getMonth() + 1) + "/" + d.getDate();
    }) || [];
    const timeData = statsData?.trends?.map((t) => t.hours) || [];
    const emotionVals = emotionTrend?.items?.map((i) => {
      return CHART_EMOTION_SCORES[i.emotionTag] ?? 3;
    }) || [];
    if (chartLabels.length > 0) {
      studyChartRef.current = initStudyChart(studyRef.current, chartLabels, timeData);
      emotionChartRef.current = initEmotionChart(emotionRef.current, chartLabels, emotionVals);
    }
    return () => {
      if (studyChartRef.current) studyChartRef.current.destroy();
      if (emotionChartRef.current) emotionChartRef.current.destroy();
    };
  }, [statsData, emotionTrend]);

  const totalHours = statsData?.totalHours ?? 0;
  const avgHours = statsData?.averageDailyHours ?? 0;
  const streakDays = statsData?.streakDays ?? 0;
  const totalSessions = statsData?.validSessionCount ?? statsData?.checkinCount ?? 0;

  const mainEmotion = {
    emoji: "??",
    text: emotionTrend?.mainEmotion || "平静",
    desc: emotionTrend?.summary || "暂无数据",
  };

  const cards = [
    { id: "total", label: "总学习时长", value: totalHours, unit: "小时", change: null, isUp: null, icon: "fa-clock", color: "primary" },
    { id: "avg", label: "日均学习", value: avgHours, unit: "小时", change: null, isUp: null, icon: "fa-calendar-check", color: "secondary" },
    { id: "streak", label: "连续打卡", value: streakDays, unit: "天", change: null, isUp: null, icon: "fa-fire", color: "success" },
    { id: "count", label: "完成打卡", value: totalSessions, unit: "次", change: null, isUp: null, icon: "fa-check-circle", color: "warning" },
  ];

  return (
    <div className="container mx-auto px-4 pb-16">
      <div className="mb-6">
        <h2 className="text-2xl font-bold mb-6">学习统计</h2>
        <div className="flex flex-wrap gap-2">
          {["week", "month", "year"].map((p) => (
            <button key={p} onClick={() => setPeriod(p)}
              className={`px-4 py-2 rounded-full text-sm transition-all ${period === p ? "bg-primary text-white" : "bg-white text-gray-700 hover:bg-gray-100"}`}>
              {p === "week" ? "本周" : p === "month" ? "本月" : "本年"}
            </button>
          ))}
        </div>
      </div>

      {mainEmotion.text && (
        <div className="bg-white rounded-xl shadow-md p-6 mb-6 flex items-center space-x-4">
          <div className="w-12 h-12 rounded-full bg-gradient-to-r from-primary/20 to-secondary/20 flex items-center justify-center text-2xl shrink-0">
            {mainEmotion.emoji}
          </div>
          <div>
            <h3 className="text-lg font-semibold text-gray-800">
              主要情绪：<span className="text-primary font-bold">{mainEmotion.text}</span>
            </h3>
            <p className="text-sm text-gray-600 mt-1">{mainEmotion.desc}</p>
          </div>
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        {cards.map((card) => {
          const colors = CARD_COLORS[card.color] || CARD_COLORS.primary;
          return (
            <div key={card.id} className="bg-white rounded-xl shadow-md p-6 card-hover">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-sm font-medium text-gray-500">{card.label}</h3>
                <div className={`w-10 h-10 rounded-full ${colors.bg} flex items-center justify-center ${colors.text}`}>
                  <i className={`fa ${card.icon}`}></i>
                </div>
              </div>
              <p className="text-3xl font-bold text-dark">
                {card.value}<span className="text-lg font-normal text-gray-500 ml-1">{card.unit}</span>
              </p>
            </div>
          );
        })}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
        <div className="bg-white rounded-xl shadow-md p-6">
          <h3 className="text-lg font-semibold mb-6">学习时长趋势</h3>
          <div className="h-64"><canvas ref={studyRef}></canvas></div>
        </div>
        <div className="bg-white rounded-xl shadow-md p-6">
          <h3 className="text-lg font-semibold mb-6">情绪变化趋势</h3>
          <div className="h-64"><canvas ref={emotionRef}></canvas></div>
        </div>
      </div>

      <div className="bg-white rounded-xl shadow-md p-6">
        <h3 className="text-lg font-semibold mb-6">学习打卡日历</h3>
        <div className="grid grid-cols-7 gap-2">
          {calendarDays.map((item) => {
            if (item.type === "header") {
              return <div key={item.key} className="text-center text-sm font-medium text-gray-500 py-2">{item.label}</div>;
            }
            if (item.type === "empty") {
              return <div key={item.key} className="h-10"></div>;
            }
            return (
              <div key={item.key}
                className={`h-10 flex items-center justify-center rounded text-sm ${item.status === "today" ? "bg-primary text-white font-medium" : item.status === "checked" ? "bg-success text-white" : "bg-gray-200 text-gray-600"}`}>
                {item.day}
              </div>
            );
          })}
        </div>
        <div className="mt-4 flex justify-center space-x-6 text-sm text-gray-600">
          <div className="flex items-center"><div className="w-4 h-4 rounded bg-success mr-2"></div><span>已打卡</span></div>
          <div className="flex items-center"><div className="w-4 h-4 rounded bg-gray-200 mr-2"></div><span>未打卡</span></div>
          <div className="flex items-center"><div className="w-4 h-4 rounded bg-primary mr-2"></div><span>今天</span></div>
        </div>
      </div>
    </div>
  );
}



