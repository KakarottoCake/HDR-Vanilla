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

    // grab clanks are universally enabled on F8 of the dash grab status
    let frame = fighter.global_table[CURRENT_FRAME].get_i32();
    if frame == 7 {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    // and are disabled when the grab ends (but not later than F20)
    if GrabModule::is_rebound(fighter.module_accessor) 
    && (frame >= 19 || fighter.is_flag(*FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)) {
        GrabModule::set_rebound(fighter.module_accessor, false);
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