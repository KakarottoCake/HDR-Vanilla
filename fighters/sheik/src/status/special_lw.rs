use super::*;

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);

    // Allow multiple Bouncing Fishes per airtime
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);

    ret
}

unsafe extern "C" fn special_lw_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_SHEIK_SPECIAL_LW_ATTACK,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_lw_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.object(), vars::sheik::instance::SPECIAL_LW_HIT); // clear hit-status every new attack
    fighter.off_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_LW_FLAG_TOUCH_WALL);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack_return"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.main_shift(special_lw_attack_main_loop)
}

unsafe extern "C" fn special_lw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    // disabled cancelling out of the anim (see below if re-adding)
    if !StatusModule::is_changing(fighter.module_accessor) {
        // return-to-idle frame LC
        if fighter.motion_frame() > 23.0 {
            let land_cancel_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.land_cancel_frame"); // matched to bounce landing lag
            if fighter.check_land_cancel(Some(land_cancel_frame)) {
                return 1.into()
            }
        }
        // special landing anim
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_LANDING.into(), false.into())
        }
        // wall bounce
        if !fighter.is_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_LW_FLAG_RETURN) {
            let attack_wall_check_end_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_lw.attack_wall_check_end_frame"); // return to idle start frame
            if fighter.status_frame() + 1 <= attack_wall_check_end_frame {
                wall_bounce(fighter);
            }
        }
        // don't bounce on shield
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.object(), vars::sheik::instance::SPECIAL_LW_HIT);
        }
        // slow on shield
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            let control_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) as *mut smash::app::KineticEnergy;
            let attack_guard_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.attack_guard_mul");
            smash::app::lua_bind::KineticEnergy::mul_speed(control_energy, &Vector3f::new(attack_guard_mul, 1.0, 1.0)); 
        }
        // bounce
        if VarModule::is_flag(fighter.object(), vars::sheik::instance::SPECIAL_LW_HIT)
        && !StopModule::is_stop(fighter.module_accessor) {
            fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN.into(), false.into())
        }
        // ending
        if MotionModule::is_end(fighter.module_accessor) { // ADD CHECK for above flag IF KEEPING SPECIAL FALL
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_return_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_SHEIK_SPECIAL_LW_RETURN,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_lw_return_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_LW_FLAG_TOUCH_WALL);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_return"), 0.0, 1.0, false, 0.0, false, false);
    fighter.main_shift(special_lw_return_main_loop)
}

unsafe extern "C" fn special_lw_return_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    let frame = fighter.status_frame() + 1;
    let somersault_cancel_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_lw.somersault_cancel_frame");
    let somersault_wall_cancel_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_lw.somersault_wall_cancel_frame");
    // cancel _ frame of bounce
    if frame >= somersault_wall_cancel_frame
    || (frame >= somersault_cancel_frame && VarModule::is_flag(fighter.object(), vars::sheik::instance::SPECIAL_LW_HIT)) {
        fighter.check_jump_cancel(false, false);
        fighter.check_airdodge_cancel();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        // return-to-idle frame LC
        if fighter.motion_frame() > 66.0 {
            let land_cancel_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.land_cancel_frame"); // matched to bounce landing lag
            if fighter.check_land_cancel(Some(land_cancel_frame)) {
                return 1.into()
            }
        }
        // special landing anim
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_LANDING.into(), false.into())
        }
        // wall bounce
        let somersault_wall_check_start_frame = fighter.get_param_float("param_special_lw", "somersault_wall_check_start_frame") as i32;
        let somersault_wall_check_end_frame = fighter.get_param_float("param_special_lw", "somersault_wall_check_end_frame") as i32;
        if frame >= somersault_wall_check_start_frame 
        && frame <= somersault_wall_check_end_frame {
            wall_bounce(fighter);
        }
        // attack check
        let somersault_attack_frame = fighter.get_param_int("param_special_lw", "somersault_attack_frame");
        let max_count = fighter.get_param_int("param_special_lw", "max_count");
        let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
        if frame >= somersault_attack_frame
        && fighter.get_int(*FIGHTER_SHEIK_STATUS_SPECIAL_LW_WORK_INT_RETURN_TO_ATTACK_NUM) < max_count 
        && (pad_flag & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0
        || pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0) {
            fighter.inc_int(*FIGHTER_SHEIK_STATUS_SPECIAL_LW_WORK_INT_RETURN_TO_ATTACK_NUM);
            fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into())
        }
        // ending
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
        }
    }
    0.into()
}

unsafe extern "C" fn wall_bounce(fighter: &mut L2CFighterCommon) {
    let mut touch_wall = false;
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
    } else {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
    }
    fighter.on_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_LW_FLAG_TOUCH_WALL);
    if touch_wall {fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN.into(), false.into()); }
}

pub fn install(agent: &mut Agent) {
    //agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);

    agent.status(Pre, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK, special_lw_attack_pre);
    agent.status(Main, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK, special_lw_attack_main);

    agent.status(Pre, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN, special_lw_return_pre);
    agent.status(Main, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN, special_lw_return_main);
}