-- Indexes for the learning statistics page.

CREATE INDEX `idx_sessions_stats_user_end`
    ON `study_sessions` (`user_id`, `end_time`, `status`, `is_valid`);

CREATE INDEX `idx_emotions_stats_created_session`
    ON `emotion_records` (`created_at`, `session_id`);
