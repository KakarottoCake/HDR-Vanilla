use super::*;

pub unsafe extern "C" fn special_lw3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn special_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    special_lw3_change_motion(fighter);

    fighter.main_shift(special_lw3_main_loop)
}

unsafe fn special_lw3_change_motion(fighter: &mut L2CFighterCommon) {
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {
        match VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE) {
            0 => {
                Hash40::new("special_lw3_1g")
            },
            1 => {
                Hash40::new("special_lw3_2g")
            },
            _ => {
                Hash40::new("special_lw3_3g")
            }
        }
    }
    else {
        match VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE) {
            0 => {
                Hash40::new("special_lw3_1a")
            },
            1 => {
                Hash40::new("special_lw3_2a")
            },
            _ => {
                Hash40::new("special_lw3_3a")
            }
        }
    };
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
}

unsafe extern "C" fn special_lw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW3_ENABLE_LANDING) {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            }
            else {
                fighter.set_float(14.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME); // parameterize
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    if VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW3_ENABLE_BOUNCE) {
        VarModule::off_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW3_ENABLE_BOUNCE);
        if !VarModule::is_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STALL) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
                // slightly different physics on hit to improve feel
                let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                KineticModule::clear_speed_all(fighter.module_accessor);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed_x);
                KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(-0.3, 1.5, 0.0));
            }
            else {
                VarModule::on_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STALL);
                let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                let mut sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                sum_speed_y = sum_speed_y.clamp(-2.0, 0.5);
                KineticModule::clear_speed_all(fighter.module_accessor);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, sum_speed_y);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed_x);
                KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(-0.3, 1.5, 0.0));
            }
        }
    }
    if !VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW3_INC_STAGE)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW3_INC_STAGE);
        special_lw3_change_stage(fighter, VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE));
    }

    return 0.into();
}

unsafe fn special_lw3_change_stage(fighter: &mut L2CFighterCommon, stage: i32) {
    match stage {
        0 => {
            VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_TIMER, 480);  // parameterize
            app::FighterUtil::flash_eye_info(fighter.module_accessor);
            let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_steam1"), Hash40::new("head"), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
            VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_1, handle as i32);
            VarModule::inc_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE);
        },
        1 => {
            VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_TIMER, 480);  // parameterize
            app::FighterUtil::flash_eye_info(fighter.module_accessor);
            let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_steam2"), Hash40::new("head"), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
            VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_2, handle as i32);
            VarModule::inc_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE);
        }
        _ => {
            VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE, 0);
            let handle = VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_1) as u32;
            EffectModule::kill(fighter.module_accessor, handle, false, false);
            let handle2 = VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_2) as u32;
            EffectModule::kill(fighter.module_accessor, handle2, false, false);
            VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_1, -1);
            VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_2, -1);
            if fighter.is_motion(Hash40::new("special_lw3_3g"))
            && VarModule::is_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STALL)
            && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            && !VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW3_CLEAR_CRIT) {
                VarModule::on_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW3_CLEAR_CRIT);
                SlowModule::set_whole(fighter.module_accessor, 4, 1);
                EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), false, true, true);
            }
        }
    }
}

pub unsafe extern "C" fn special_lw3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    SlowModule::clear_whole(fighter.module_accessor);
    CameraModule::reset_all(fighter.module_accessor);
    EffectModule::remove_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), 0);
    return 0.into();
}