
use std::{collections::HashSet, sync::{LazyLock, Mutex}};

pub static STAGE_MANAGER: LazyLock<Mutex<StageManager>> = LazyLock::new(||{Mutex::new(StageManager::new())});

pub struct StageManager {
    pub selected_panel: Option<i32>,
    pub selected_preview: Option<i32>,
    pub is_my_music: Option<bool>,
    pub perma_striked_stages: HashSet<i32>
}

impl StageManager{
    pub fn new() -> Self {
        Self {
            selected_panel: None,
            selected_preview: None,
            is_my_music: None,
            perma_striked_stages: HashSet::new(),
        }
    }
}