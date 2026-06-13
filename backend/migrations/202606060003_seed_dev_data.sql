-- 《AI情绪治愈自习打卡空间》基础初始化数据脚本（data.sql）


-- 清理旧数据
SET FOREIGN_KEY_CHECKS = 0;

TRUNCATE TABLE `audit_logs`;
TRUNCATE TABLE `checkin_records`;
TRUNCATE TABLE `emotion_records`;
TRUNCATE TABLE `study_breaks`;
TRUNCATE TABLE `study_sessions`;
TRUNCATE TABLE `study_room_seats`;
TRUNCATE TABLE `study_rooms`;
TRUNCATE TABLE `admin_users`;
TRUNCATE TABLE `users`;

SET FOREIGN_KEY_CHECKS = 1;

-- =========================
-- 用户初始化数据
-- 密码明文：abc12345
-- =========================
INSERT INTO `users` (`id`, `nickname`, `password_hash`, `phone`, `avatar_url`, `profile`, `status`, `created_at`, `updated_at`, `streak_days`)
VALUES (1, '苹果树', '$2b$10$htchWVg0WTJ6KI4Jfg1jQu5t2H6AlUvLqWLYEgjcgBpGl18LQvM16', '13800000001', 'https://api.dicebear.com/7.x/avataaars/svg?seed=user1', '今天也是元气满满的一天！', 'active', NOW(), NOW(), 5),
        (2, '油橄榄', '$2b$10$htchWVg0WTJ6KI4Jfg1jQu5t2H6AlUvLqWLYEgjcgBpGl18LQvM16', '13800000002', 'https://api.dicebear.com/7.x/avataaars/svg?seed=user2', '期末周求放过😭', 'active', NOW(), NOW(), 2);

-- =========================
-- 管理员初始化数据
-- 密码明文：admin123
-- =========================
INSERT INTO `admin_users` ( `id`, `admin_name`, `password_hash`, `role`, `created_at`)
VALUES (1, 'admin', '$2b$10$3/6C1lgpCuNPRNcllYg1C.OhGg6y9RFqBC8LKY58zTflK4yszbpoG', 'admin', NOW());

-- =========================
-- 自习室初始化数据
-- =========================
INSERT INTO `study_rooms` (`id`, `name`, `description`, `capacity`, `is_private`, `password`, `status`, `creator_id`, `created_at`, `open_at`, `close_at`)
VALUES (1, '🌱 24H晨读室', '本房间全天候开放，欢迎早起打卡', 30, 0, NULL, 'open', 1, NOW(), NOW(), DATE_ADD(NOW(), INTERVAL 30 DAY)),
    (2, '🔒 考研冲刺终极闭关房', '本房间为私密房间', 10, 1, '888888', 'open', 1, NOW(), NOW(), DATE_ADD(NOW(), INTERVAL 7 DAY));

-- =========================
-- 自习室座位初始化数据
-- =========================
INSERT INTO `study_room_seats` (`id`, `room_id`, `seat_code`, `status`)
VALUES
(1, 1, 'A01', 'available'),
(2, 1, 'A02', 'available'),
(3, 1, 'A03', 'occupied'),
(4, 1, 'A04', 'available'),
(5, 2, 'B01', 'available'),
(6, 2, 'B02', 'available');

-- =========================
-- 自习会话初始化数据
-- 模拟用户2正在学习
-- =========================
INSERT INTO `study_sessions` (`id`, `room_id`, `user_id`, `seat_id`, `mode`, `study_content`, `start_time`, `end_time`, `duration_minutes`, `is_valid`, `status`, `last_heartbeat_at`)
VALUES (1, 1, 2, 3, 'pomodoro', '复习《数据结构》算法题', DATE_SUB(NOW(), INTERVAL 45 MINUTE), NULL, 45, 1, 'studying', NOW());

-- =========================
-- 休息记录初始化数据
-- =========================
INSERT INTO `study_breaks` (`id`, `session_id`, `start_time`, `end_time`, `duration`, `is_extended`)
VALUES (1, 1, DATE_SUB(NOW(), INTERVAL 20 MINUTE), DATE_SUB(NOW(), INTERVAL 15 MINUTE), 300, 0);

-- =========================
-- 情绪记录初始化数据
-- =========================
INSERT INTO `emotion_records` (`id`, `session_id`, `emotion_tag`, `emotion_score`, `user_note`, `ai_feedback`, `created_at`)
VALUES (1, 1, '焦虑', 8, '二叉树旋转算法太难了，看了两个小时还是不懂，快崩溃了😭🤯', '亲爱的油橄榄，抱歉听到你现在的沮丧。AVL旋转确实是很多同学的难点，请先休息一下，慢慢拆解问题，你已经坚持得很棒了❤', NOW());

-- =========================
-- 打卡记录初始化数据
-- =========================
INSERT INTO `checkin_records` (`id`, `user_id`, `checkin_date`, `emotion_record_id`, `total_minutes`, `is_makeup`, `makeup_reason`, `summary_note`, `created_at`)
VALUES (1, 1, DATE_SUB(CURDATE(), INTERVAL 1 DAY), NULL, 180, 0, NULL, '昨天完成了Rust并发编程第三章，效率很高！继续保持 🚀', DATE_SUB(NOW(), INTERVAL 1 DAY));

-- =========================
-- 管理员日志初始化数据
-- =========================
INSERT INTO `audit_logs` (`id`, `admin_id`, `action`, `target_type`, `target_id`, `reason`, `created_at`)
VALUES (1, 1, 'create_room', 'room', 1, '初始化创建公共自习室', NOW());