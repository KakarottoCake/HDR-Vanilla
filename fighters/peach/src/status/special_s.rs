use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
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

    return 0.into();
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_start_momentum(fighter);
    fighter.set_int(1, *FIGHTER_PEACH_STATUS_SPECIAL_S_WORK_INT_ENABLE_UNIQ);

    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        //special_s_check_flick(fighter);
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP.into(), false.into());
    }

    return 0.into();
}

unsafe extern "C" fn special_s_start_momentum(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_y = fighter.get_param_float("param_special_s", "special_s_start_speed_y");
    let stable_y = fighter.get_param_float("param_special_s", "special_s_jump_stable_y");
    let max_y = fighter.get_param_float("air_speed_y_stable", "");
    fighter.change_motion_by_situation("special_s_start", "special_air_s_start", 0.0, 1.0, false, 0.0, false, false);
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, stable_y);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, (speed_y+start_y).clamp(-max_y, max_y));
    0.into()
}

unsafe extern "C" fn special_s_check_flick(fighter: &mut L2CFighterCommon) -> L2CValue {
    let buffer = ControlModule::get_command_life_count_max(fighter.module_accessor) as usize;
    let hold_frames = InputModule::get_trigger_count(fighter.battle_object, Buttons::Special);
    if fighter.is_button_on(Buttons::Special) && hold_frames > buffer {
        fighter.on_flag(*FIGHTER_PEACH_STATUS_SPECIAL_S_FLAG_FLICK_START);
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x20cbc92683));
        fighter.push_lua_stack(&mut L2CValue::I32(1));
        fighter.push_lua_stack(&mut L2CValue::I32(*FIGHTER_LOG_DATA_INT_HAJIKI_NUM));
        app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.set_int(0, *FIGHTER_PEACH_STATUS_SPECIAL_S_WORK_INT_ENABLE_UNIQ);
    }
    0.into()
}

unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());
    0.into()
}


// FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP

unsafe extern "C" fn special_s_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::peach::instance::DISABLE_SPECIAL_S);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_jump"), 0.0, 1.0, false, 0.0, false, false);

    fighter.main_shift(special_s_jump_main_loop)
}

unsafe extern "C" fn special_s_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_s_jump_momentum(fighter);
    wall_check(fighter);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) || fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END.into(), false.into());
        return 1.into();
    }
    
    return 0.into();
}

unsafe extern "C" fn special_s_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[globals::STATUS_KIND] == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END {
        special_s_end_momentum(fighter);
    }
    0.into()
}

// rapidly decaying speed, drift to change distance
unsafe extern "C" fn special_s_jump_momentum(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = fighter.lr();
    let start_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_start");
    let min_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_min");
    let brake_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_brake_x");
    let accel_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_accel_x");
    let start_y = fighter.get_param_float("param_special_s", "special_s_jump_speed_y");
    let stable_y = fighter.get_param_float("param_special_s", "special_s_jump_stable_y");
    let max_y = fighter.get_param_float("air_speed_y_stable", "");
    if StatusModule::is_changing(fighter.module_accessor) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_x, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_x, 0.0);
        sv_kinetic_energy!(set_accel_x_add, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0);
        sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_x*lr, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, stable_y);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, (speed_y+start_y).clamp(-stable_y, stable_y));
        return 1.into();
    }
    // speed cannot go below minimum, cannot exceed starting value due to higher brake value
    let add_speed = (fighter.left_stick_x() * lr * accel_x) - brake_x;
    let mut new_speed = (add_speed*lr) + speed_x;
    new_speed = if speed_x > 0.0 {new_speed.clamp(min_x, start_x)} else {new_speed.clamp(-start_x, -min_x)};
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed, 0.0);
    0.into()
}


// prevent excessive speed transfer?
unsafe extern "C" fn special_s_end_momentum(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = fighter.lr();
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let min_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_min");
    let jump_end = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.jump_end");
    let new_speed = if speed_x > 0.0 {speed_x.clamp(min_x, jump_end)} else {speed_x.clamp(-jump_end, -min_x)};
    let control = if fighter.is_situation(*SITUATION_KIND_GROUND) {FIGHTER_KINETIC_ENERGY_ID_STOP} else {FIGHTER_KINETIC_ENERGY_ID_CONTROL};
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, new_speed, 0.0);
    0.into()
}

// wall bounce
unsafe extern "C" fn wall_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut touch_wall = false;
    if fighter.lr() > 0.0 {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
    } else {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
    }
    if touch_wall {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END.into(), true.into());
    }
    0.into()
}

// FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END

unsafe extern "C" fn special_s_away_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_S_BRAKE,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);

    agent.status(Pre, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_pre);
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_main);
    agent.status(End, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_end);

    agent.status(Pre, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END, special_s_away_end_pre);
}
