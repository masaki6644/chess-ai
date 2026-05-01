pub enum TraceEvent {
    // ===== Game =====
    GameSeen,
    GameAccepted,
    GameFiltered,

    // ===== Pipeline =====
    Expanded { positions: usize },
    Scored { positions: usize },
    Selected { positions: usize },

    // ===== Error =====
    Error { stage: &'static str },
}