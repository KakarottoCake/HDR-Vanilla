use super::*;

unsafe extern "C" fn mario_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        StatusModule::set_status_kind_interrupt(
            fighter.module_accessor,
            statuses::mario::GROUND_POUND_START,
        );
        return true.into();
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return false.into();
}

unsafe extern "C" fn mario_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    let motion_hash = Hash40::new("special_lw_start");
    let jump_squat_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "long_jump.jump_squat_frame") as f32;
    let start_frame = 0.0;
    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, motion_hash);
    let mut motion_rate = end_frame / jump_squat_frame;
    if motion_rate < 1.0 {
        motion_rate += 0.001;
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        motion_hash,
        start_frame,
        motion_rate,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT.into(), true.into());
        return false.into();
    }
    return false.into();
}

unsafe extern "C" fn mario_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

unsafe extern "C" fn mario_special_lw_shoot_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return false.into();
}

unsafe extern "C" fn mario_special_lw_shoot_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * lr;
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "long_jump.speed_x_mul");
    let speed_x_min = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "long_jump.speed_x_min");
    let speed_x_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "long_jump.speed_x_max");
    let speed_y_min = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "long_jump.speed_y_min");
    let speed_y_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "long_jump.speed_y_max");
    let back_speed_x_threshold = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "long_jump.back_speed_x_threshold");
    let back_speed_x_min = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "long_jump.back_speed_x_min");
    let back_speed_x_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "long_jump.back_speed_x_max");
    if speed_x <= back_speed_x_threshold {
        if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            speed_x = f32::clamp(speed_x * speed_x_mul, back_speed_x_min, back_speed_x_max);
            speed_y = 0.0;
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x * lr, speed_y);
            VarModule::set_int(fighter.battle_object, vars::mario::status::SPECIAL_LW_LONG_JUMP_KIND, vars::mario::LONG_JUMP_B);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            speed_x = speed_x_min;
            speed_y = speed_y_min;
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x * lr);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
            VarModule::set_int(fighter.battle_object, vars::mario::status::SPECIAL_LW_LONG_JUMP_KIND, vars::mario::LONG_JUMP_W);
        }
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        speed_x = f32::clamp(speed_x * speed_x_mul, speed_x_min, speed_x_max);
        speed_y = util::nlerp(speed_y_min, speed_y_max, 1.0, (speed_x - speed_x_min) / (speed_x_max - speed_x_min));
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x_max, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x * lr);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        let long_jump_kind = if (speed_x >= speed_x_max) { vars::mario::LONG_JUMP_S } else { vars::mario::LONG_JUMP_M };
        VarModule::set_int(fighter.battle_object, vars::mario::status::SPECIAL_LW_LONG_JUMP_KIND, long_jump_kind);
    }
    return false.into();
}

unsafe extern "C" fn mario_special_lw_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_lw_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::mario::status::SPECIAL_LW_ENABLE_LANDING)
    && !fighter.sub_air_check_fall_common().get_bool() {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
        }
    }
    return false.into();
}

unsafe extern "C" fn mario_special_lw_shoot_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let air_accel_mul = if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
        1.0
    }
    else {
        ParamModule::get_float(fighter.battle_object, ParamType::Agent, "long_jump.air_accel_y_mul")
    };
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0) * air_accel_mul;
    let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -air_accel_y
    );
    let long_jump_air_accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "long_jump.air_accel_x_mul");
    sv_kinetic_energy!(
        set_accel_x_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_accel_x * long_jump_air_accel_x_mul
    );
    return false.into();
}

unsafe extern "C" fn mario_special_lw_shoot_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

unsafe extern "C" fn mario_special_lw_charge_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return false.into();
}

unsafe extern "C" fn mario_special_lw_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_landing"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_charge_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_lw_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, true);
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    return false.into();
}

unsafe extern "C" fn mario_special_lw_charge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_special_lw_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_special_lw_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_special_lw_end);

    agent.status(Pre, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_pre);
    agent.status(Init, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_init);
    agent.status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_main);
    agent.status(Exec, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_exec);
    agent.status(End, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_end);

    agent.status(Pre, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, mario_special_lw_charge_pre);
    agent.status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, mario_special_lw_charge_main);
    agent.status(End, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, mario_special_lw_charge_end);
}