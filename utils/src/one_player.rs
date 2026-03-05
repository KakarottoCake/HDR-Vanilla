use skyline::hooks::InlineCtx;
use smash2::app::FighterManager;
use smash::{app::{BattleObject, lua_bind::{PostureModule, StatusModule}}, lib::lua_const::{FIGHTER_STATUS_KIND_REBIRTH,FIGHTER_STATUS_KIND_DEAD}};
use std::sync::atomic::{AtomicBool, Ordering};
use utils_dyn::util::MATCH_EXITING;

pub static IS_RULE_TIME: AtomicBool = AtomicBool::new(false);
pub static SPAWN_POS_CAPTURED: AtomicBool = AtomicBool::new(false);

static mut SPAWN_POS: smash::phx::Vector3f = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

#[skyline::hook(offset = 0x1b7b814, inline)]
unsafe fn match_load(ctx: &mut InlineCtx) {
    if !one_player_entry() {
        return;
    }

    let result_ptr = ctx.registers[22].x() as *const u64;
    let pane = *result_ptr.add(1);
    if pane == 0 {
        return;
    }

    let internal = *(pane as *const u64);
    if internal == 0 {
        return;
    }

    let parent = *((internal as *const u8).add(0x18) as *const u64);
    if parent == 0 {
        return;
    }

    // hide timer
    *(parent as *mut u8).add(0x58) &= 0xFE;

    // Capture the player's spawn position while still in the loading phase
    if !SPAWN_POS_CAPTURED.load(Ordering::Relaxed) {
        use smash::app::lua_bind::*;
        if let Some(object) = crate::util::get_battle_object_from_entry_id(0) {
            let object = &*object;
            SPAWN_POS.x = PostureModule::pos_x(object.module_accessor);
            SPAWN_POS.y = PostureModule::pos_y(object.module_accessor);
            SPAWN_POS.z = PostureModule::pos_z(object.module_accessor);
            SPAWN_POS_CAPTURED.store(true, Ordering::Relaxed);
        }
    }
}

// Set the match timer to 99 minutes every frame in 1P mode so it never expires.
#[skyline::hook(offset = 0x15812b8, inline)]
unsafe fn once_per_frame(ctx: &mut InlineCtx) {
    if !one_player_entry() {
        return;
    }

    let match_state = ctx.registers[0].x() as *mut u8;
    // 99 minutes in frames
    *(match_state.add(0x3d4) as *mut i32) = 99 * 60 * 60;
}

// Prevent the match state from transitioning to the ending sequence in 1P mode.
#[skyline::hook(offset = 0x1585a5c, inline)]
unsafe fn prevent_match_end_transition(ctx: &mut InlineCtx) {
    if MATCH_EXITING.load(Ordering::Relaxed) || !one_player_entry() {
        return;
    }

    let base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
    let normal_update = base + 0x1581270;
    ctx.registers[8].set_x(normal_update);
}

// Don't play the match end sequence for one player matches
#[skyline::hook(offset = 0x1587270)]
unsafe fn bypass_match_end_sequence(param_1: u64) {
    call_original!(param_1);

    if !one_player_entry() {
        return;
    }

    let base_address = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
    let game_ending_state = base_address + 0x1587990;
    let finished_state = base_address + 0x1581260;

    let state_ptr = (param_1 + 0x150) as *mut u64;

    // If the game ending state was added to the sequence queue,
    // change it to the finished (do nothing) state
    if *state_ptr == game_ending_state {
        *state_ptr = finished_state;
    }
}

// Don't immediately end a one player match.
// Also forces respawn at center stage when the fighter dies.
#[skyline::hook(offset = 0x14f9420)]
unsafe fn match_over_reader(param_1: u64) -> u32 {
    if MATCH_EXITING.load(Ordering::Relaxed) || !one_player_entry() {
        return call_original!(param_1);
    }

    if IS_RULE_TIME.load(Ordering::Relaxed) {
        return 0;
    }

    // Force respawn in stock mode if fighter is stuck in dead status
    // Not needed in timed mode because the match is technically never "over"
    if let Some(object) = crate::util::get_battle_object_from_entry_id(0) {
        let boma = &mut *object;
        let status = StatusModule::status_kind(boma.module_accessor);
        if status == *FIGHTER_STATUS_KIND_DEAD {
            respawn_fighter(boma);
        }
    }

    return 0;
}

unsafe fn respawn_fighter(boma: &mut BattleObject) {
    let pos = if SPAWN_POS_CAPTURED.load(Ordering::Relaxed) {
        smash::phx::Vector3f {
            x: SPAWN_POS.x,
            y: SPAWN_POS.y,
            z: SPAWN_POS.z,
        }
    } else {
        smash::phx::Vector3f {
            x: 0.0,
            y: 75.0,
            z: 0.0,
        }
    };
    PostureModule::set_pos(boma.module_accessor, &pos);
    StatusModule::change_status_force(boma.module_accessor, *FIGHTER_STATUS_KIND_REBIRTH, true);
}

fn one_player_entry() -> bool {
    if let Some(fighter_manager) = FighterManager::instance() {
        return fighter_manager.entry_count() == 1;
    }
    false
}

pub fn install() {
    skyline::install_hooks!(
        match_over_reader,
        bypass_match_end_sequence,
        prevent_match_end_transition,
        once_per_frame,
        match_load,
    );
}
