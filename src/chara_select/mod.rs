use std::{
    collections::HashMap,
    fs::*,
    path::{ Path, PathBuf },
    sync::{ LazyLock, RwLock }
};
use skyline::hooks::InlineCtx;
use smash2::{
    cpp::Vector,
    phx::hash40
};
use serde::Deserialize;
use utils::modules::TourneyConfig;

mod layout;
mod random;
mod player_port;

pub const KEY_MASK: u64 = 0xFFFFFF_0000000000;

pub static mut CHARA_DATA: LazyLock<RwLock<CharaData>> = LazyLock::new(|| 
    RwLock::new(CharaData::default())
);

#[derive(Debug, Clone)]
pub struct CharaData {
    main_id: u64,
    sub_id: u64,
    last_selection: String,
    melee_random: bool,
    costume: u8,
    costume_rng: Vec<i32>,

    whitelist: Vec<String>,
    blacklist: Vec<String>
}
impl Default for CharaData {
    fn default() -> Self {
        CharaData {
            main_id: 0x0,
            sub_id: 0x0,
            last_selection: String::new(),
            melee_random: false,
            costume: 0,
            costume_rng: (0..8).collect::<Vec<i32>>(),

            whitelist: Vec::new(),
            blacklist: Vec::new()
        }
    }
}

pub fn is_tourney_mode() -> bool {
    match TourneyConfig::load() {
        Some(config) => config.enabled,
        None => false
    }
}

pub static mut PLAYER_TAG_INDEX: &'static mut [u8] = &mut [0; 8];

// big thanks to azel-s for this tag parsing code ^^
pub unsafe fn get_tag_from_save(idx: u8) -> String {
    let tag_address =
        (***(((*((*((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8)
            .add(0x5314510) as *const u64)) as *const u64))
            + 0x58) as *const *const *const u64)
            + ((idx as u64) * 0xF7D8)
            + 0xC) as *const u16;

    let mut tag_length = 0;
    while *tag_address.add(tag_length) != 0 {
        tag_length += 1;
    }

    String::from_utf16_lossy(std::slice::from_raw_parts(tag_address, tag_length))
}

#[skyline::hook(offset = 0x19fd0b0)]
unsafe fn update_player_tag(arg1: u64, tag_index: *const u8) {
    let player_id = *((arg1 + 0x1d4) as *const u8) as usize;
    if (0..8).contains(&player_id) {
        PLAYER_TAG_INDEX[player_id] = *tag_index;
    }
    call_original!(arg1, tag_index);
}

pub fn install() {
    skyline::install_hooks!(
        update_player_tag
    );

    layout::install();
    random::install();
    player_port::install();
}