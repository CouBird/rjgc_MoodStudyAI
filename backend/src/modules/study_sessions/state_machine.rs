use crate::constants::statuses::study_session;

pub fn can_transition(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        (study_session::STUDYING, study_session::STUDYING)
            | (study_session::PAUSED, study_session::PAUSED)
            | (study_session::STUDYING, study_session::PAUSED)
            | (study_session::PAUSED, study_session::STUDYING)
            | (study_session::STUDYING, study_session::RESTING)
            | (study_session::RESTING, study_session::STUDYING)
            | (study_session::STUDYING, study_session::ENDED)
            | (study_session::PAUSED, study_session::ENDED)
    )
}
