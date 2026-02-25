use utils_dyn::util::MATCH_EXITING;
use smash2::app::FighterManager;
use skyline::hooks::InlineCtx;
use std::sync::atomic::{Ordering, AtomicBool};

pub static IS_RULE_TIME: AtomicBool = AtomicBool::new(false);

#[repr(C)]
struct PaneResult {
    data: [u64; 4],
}

// Hide timer for 1P matches
#[skyline::hook(offset = 0x3776360)]
unsafe fn hide_timer(layout_view: u64, name: *const u8) -> PaneResult {
    let result = call_original!(layout_view, name);
    if let Some(fighter_manager) = FighterManager::instance() {
        if fighter_manager.entry_count() == 1 && result.data[1] != 0 {
            let len = skyline::libc::strlen(name);
            let name_str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(name, len));
            if name_str == "set_rep_time_pil_s" {
                let internal = *(result.data[1] as *const u64);
                if internal != 0 {
                    let parent = *((internal as *const u8).add(0x18) as *const u64);
                    if parent != 0 {
                        *(parent as *mut u8).add(0x58) &= 0xFE;
                    }
                }
            }
        }
    }
    result
}

// Set the match timer to 99 minutes every frame in 1P mode so it never expires.
#[skyline::hook(offset = 0x15812b8, inline)]
unsafe fn set_infinite_time(ctx: &mut InlineCtx) {
    if let Some(fighter_manager) = FighterManager::instance() {
        if fighter_manager.entry_count() == 1 {
            let match_state = ctx.registers[0].x() as *mut u8;
            // 99 minutes in frames = 99 * 60 * 60 = 356400
            *(match_state.add(0x3d4) as *mut i32) = 356400;
        }
    }
}

// Prevent the match state from transitioning to the ending sequence in 1P mode.
#[skyline::hook(offset = 0x1585a5c, inline)]
unsafe fn prevent_match_end_transition(ctx: &mut InlineCtx) {
    if MATCH_EXITING.load(Ordering::Relaxed){
        return;
    }

    if let Some(fighter_manager) = FighterManager::instance() {
        if fighter_manager.entry_count() == 1 {
            let base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
            let normal_update = base + 0x1581270;
            ctx.registers[8].set_x(normal_update);
        }
    }
}

// Don't play the match end sequence for one player matches
#[skyline::hook(offset = 0x1587270)]
unsafe fn bypass_match_end_sequence(param_1: u64) {
    call_original!(param_1);

    if let Some(fighter_manager) = FighterManager::instance() {
        if fighter_manager.entry_count() == 1 {
            
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
    }
}

// Don't immediately end a one player match.
// Also forces respawn at center stage when the fighter dies.
#[skyline::hook(offset = 0x14f9420)]
unsafe fn match_over_reader(param_1: u64) -> u32 {
    if MATCH_EXITING.load(Ordering::Relaxed){
        return call_original!(param_1);
    }

    if let Some(fighter_manager) = FighterManager::instance() {
        if fighter_manager.entry_count() == 1 {
            // Force respawn in stock mode if fighter is stuck in dead status
            // Not needed in timed mode because the match is technically never "over"
            if !IS_RULE_TIME.load(Ordering::Relaxed) {
                use smash::app::lua_bind::*;
                use smash::lib::lua_const::*;
                if let Some(object) = crate::util::get_battle_object_from_entry_id(0) {
                    let object = &mut *object;
                    let status = StatusModule::status_kind(object.module_accessor);
                    if status == *FIGHTER_STATUS_KIND_DEAD {
                        // This is an estimation of a good angel platform height
                        // Might not be great for certain stages, didn't test all of them
                        let pos = smash::phx::Vector3f { x: 0.0, y: 75.0, z: 0.0 };
                        PostureModule::set_pos(object.module_accessor, &pos);
                        StatusModule::change_status_force(object.module_accessor, *FIGHTER_STATUS_KIND_REBIRTH, true);
                    }
                }
            }

            return 0;
        }
    }

    return call_original!(param_1);
}

pub fn install() {
    skyline::install_hooks!(
        match_over_reader,
        bypass_match_end_sequence,
        prevent_match_end_transition,
        set_infinite_time,
        hide_timer,
    );
}