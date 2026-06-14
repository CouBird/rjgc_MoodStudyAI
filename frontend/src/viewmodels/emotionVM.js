/**
 * EmotionVM — 情绪模块视图模型
 *
 * 职责：字段归一化、分数映射、响应格式化、AI反馈补全
 * 禁止在 API 层处理业务逻辑。
 *
 * 后端响应格式差异：
 *   aiFeedback 字段实际为 String（文档预期为对象），本层统一转换
 *   当 aiFeedback 缺失字段时，本层从 EMOTION_COMFORT_MAP 注入默认值
 */

import { EMOTION_SCORE_MAP, EMOTION_COMFORT_MAP } from '../constants/emotions';

/**
 * 构建情绪提交请求体
 * 归一化字段名，注入分数
 */
export function toEmotionPayload(data) {
  if (!data) return null;

  const emotionTag = data.emotionTag ?? data.emotion ?? null;
  if (!emotionTag) {
    if (import.meta.env.DEV) {
      console.warn('[EmotionVM] missing emotionTag in payload data');
    }
    return null;
  }

  const payload = {
    emotionTag,
    emotionScore: data.emotionScore ?? EMOTION_SCORE_MAP[emotionTag] ?? 5,
  };

  // 后端字段名为 userNote，归一化输入
  const note = data.userNote ?? data.emotionNote ?? data.note ?? null;
  if (note) {
    payload.userNote = note;
  }

  return payload;
}

/**
 * 转换 AI 反馈响应为完整对象格式
 * 当后端返回 String 或对象缺字段时，从 EMOTION_COMFORT_MAP 注入默认值
 *
 * 注意：emotionTag 需要从调用方传入以匹配正确的安慰文本
 */
export function toAiFeedbackVM(raw, emotionTag) {
  if (!raw) return null;

  const feedback = parseAiFeedback(raw.aiFeedback ?? raw);
  const comfortText = feedback?.comfortText ?? feedback?.comfort_text ?? null;
  const studyAdvice = feedback?.studyAdvice ?? feedback?.study_advice ?? raw.studyAdvice ?? null;
  const relaxAdvice = feedback?.relaxAdvice ?? feedback?.relax_advice ?? raw.relaxAdvice ?? null;

  // 注入默认值（仅当对应字段缺失时）
  const defaultComfort = emotionTag ? (EMOTION_COMFORT_MAP[emotionTag] ?? null) : null;

  return Object.freeze({
    comfortText: comfortText || defaultComfort || '每一种情绪都值得被接纳。感谢你今天坚持学习。',
    studyAdvice: studyAdvice || '建议每学习25分钟休息5分钟，劳逸结合效率更高',
    relaxAdvice: relaxAdvice || '可以听听轻音乐，或者站起来活动一下身体',
  });
}

function parseAiFeedback(value) {
  if (!value) return null;

  if (typeof value === 'object') {
    return value;
  }

  if (typeof value !== 'string') {
    return null;
  }

  const trimmed = value.trim();
  if (!trimmed) return null;

  try {
    const parsed = JSON.parse(trimmed);
    if (parsed && typeof parsed === 'object') {
      return parsed;
    }
  } catch {
    // 兼容旧后端：纯文本反馈直接作为 comfortText 展示。
  }

  return { comfortText: trimmed };
}

/**
 * 转换情绪趋势数据为 ViewModel
 */
export function toEmotionTrendVM(raw) {
  if (!raw) return null;

  return Object.freeze({
    period: raw.period ?? null,
    mainEmotion: raw.mainEmotion ?? null,
    summary: raw.summary ?? null,
    items: Array.isArray(raw.items) ? raw.items.map(item => ({
      emotionTag: item.emotionTag ?? null,
      emotionValue: item.emotionValue ?? null,
      count: item.count ?? 0,
      date: item.date ?? null,
    })) : [],
    emotionMap: raw.emotionMap ?? null,
    trends: Array.isArray(raw.trends) ? raw.trends : [],
    tagDistribution: raw.tagDistribution ?? null,
  });
}

export default toEmotionPayload;
