use super::*;
use globals::*;

// This file contains code for dash grab

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_CatchTurn_Main
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CatchTurn_Main)]
unsafe fn status_CatchTurn_Main(fighter: &mut L2CFighterCommon) -> L2CValue {

    // grab clanks are universally enabled on F9 of the pivot grab status
    let frame = fighter.global_table[CURRENT_FRAME].get_i32();
    if frame == 8 {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    // and are disabled when the grab ends (but not later than F21)
    if GrabModule::is_rebound(fighter.module_accessor) 
    && (frame >= 20 || fighter.is_flag(*FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)) {
        GrabModule::set_rebound(fighter.module_accessor, false);
    }

    call_original!(fighter)
}
