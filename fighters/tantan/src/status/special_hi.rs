use super::*;

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    // prevent attack_fall from circumventing disabled up special measures
    if fighter.is_prev_status(*FIGHTER_TANTAN_STATUS_KIND_ATTACK_FALL)
    && VarModule::is_flag(fighter.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }

    return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR_HOP);
    fighter.off_flag(*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR_LASSO_IMMIDIATE);
    fighter.change_status(FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR.into(), false.into());
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10) + -1);

    return 0.into();
}

unsafe extern "C" fn special_hi_ground_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(situation),
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_ground_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    let rush_angle_g = fighter.get_param_float("param_special_hi", "rush_angle_g");
    fighter.set_float(rush_angle_g.to_radians(), *FIGHTER_TANTAN_STATUS_SPECIAL_HI_WORK_FLOAT_GROUND_ANGLE_RAD);

    fighter.main_shift(special_hi_ground_main_loop)
}

unsafe extern "C" fn special_hi_ground_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_hi").into());
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);

    return 0.into();
}

unsafe extern "C" fn special_hi_ground_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_start_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), 0x151502d27a);
    if fighter.status_frame() >= charge_start_frame {
        let max_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), 0x15c81d2557);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                || fighter.status_frame() >= max_charge_frame {
                let high_jump_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), 0x16aa6f1051);
                if fighter.status_frame() < high_jump_frame {
                    fighter.off_flag(*FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_GROUND_HIGH_JUMP);
                }
                else {
                    fighter.on_flag(*FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_GROUND_HIGH_JUMP);
                }
                fighter.change_status(FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND_JUMP.into(), false.into());
                return 0.into();
            }
        }
        else {
            if fighter.status_frame() == max_charge_frame - 5 {
                let high_jump_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), 0x16aa6f1051);
                fighter.on_flag(*FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_GROUND_HIGH_JUMP);
                fighter.change_status(FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND_JUMP.into(), false.into());
                return 0.into();
            }
        } 
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_ground_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.5);
    }
    else {
        VarModule::on_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_GROUND_START);
    }

    return smashline::original_status(Init, fighter, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND_JUMP)(fighter);
}

unsafe extern "C" fn special_hi_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    if situation == *SITUATION_KIND_GROUND {
        fighter.on_flag(*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR_HOP);
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(situation),
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn special_hi_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_frame() == 6 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && !VarModule::is_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_GROUND_START) {
        fighter.off_flag(*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR_HOP);
        fighter.change_status(FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND.into(), false.into());
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_09) + -1);
        return 1.into();
    }
    let angle = (fighter.stick_x() * -10.0 * PostureModule::lr(fighter.module_accessor)) - 5.0;
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    if (fighter.motion_frame() >= 10.0) {
        fighter.set_joint_rotate("claviclel", Vector3f::new(0.0, angle, 0.0));
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_air_reach_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    fighter.set_joint_rotate("claviclel", Vector3f::new(0.0, angle, 0.0));

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    
    agent.status(Pre, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND, special_hi_ground_pre);
    agent.status(Init, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND, special_hi_ground_init);
    agent.status(Main, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND, special_hi_ground_main);

    agent.status(Init, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND_JUMP, special_hi_ground_jump_init);
    
    agent.status(Pre, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, special_hi_air_pre);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, special_hi_air_exec);

    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR_REACH, special_hi_air_reach_exec);
}