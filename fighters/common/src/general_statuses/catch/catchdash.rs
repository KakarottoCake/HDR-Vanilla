use super::*;
use globals::*;

// This file contains code for dash grab

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_CatchDash_common,
            status_CatchDash_Main,
            status_end_CatchDash,
            bind_address_call_status_end_CatchDash
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_CatchDash_common)]
unsafe fn status_pre_CatchDash_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    JostleModule::set_overlap_rate_mul(fighter.module_accessor, 5.0);  // 0.3 (base overlap rate) * 5.0 = 1.5 overlap rate
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CatchDash_Main)]
unsafe fn status_CatchDash_Main(fighter: &mut L2CFighterCommon) -> L2CValue {

    if ParamModule::has_param_module(fighter.battle_object) {
        let frame = fighter.global_table[CURRENT_FRAME].get_i32();
        // grab clanks are universally enabled on start_frame
        let start_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "grab_rebound.catchdash_start_frame");
        if frame == start_frame {
            GrabModule::set_rebound(fighter.module_accessor, true);
        }
        // and are disabled when the grab ends (but not later than end_frame)
        let end_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "grab_rebound.catchdash_end_frame");
        if GrabModule::is_rebound(fighter.module_accessor) 
        && (frame >= end_frame || fighter.is_flag(*FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)) {
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
    }

    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_CatchDash)]
unsafe fn status_end_CatchDash(fighter: &mut L2CFighterCommon) -> L2CValue {
    JostleModule::set_overlap_rate_mul(fighter.module_accessor, 1.0);  // reset to 0.3 overlap rate
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_bind_address_call_status_end_CatchDash)]
unsafe fn bind_address_call_status_end_CatchDash(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_CatchDash();
    0.into()
}