use super::*;

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] fill out the kinetic energy from parameter data and also change the motion kind
    //      depending on whether you are grounded or not.
    fighter.sub_set_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_hi"));
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_start"), L2CValue::Hash40s("special_air_hi_start"), false.into());

    // [v] set the flag that says you are grounded, which impacts other parts of the move later
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_GROUND_START);
    }

    fighter.main_shift(special_hi_main_loop)
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] update the kinetic energy from parameter data and once change the motion kind
    //      if your situation changes
    fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_hi"));
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_start"), L2CValue::Hash40s("special_air_hi_start"), true.into());

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    // [v] check if you are doing the input for spreadbullet, and if the animation is over (this is the initial swipe)
    //      transition into the jump
    // special_hi_common_check_spreadbullet(fighter);

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP.into(), false.into());
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
}
