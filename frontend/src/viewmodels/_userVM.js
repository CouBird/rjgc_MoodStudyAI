/**
 * UserVM 纯函数 — 无 React/Store 依赖
 *
 * 将 raw UserResponse 转为 UserVM
 * 此模块不依赖 React context，可在任何层安全使用
 */
export function toUserVM(raw) {
  if (!raw) return null;

  return Object.freeze({
    userId: raw.userId ?? null,
    phone: raw.phone ?? null,
    nickname: raw.nickname ?? null,
    displayName: raw.nickname ?? null,
    avatarUrl: raw.avatarUrl ?? null,
    role: raw.role ?? "user",
    status: raw.status ?? "active",
    createdAt: raw.createdAt ?? null,
    isAuthenticated: true,
  });
}

export default toUserVM;
