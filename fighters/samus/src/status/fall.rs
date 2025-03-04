use super::*;

unsafe extern "C" fn fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_AIR_LASSO_HANG, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND]) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_FALL)(fighter)
}

unsafe extern "C" fn landing_fall_special_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL)(fighter);
    if fighter.is_prev_status_one_of(&[*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A]) {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("landing_heavy"), -1.0, 1.0, 0.0);
    }

    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_FALL, fall_main);
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, landing_fall_special_init);
}