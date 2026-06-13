/**
 * UserVM — 统一用户视图模型（React Hook 包装）
 *
 * UI 组件唯一入口：useUserVM()
 * 禁止在组件内直接访问 userContext.userInfo
 */

import { useUser } from "../store/userContext";
import { toUserVM } from "./_userVM";

/**
 * useUserVM — UI 组件唯一入口
 */
export function useUserVM() {
  const ctx = useUser();
  if (!ctx.userInfo) {
    return {
      userId: null,
      phone: null,
      nickname: null,
      displayName: null,
      avatarUrl: null,
      role: null,
      status: null,
      createdAt: null,
      isAuthenticated: false,
      loading: ctx.loading,
    };
  }
  return {
    ...toUserVM(ctx.userInfo),
    loading: ctx.loading,
    isAuthenticated: true,
  };
}

export { toUserVM } from "./_userVM";
export default toUserVM;
