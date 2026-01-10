use super::*;

// 71000186b0
unsafe extern "C" fn attack_ext_exec_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StopModule::is_stop(fighter.module_accessor) {
        let slow_count = fighter.get_int(*FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ATTACK_SLOW_COUNT);
        if slow_count != 0 {
            return 0.into();
        }
        WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ATTACK_BOTH_RESTRICT_FRAME, 0);
        WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ATTACK_MINI_JUMP_ATTACK_FRAME, 0);
        let mut attack_hold_frame = fighter.get_int(*FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ATTACK_HOLD_FRAME);
        if attack_hold_frame > 0 {
            if !fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_INFINITY_SMASH_HOLD)
            || attack_hold_frame > 1 {
                attack_hold_frame -= 1;
            }
        }
        else {
            let mut strans_off_frame = fighter.get_int(*FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ATTACK_STRANS_OFF_FRAME);
            if strans_off_frame > 0 {
                strans_off_frame -= 1;
                if strans_off_frame != 0 {
                    if strans_off_frame == 1 {
                        FighterControlModuleImpl::delete_command(fighter.module_accessor, 0, *FIGHTER_PAD_CMD_CAT1_JUMP);
                    }
                }
                else {
                    set_transition_terms(fighter, true);
                }
                fighter.set_int(strans_off_frame, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ATTACK_STRANS_OFF_FRAME);
            }
        }
        fighter.set_int(attack_hold_frame, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ATTACK_HOLD_FRAME);
        if WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ATTACK_MAP_COLL_FRAME_L, 0) {
            if let Some(func_ptr) = smashline::api::get_target_function("lua2cpp_tantan.nrs", 0x186b0) {
                let map_coll_offset_x = fighter.get_param_float("param_attack", "map_coll_offset_x");
                let set_rhombus_thing: fn(&mut L2CFighterCommon, i32, u64, f32) = std::mem::transmute(func_ptr);
                set_rhombus_thing(fighter, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_MAP_COLL_OFFSET_X_L, hash40("arml1"), map_coll_offset_x);
            }
        }
        if WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ATTACK_MAP_COLL_FRAME_R, 0) {
            if let Some(func_ptr) = smashline::api::get_target_function("lua2cpp_tantan.nrs", 0x186b0) {
                let map_coll_offset_x = fighter.get_param_float("param_attack", "map_coll_offset_x");
                let set_rhombus_thing: fn(&mut L2CFighterCommon, i32, u64, f32) = std::mem::transmute(func_ptr);
                set_rhombus_thing(fighter, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_MAP_COLL_OFFSET_X_R, hash40("armr1"), map_coll_offset_x);
            }
        }
        fighter.inc_int(*FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ATTACK_FRAME);
    }

    return 0.into();
}

unsafe fn set_transition_terms(fighter: &mut L2CFighterCommon, some_bool: bool) {
    if some_bool {
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::enable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    }
    else {
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term_forbid_indivi(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    }
}

unsafe fn check_recoil_cancel(fighter: &mut L2CFighterCommon) {
    if !VarModule::is_flag(fighter.battle_object, vars::tantan::instance::ARMS_ATTACK_CANCEL) { return; }
    let mut new_status = 0;
    if fighter.is_cat_flag(Cat1::AttackS4) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_S3;
    } else if fighter.is_cat_flag(Cat1::AttackHi4) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
    } else if fighter.is_cat_flag(Cat1::AttackLw4) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_LW3;
    } else if fighter.is_cat_flag(Cat1::AttackS3) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_S3;
    } else if fighter.is_cat_flag(Cat1::AttackHi3) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
    } else if fighter.is_cat_flag(Cat1::AttackLw3) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_LW3;
    } else if fighter.is_cat_flag(Cat1::AttackN) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK;
    }

    if (new_status > 0) {
        //DamageModule::add_damage(fighter.module_accessor, new_status as f32, 0);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if fighter.is_pad_flag(PadFlag::JumpTrigger) {
                StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
            }
            else {
                StatusModule::change_status_force(fighter.module_accessor, new_status, false);
            }
        }
        else {
            StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
        }
        VarModule::off_flag(fighter.battle_object, vars::tantan::instance::ARMS_ATTACK_CANCEL);
    }
}

unsafe extern "C" fn attack_ext_default_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_ext_exec_inner(fighter);
    //if !StatusModule::is_changing(fighter.module_accessor) {
        check_recoil_cancel(fighter);
    //}

    return 0.into();
}

unsafe extern "C" fn attackfall_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_ext_exec_inner(fighter);
    fighter.sub_fall_uniq_process_exec();
    //if !StatusModule::is_changing(fighter.module_accessor) {
        check_recoil_cancel(fighter);
    //}

    return 0.into();
}

unsafe extern "C" fn attackjumpsquat_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT)(fighter);
    //if !StatusModule::is_changing(fighter.module_accessor) {
        check_recoil_cancel(fighter);
    //}

    return 0.into();
}

unsafe extern "C" fn attackwalk_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK)(fighter);
    //if !StatusModule::is_changing(fighter.module_accessor) {
        check_recoil_cancel(fighter);
    //}

    return 0.into();
}

unsafe extern "C" fn attackwalkback_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK_BACK)(fighter);
    //if !StatusModule::is_changing(fighter.module_accessor) {
        check_recoil_cancel(fighter);
    //}

    return 0.into();
}

unsafe extern "C" fn attacklanding_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    //if !StatusModule::is_changing(fighter.module_accessor) {
        check_recoil_cancel(fighter);
    //}

    return 0.into();
}

unsafe extern "C" fn attacklandinglight_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING_LIGHT)(fighter);
    //if !StatusModule::is_changing(fighter.module_accessor) {
        check_recoil_cancel(fighter);
    //}

    return 0.into();
}

unsafe extern "C" fn attacksquat_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    //if !StatusModule::is_changing(fighter.module_accessor) {
        check_recoil_cancel(fighter);
    //}

    return 0.into();
}

unsafe extern "C" fn attacksquatwait_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_WAIT)(fighter);
    //if !StatusModule::is_changing(fighter.module_accessor) {
        check_recoil_cancel(fighter);
    //}

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_FALL, attackfall_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_FALL_AERIAL, attackfall_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT, attackjumpsquat_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP, attack_ext_default_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL, attack_ext_default_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING, attacklanding_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING_LIGHT, attacklandinglight_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT, attacksquat_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV, attack_ext_default_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_WAIT, attacksquatwait_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT, attack_ext_default_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK, attackwalk_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK_BACK, attackwalkback_exec);
    agent.status(Exec, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK_BRAKE, attack_ext_default_exec);
}