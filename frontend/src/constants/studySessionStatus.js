/**
 * 学习会话状态常量
 *
 * 所有组件/上下文必须使用此枚举值
 * 
 * IDLE  / BREAK — 前端内部状态（后端不存储）
 * STUDYING / PAUSED / ENDED — 与后端 state_machine 一致
 */
export const StudySessionStatus = Object.freeze({
  /** 未开始 / 已结束（前端内部） */
  IDLE: "idle",
  /** 学习中（映射后端 state = studying） */
  STUDYING: "studying",
  /** 已暂停（映射后端 state = paused） */
  PAUSED: "paused",
  /** 已结束（映射后端 state = ended） */
  ENDED: "ended",
  /** 休息中（番茄钟模式，前端内部） */
  BREAK: "break",
});
