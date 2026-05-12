use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_HI

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int(2, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ);

    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    fighter.change_motion_by_situation("special_hi_start", "special_air_hi_start", 0.0, 1.0, false, 0.0, false, false);
    fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_AIR_HI_START);
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        let start_y_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_y_max");
        let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::FighterKineticEnergyGravity;
        smash::app::lua_bind::FighterKineticEnergyGravity::set_speed(gravity_energy, sum_speed_y.clamp(-start_y_max, start_y_max));
    }

    let special_hi_parasol_limit_time = fighter.get_param_int("param_special_hi", "special_hi_parasol_limit_time");
    fighter.set_int(special_hi_parasol_limit_time, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_PARASOL_LIMIT_TIME_COUNTER);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_PEACH_CLIFF_HANG_DATA_SPECIAL_HI as u32);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        angling(fighter);
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn angling(fighter: &mut L2CFighterCommon) -> bool {
    if fighter.is_flag(*FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS) {
        fighter.off_flag(*FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS);
        // start rise
        if fighter.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ) > 1 {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
            
            fighter.set_int(1, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ);
            return true;
        }
        // angle
        if fighter.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ) > 0 {
            // angle f8
            fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());

            let maxrot = fighter.get_param_float("param_special_hi", "special_hi_start_dir_mul");
            let speed= fighter.get_param_float("param_special_hi", "special_hi_start_trans_speed_mul");
            let facing = fighter.lr();
            let stick = Vector2f::new(
                fighter.left_stick_x(),
                fighter.left_stick_y()        
            );

            let angle_from_vertical = app::sv_math::vec2_angle(stick.x, stick.y, 0.0, 1.0).to_degrees();
            let mut angle = angle_from_vertical * stick.x.signum() * -1.0;
            angle = if facing < 0.0 {
                angle.clamp(-maxrot, maxrot)
            } else {
                angle.clamp(-maxrot, maxrot)
            };
            VarModule::set_float(fighter.battle_object, vars::peach::instance::SPECIAL_HI_ANGLE, angle);
            let angle_rad = angle.to_radians();

            sv_kinetic_energy!(set_angle, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, angle_rad);
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed);
            fighter.set_int(0, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_ENABLE_UNIQ);
            return true;
        }
    }
    return false
}

// FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END

unsafe extern "C" fn special_hi_air_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StopModule::is_stop(fighter.module_accessor) {
        special_hi_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_hi_substatus as *const () as _));
    smashline::original_status(Main, fighter, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END)(fighter)
}

unsafe extern "C" fn special_hi_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if fighter.status_frame() >= 1
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && param_1.get_bool() {
        fighter.sub_air_check_dive();
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);

    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END, special_hi_air_end_main);
}
