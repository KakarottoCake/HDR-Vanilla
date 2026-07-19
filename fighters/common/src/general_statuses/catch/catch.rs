use super::*;
use globals::*;

// This file contains code for dash grab

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_Catch_Main
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Catch_Main)]
unsafe fn status_Catch_Main(fighter: &mut L2CFighterCommon) -> L2CValue {

    if ParamModule::has_param_module(fighter.battle_object) {
        let frame = fighter.global_table[CURRENT_FRAME].get_i32();
        // grab clanks are universally enabled on start_frame
        let start_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "grab_rebound.catch_start_frame");
        if frame == start_frame {
            GrabModule::set_rebound(fighter.module_accessor, true);
        }
        // and are disabled when the grab ends (but not later than end_frame)
        let end_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "grab_rebound.catch_end_frame");
        if GrabModule::is_rebound(fighter.module_accessor) 
        && (frame >= end_frame || fighter.is_flag(*FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)) {
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
    }

    call_original!(fighter)
}
