/**
 * 工具函数模块
 *
 * 纯函数集合 — 无 React 依赖，可在任何层安全使用。
 */

/**
 * 检测文本是否包含敏感词
 */
export const checkSensitiveWord = (text) => {
  if (!text) return false;
  const sensitiveWords = ['cnm', '违法词汇', '极端词汇', '敏感词汇'];
  const lowerText = text.toLowerCase();
  return sensitiveWords.some(word => lowerText.includes(word));
};

/**
 * 获取最早可选日期时间字符串（当前时间+1小时）
 */
export const getMinDateTimeString = () => {
  const minTime = new Date();
  minTime.setHours(minTime.getHours() + 1);
  const y = minTime.getFullYear();
  const M = String(minTime.getMonth() + 1).padStart(2, '0');
  const d = String(minTime.getDate()).padStart(2, '0');
  const h = String(minTime.getHours()).padStart(2, '0');
  const m = String(minTime.getMinutes()).padStart(2, '0');
  return y + '-' + M + '-' + d + 'T' + h + ':' + m;
};

/**
 * 格式化秒数为 MM:SS 或 HH:MM:SS
 * 所有内部时长存储使用秒（seconds），仅在显示边界调用此函数
 */
export const formatTime = (totalSeconds) => {
  const h = String(Math.floor(totalSeconds / 3600)).padStart(2, '0');
  const m = String(Math.floor((totalSeconds % 3600) / 60)).padStart(2, '0');
  const s = String(totalSeconds % 60).padStart(2, '0');
  return h === '00' ? m + ':' + s : h + ':' + m + ':' + s;
};

export const avatarFallback = (name = '用户') => {
  const label = String(name || '用户').trim().slice(0, 1) || '用';
  const svg = `
    <svg xmlns="http://www.w3.org/2000/svg" width="96" height="96" viewBox="0 0 96 96">
      <rect width="96" height="96" rx="48" fill="#6366f1"/>
      <text x="48" y="56" text-anchor="middle" font-size="34" font-family="Arial, sans-serif" font-weight="700" fill="#ffffff">${label}</text>
    </svg>`;
  return `data:image/svg+xml;charset=UTF-8,${encodeURIComponent(svg)}`;
};

export const resolveAvatarUrl = (avatarUrl, name = '用户') => {
  const value = String(avatarUrl || '').trim();
  if (!value || value === 'default.png' || value === 'unknown-avatar') {
    return avatarFallback(name);
  }

  if (/^(https?:|data:|blob:)/i.test(value)) {
    return value;
  }

  if (value.startsWith('/')) {
    return value;
  }

  if (value.startsWith('storage/')) {
    return `/${value}`;
  }

  return `/storage/avatars/${value}`;
};
