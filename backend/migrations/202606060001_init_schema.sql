CREATE TABLE IF NOT EXISTS `users` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `nickname` VARCHAR(20) NOT NULL COMMENT '用户昵称，最长20个字符',
    `password_hash` VARCHAR(255) NOT NULL,
    `phone` VARCHAR(20) NOT NULL UNIQUE COMMENT '手机号，全局唯一',
    `avatar_url` VARCHAR(255) DEFAULT 'default.png',
    `profile` VARCHAR(255) COMMENT '个人简介',
    `status` VARCHAR(20) NOT NULL DEFAULT 'active',
    `created_at` TIMESTAMP NOT NULL,
    `updated_at` TIMESTAMP NOT NULL,
    `streak_days` INT NOT NULL DEFAULT 0,
    PRIMARY KEY(`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `study_rooms` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(100) NOT NULL UNIQUE COMMENT '房间名称，全局唯一',
    `description` TEXT,
    `capacity` INTEGER NOT NULL DEFAULT 50 COMMENT '房间容量，范围1-50',
    `is_private` TINYINT(1) NOT NULL DEFAULT 0,
    `password` VARCHAR(64),
    `status` VARCHAR(20) NOT NULL DEFAULT 'open' COMMENT 'open 或 closed',
    `creator_id` BIGINT NOT NULL,
    `created_at` TIMESTAMP NOT NULL,
    `open_at` TIMESTAMP NOT NULL,
    `close_at` TIMESTAMP NOT NULL,
    PRIMARY KEY(`id`),
    CHECK (`capacity` BETWEEN 1 AND 50),
    CHECK (status IN ('open','closed'))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `study_room_seats` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `room_id` BIGINT NOT NULL,
    `seat_code` VARCHAR(20) NOT NULL,
    `status` VARCHAR(20) NOT NULL DEFAULT 'available',
    PRIMARY KEY(`id`),
    UNIQUE KEY `uk_room_seat` (`room_id`, `seat_code`),
    CHECK (status IN ('available', 'occupied'))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `study_sessions` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `room_id` BIGINT NOT NULL,
    `user_id` BIGINT NOT NULL,
    `seat_id` BIGINT NOT NULL COMMENT '座位ID',
    `mode` VARCHAR(20) NOT NULL DEFAULT 'normal' COMMENT 'normal正向计时，pomodoro番茄钟',
    `study_content` VARCHAR(255) COMMENT '学习内容',
    `start_time` TIMESTAMP NOT NULL,
    `end_time` TIMESTAMP,
    `duration_minutes` INT NOT NULL DEFAULT 0,
    `is_valid` TINYINT(1) NOT NULL DEFAULT 1,
    `status` VARCHAR(20) NOT NULL DEFAULT 'studying' COMMENT 'studying, paused, resting, ended',
    `last_heartbeat_at` TIMESTAMP NULL,
    PRIMARY KEY(`id`),
    CHECK (status IN (
        'studying',
        'paused',
        'resting',
        'ended'
        )),
    CHECK (mode IN ('normal','pomodoro'))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `study_breaks` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `session_id` BIGINT NOT NULL,
    `start_time` TIMESTAMP NOT NULL,
    `end_time` TIMESTAMP,
    `duration` INTEGER NOT NULL DEFAULT 0,
    `is_extended` TINYINT(1) NOT NULL DEFAULT 0,
    PRIMARY KEY(`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `emotion_records` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `session_id` BIGINT NOT NULL,
    `emotion_tag` VARCHAR(30) NOT NULL COMMENT '情绪标签',
    `emotion_score` INTEGER NOT NULL COMMENT '情绪分数',
    `user_note` TEXT COMMENT '情绪备注',
    `ai_feedback` TEXT COMMENT 'AI情绪治愈反馈',
    `created_at` TIMESTAMP NOT NULL,
    PRIMARY KEY(`id`),
    CHECK (`emotion_score` BETWEEN 1 AND 10)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `checkin_records` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `user_id` BIGINT NOT NULL,
    `checkin_date` DATE NOT NULL,
    `emotion_record_id` BIGINT NULL,
    `total_minutes` INT NOT NULL DEFAULT 0,
    `is_makeup` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否为补卡',
    `makeup_reason` VARCHAR(255) NULL,
    `summary_note` TEXT,
    `created_at` TIMESTAMP NOT NULL,
    PRIMARY KEY(`id`),
    UNIQUE KEY `uk_user_date` (`user_id`, `checkin_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `admin_users` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `admin_name` VARCHAR(50) NOT NULL UNIQUE,
    `password_hash` VARCHAR(255) NOT NULL,
    `role` VARCHAR(20) NOT NULL DEFAULT 'admin',
    `created_at` TIMESTAMP NOT NULL,
    PRIMARY KEY(`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `audit_logs` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `admin_id` BIGINT NOT NULL,
    `action` VARCHAR(50) NOT NULL COMMENT '操作类型',
    `target_type` VARCHAR(30) NOT NULL,
    `target_id` BIGINT NOT NULL,
    `reason` TEXT COMMENT '操作原因',
    `created_at` TIMESTAMP NOT NULL,
    PRIMARY KEY(`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- 外键约束维护
ALTER TABLE `study_rooms` ADD CONSTRAINT `fk_rooms_creator` FOREIGN KEY(`creator_id`) REFERENCES `users`(`id`) ON DELETE NO ACTION;
ALTER TABLE `study_room_seats` ADD CONSTRAINT `fk_seats_room` FOREIGN KEY(`room_id`) REFERENCES `study_rooms`(`id`) ON DELETE NO ACTION;
ALTER TABLE `study_sessions` ADD CONSTRAINT `fk_sessions_user` FOREIGN KEY(`user_id`) REFERENCES `users`(`id`) ON DELETE NO ACTION;
ALTER TABLE `study_sessions` ADD CONSTRAINT `fk_sessions_room` FOREIGN KEY(`room_id`) REFERENCES `study_rooms`(`id`) ON DELETE NO ACTION;
ALTER TABLE `study_sessions` ADD CONSTRAINT `fk_sessions_seat` FOREIGN KEY(`seat_id`) REFERENCES `study_room_seats`(`id`) ON DELETE NO ACTION;
ALTER TABLE `study_breaks` ADD CONSTRAINT `fk_breaks_session` FOREIGN KEY(`session_id`) REFERENCES `study_sessions`(`id`) ON DELETE CASCADE;
ALTER TABLE `emotion_records` ADD CONSTRAINT `fk_emotions_session` FOREIGN KEY(`session_id`) REFERENCES `study_sessions`(`id`) ON DELETE CASCADE;
ALTER TABLE `checkin_records` ADD CONSTRAINT `fk_checkins_user` FOREIGN KEY(`user_id`) REFERENCES `users`(`id`) ON DELETE NO ACTION;
ALTER TABLE `checkin_records` ADD CONSTRAINT `fk_checkins_emotion` FOREIGN KEY(`emotion_record_id`) REFERENCES `emotion_records`(`id`) ON DELETE SET NULL;
ALTER TABLE `audit_logs` ADD CONSTRAINT `fk_logs_admin` FOREIGN KEY(`admin_id`) REFERENCES `admin_users`(`id`) ON DELETE NO ACTION;

-- 索引
CREATE INDEX idx_session_start_time ON study_sessions(user_id, start_time);
CREATE INDEX idx_user_status ON study_sessions(user_id, status);
CREATE INDEX idx_room_status ON study_sessions(room_id, status);
CREATE INDEX idx_emotion_session ON emotion_records(session_id);