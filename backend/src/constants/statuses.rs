pub mod user {
    pub const ACTIVE: &str = "active";
    pub const DISABLED: &str = "disabled";
}

pub mod room {
    pub const OPEN: &str = "open";
    pub const CLOSED: &str = "closed";
}

pub mod seat {
    pub const AVAILABLE: &str = "available";
    pub const OCCUPIED: &str = "occupied";
}

pub mod study_session {
    pub const STUDYING: &str = "studying";
    pub const PAUSED: &str = "paused";
    pub const RESTING: &str = "resting";
    pub const ENDED: &str = "ended";
}
