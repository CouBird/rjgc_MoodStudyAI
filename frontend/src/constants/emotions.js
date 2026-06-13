/**
 * 情绪模块常量 — 统一来源
 *
 * 所有 UI 组件的情绪展示数据必须从此文件引用
 * 禁止在各组件中重复定义 EMOTION_EMOJI_MAP / EMOTION_SCORE_MAP
 */

// 情绪标签 → Emoji
export const EMOTION_EMOJI_MAP = Object.freeze({
  "平静": "😊",
  "自豪": "😎",
  "满足": "😌",
  "快乐": "😄",
  "疲惫": "😴",
  "焦虑": "😰",
  "难过": "😢",
});

// 情绪标签 → 默认分数（前端提交时使用）
export const EMOTION_SCORE_MAP = Object.freeze({
  "平静": 5,
  "自豪": 8,
  "满足": 7,
  "快乐": 9,
  "疲惫": 3,
  "焦虑": 2,
  "难过": 2,
});

// 情绪标签 → 安慰文本（AI 反馈 fallback）
export const EMOTION_COMFORT_MAP = Object.freeze({
  "平静": "今天的你也辛苦了！完成了专注学习，非常棒！闭上眼睛，做10次深呼吸，让大脑放松下来。",
  "自豪": "你真棒！完成了学习目标，这份成就感值得被记住！继续保持，你正在成为更好的自己。",
  "满足": "满足于当下的进步，这是最好的学习状态。享受这份充实感，它为明天的学习注入能量。",
  "快乐": "学习让你感到快乐，这是最棒的事情！带着这份愉悦的心情去休息一下吧。",
  "疲惫": "感觉到疲惫是正常的，说明你真的投入了。好好休息一下，恢复精力后再继续。",
  "焦虑": "感到焦虑时，试着把注意力放在呼吸上。你已经做得很好了，每一步都算数。",
  "难过": "没关系，每个人都会有低落的时候。允许自己难过一会儿，然后重新出发。",
});

// 情绪下拉选项（用于 StudyTimer 的情绪选择器）
export const EMOTION_OPTIONS = Object.freeze([
  { label: "平静", emoji: "😊", val: "平静" },
  { label: "自豪", emoji: "😎", val: "自豪" },
  { label: "满足", emoji: "😌", val: "满足" },
  { label: "快乐", emoji: "😄", val: "快乐" },
  { label: "疲惫", emoji: "😴", val: "疲惫" },
  { label: "焦虑", emoji: "😰", val: "焦虑" },
  { label: "难过", emoji: "😢", val: "难过" },
]);

export const EMOTION_LABELS = Object.freeze(["平静", "自豪", "满足", "快乐", "焦虑", "疲惫", "难过"]);
