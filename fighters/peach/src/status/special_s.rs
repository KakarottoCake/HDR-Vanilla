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
        special_s_check_flick(fighter);
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP.into(), false.into());
    }

    return 0.into();
}

unsafe extern "C" fn special_s_start_momentum(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_y = fighter.get_param_float("param_special_s", "special_s_start_speed_y");
    let max_y = fighter.get_param_float("air_speed_y_stable", "");
    //MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
    fighter.change_motion_by_situation("special_s_start", "special_air_s_start", 0.0, 1.0, false, 0.0, false, false);
    //fighter.set_situation(SITUATION_KIND_AIR.into());
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    // fix gr start instant landing (is_ground doesnt work?)
    //let pos = *PostureModule::pos(fighter.module_accessor);
    //let ground = &mut Vector2f{x: 0.0, y: 0.0};
    //let ray_check = GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x: pos.x, y: pos.y + 2.0}, &Vector2f{x: pos.x, y: pos.y - 2.0}, ground, true);
    //if ray_check {
    //    let ray_check_y = ground.y;
    //    PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos.x, y: ground.y + 11.0, z: pos.z});
    //    speed_y = 0.25;
    //}
    //let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::KineticEnergy;
    //lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f{x: 0.0, y: (speed_y+start_y).min(max_y)}, &Vector3f::zero(), fighter.module_accessor);

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, (speed_y+start_y).min(max_y));
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

    agent.status(Pre, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END, special_s_away_end_pre);
}
