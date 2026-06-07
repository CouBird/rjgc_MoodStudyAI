-- Business constraints derived from project documents.

ALTER TABLE `users`
    ADD CONSTRAINT `chk_users_status`
    CHECK (`status` IN ('active', 'disabled'));

ALTER TABLE `emotion_records`
    ADD CONSTRAINT `uk_emotion_session`
    UNIQUE (`session_id`);

CREATE INDEX `idx_rooms_status_close_at`
    ON `study_rooms` (`status`, `close_at`);

CREATE INDEX `idx_checkins_user_date`
    ON `checkin_records` (`user_id`, `checkin_date`);
