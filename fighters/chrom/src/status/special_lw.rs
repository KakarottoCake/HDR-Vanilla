use super::*;

pub unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return false.into();
}

pub unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    special_lw_set_kinetic(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return true.into();
        }
        special_lw_set_kinetic(fighter);
        return false.into();
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) 
        && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return false.into();
        }
        if fighter.is_situation(*SITUATION_KIND_AIR) 
        && fighter.sub_air_check_fall_common().get_bool() {
            return false.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return true.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return true.into();
        }
    }

    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        return true.into();
    }

    return false.into();
}

pub unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

unsafe extern "C" fn special_lw_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if fighter.is_flag(*FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion_inherit_frame_keep_rate(
                fighter.module_accessor,
                Hash40::new("special_air_lw"),
                -1.0,
                1.0,
                0.0
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_lw"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            fighter.on_flag(*FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if fighter.is_flag(*FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion_inherit_frame_keep_rate(
                fighter.module_accessor,
                Hash40::new("special_lw"),
                -1.0,
                1.0,
                0.0
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_lw"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            fighter.on_flag(*FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);
}