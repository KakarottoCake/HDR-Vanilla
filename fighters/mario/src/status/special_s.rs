use super::*;

unsafe extern "C" fn special_s_air_kinetic_helper(fighter: &mut L2CFighterCommon) {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_s_start_mul_spd_x = fighter.get_param_float("param_special_s", "special_s_start_mul_spd_x");
    let special_s_start_air_acl_x = fighter.get_param_float("param_special_s", "special_s_start_air_acl_x");
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x / special_s_start_mul_spd_x, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, special_s_start_air_acl_x, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
}

unsafe extern "C" fn special_s_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
        if fighter.is_flag(*FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
            special_s_air_kinetic_helper(fighter);
        } else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
            fighter.on_flag(*FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE);
        }
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        if fighter.is_flag(*FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
        } else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
            fighter.on_flag(*FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE);
        }
    }
}

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    return false.into();
}

unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if [
        Hash40::new("special_s")
    ].contains(&fighter.get_motion_kind().get_hash()) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_HOP);
    } else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_HOP);
    }
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        special_s_air_kinetic_helper(fighter);
    }
    return false.into();
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    VarModule::on_flag(fighter.battle_object, vars::mario::instance::SPECIAL_S_DISABLE);
    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        special_s_set_kinetic(fighter);
    }
    
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return true.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND 
        && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into();
        }
        if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND 
        && fighter.sub_air_check_fall_common().get_bool() {
            return true.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    return false.into();
}

unsafe extern "C" fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_situation(*SITUATION_KIND_AIR) {
        return false.into();
    }

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    if !fighter.is_flag(*FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL) {
        let air_accel_y = fighter.get_param_float("air_accel_y", "");
        let air_speed_y_stable = fighter.get_param_float("air_speed_y_stable", "");
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
        return false.into();
    }

    if !fighter.is_flag(*FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_HOP) {
        fighter.on_flag(*FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_HOP);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let mut speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        if fighter.is_flag(*FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP) {
            speed_y = 0.0;
        } else {
            fighter.on_flag(*FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
            let special_s_attack_spd_y = fighter.get_param_float("param_special_s", "special_s_attack_spd_y");
            speed_y = special_s_attack_spd_y;
        }
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
    }

    let special_s_attack_acl_y = fighter.get_param_float("param_special_s", "special_s_attack_acl_y");
    let special_s_attack_max_y = fighter.get_param_float("param_special_s", "special_s_attack_max_y");
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_s_attack_acl_y);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_s_attack_max_y);
    return false.into();
}

unsafe extern "C" fn special_s_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into()
}

unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exec);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exit);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);
}