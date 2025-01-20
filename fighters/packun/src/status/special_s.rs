use super::*;

unsafe extern "C" fn special_s_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        let motion = if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 
            { Hash40::new("special_s_shoot_s") } else { Hash40::new("special_s_shoot") };
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        CORRECT(fighter, *GROUND_CORRECT_KIND_AIR);
        let motion = if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 
            { Hash40::new("special_air_s_shoot_s") } else { Hash40::new("special_air_s_shoot") };
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.main_shift(special_s_shoot_main_loop)
}

unsafe extern "C" fn special_s_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
          return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                let motion = if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 
                    { Hash40::new("special_s_shoot_s") } else { Hash40::new("special_s_shoot") };
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC_DONE);
            }
            else {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC) {
                    // let accel_y = -1.0 * WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("accel_y"));
                    // sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, accel_y);

                    let control_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("control_accel_x"));
                    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
                    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, control_accel_x);
                    sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0);
                    KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);

                    let air_accel_y = -(WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0));
                    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                    let brake_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                    let limit_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_y_limit"));
                    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y);
                    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
                    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, brake_y);
                    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, limit_speed_y);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC_DONE);
                }
            }
            let motion = if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 
                { Hash40::new("special_air_s_shoot_s") } else { Hash40::new("special_air_s_shoot") };
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
        }
        // monch
        if //fighter.is_motion_one_of(&[Hash40::new("special_s_shoot_s"), Hash40::new("special_air_s_shoot_s")]) &&
        VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !fighter.is_in_hitlag()
            && fighter.is_situation(*SITUATION_KIND_GROUND) {
                if fighter.is_cat_flag(Cat2::AppealHi) {
                    let hash = if PostureModule::lr(fighter.module_accessor) < 0.0 { Hash40::new("appeal_hi_l") } else { Hash40::new("appeal_hi_r") };
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_APPEAL, false);
                    MotionModule::change_motion(fighter.module_accessor, hash, 0.0, -1.0, false, 0.0, false, false);
                    return 1.into();
                }
                else if fighter.is_cat_flag(Cat2::AppealSL | Cat2::AppealSR) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_APPEAL, false);
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_2"), 0.0, -1.0, false, 0.0, false, false);
                    return 1.into();
                }
                else if fighter.is_cat_flag(Cat2::AppealLw) {
                    let hash = if PostureModule::lr(fighter.module_accessor) < 0.0 { Hash40::new("appeal_lw_l") } else { Hash40::new("appeal_lw_r") };
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_APPEAL, false);
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_l"), 0.0, -1.0, false, 0.0, false, false);
                    return 1.into();
                }
            }
        }
    }
    if !fighter.global_table[IS_STOPPING].get_bool() {
        special_s_shoot_helper(fighter);
    }
    
    return 0.into();
}

unsafe fn special_s_shoot_helper(fighter: &mut L2CFighterCommon) {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            let sum_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let mut stop_type = ENERGY_STOP_RESET_TYPE_NONE;
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                stop_type = ENERGY_STOP_RESET_TYPE_GROUND;
            }
            else {
                stop_type = ENERGY_STOP_RESET_TYPE_AIR;
            }
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, stop_type, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_x, 0.0);
            KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT, special_s_shoot_main);
}