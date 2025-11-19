use super::*;

pub unsafe extern "C" fn special_n3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_status_kind_interrupt(*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH);
    return 1.into();
}

unsafe extern "C" fn special_n3_catch_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_n3_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    KineticModule::unable_energy_all(fighter.module_accessor);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        let brake_x = fighter.get_param_float("ground_brake", "");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, speed_x, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, brake_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw3_catch"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        let air_brake_x = fighter.get_param_float("air_brake_x", "");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x * 0.5, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, air_brake_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if !VarModule::is_flag(fighter.object(), vars::miifighter::instance::SPECIAL_N3_STALL) {
            VarModule::on_flag(fighter.object(), vars::miifighter::instance::SPECIAL_N3_STALL);
            let start_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_throw_start_accel_y"));
            let throw_speed_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_throw_speed_max_y"));
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y * 0.5, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -start_accel_y);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, throw_speed_max_y);
        }
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3_catch"), 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.main_shift(special_n3_catch_main_loop)
}

unsafe extern "C" fn special_n3_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.status_frame() < 15 {
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
    } else {
        StatusModule::set_keep_situation_air(fighter.module_accessor, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT } else { FIGHTER_STATUS_KIND_FALL };
        fighter.change_status(status.into(), false.into());
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw3_catch"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw3_catch"), -1.0, 1.0, 0.0, false, false);
        }
    }

    return 0.into()
}

unsafe extern "C" fn special_n3_throw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    fighter.set_float(1.0, *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, module_accessor);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, module_accessor);
    smash::app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, module_accessor);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if VarModule::is_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_IS_LINK) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    
    return 0.into();
}

unsafe extern "C" fn special_n3_throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_IS_LINK) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n3_yeet").into(), Hash40::new("special_air_n3_yeet").into(), false.into());
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_CATCH_SET_CATCH);
        sv_module_access::_catch(fighter.lua_state_agent);
        fighter.set_int(0, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        fighter.set_int(0, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);

        let counter_attack_power = fighter.get_float(*FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
        let attack_mul = fighter.get_param_float("param_special_lw", "lw3_attack_mul");
        let mut attack_power = counter_attack_power * attack_mul;
        let attack_power_limit = fighter.get_param_float("param_special_lw", "lw3_attack_power_limit");
        if attack_power < attack_power_limit {
            attack_power = 0.0;
        }
        if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_IS_ATTACK_ENEMY) {
            let attack_max_for_enemy = fighter.get_param_float("param_special_lw", "lw3_attack_max_for_enemy");
            if attack_max_for_enemy < attack_power {
                attack_power = attack_max_for_enemy;
            }
        }
        else {
            let attack_max = fighter.get_param_float("param_special_lw", "lw3_attack_max");
            if attack_max < attack_power {
                attack_power = attack_max
            }
        }
        let get_node_object_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
        fighter.set_float(attack_power, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
        fighter.set_int(get_node_object_id as i32, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw3_throw").into(), Hash40::new("special_air_lw3_throw").into(), false.into());
    }

    fighter.main_shift(special_n3_throw_main_loop)
}

unsafe extern "C" fn special_n3_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING) {
        if !fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                if VarModule::is_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_IS_LINK) {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n3_yeet"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw3_throw"), -1.0, 1.0, 0.0, false, false);
                }
            }
        }
    }
    let counter_throw_object_id = VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_GRABBED_OBJECT_ID);
    if VarModule::is_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_IS_LINK) {
        if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
            if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                let init_life = WorkModule::get_int(counter_throw_boma, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
                WorkModule::set_int(counter_throw_boma, init_life * 2, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                GroundModule::set_ignore_boss(counter_throw_boma, true);
                GroundModule::set_passable_check(counter_throw_boma, false);
                GroundModule::set_collidable(counter_throw_boma, false);
                JostleModule::set_status(counter_throw_boma, false);
                if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                    LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
                }
                if !LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                    VisibilityModule::set_whole(counter_throw_boma, true);
                    LinkModule::link(counter_throw_boma, *LINK_NO_ARTICLE, (*fighter.module_accessor).battle_object_id);
                    LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *LINK_NO_ARTICLE, Hash40::new("rot"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                    LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
                }
            }
            else if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                let init_life = WorkModule::get_int(counter_throw_boma, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
                WorkModule::set_int(counter_throw_boma, init_life * 2, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                GroundModule::set_ignore_boss(counter_throw_boma, true);
                GroundModule::set_passable_check(counter_throw_boma, false);
                GroundModule::set_collidable(counter_throw_boma, false);
                JostleModule::set_status(counter_throw_boma, false);
                if LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                    LinkModule::unlink(counter_throw_boma, *ITEM_LINK_NO_HAVE);
                }
                if !LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                    VisibilityModule::set_whole(counter_throw_boma, true);
                    LinkModule::link(counter_throw_boma, *ITEM_LINK_NO_HAVE, (*fighter.module_accessor).battle_object_id);
                    LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *ITEM_LINK_NO_HAVE, Hash40::new("rot"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                    LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
                }
            }
        }
    }
    else {
        let attack_power = fighter.get_float(*FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLOAT_ATTACK_POWER);
        if 0.0 < attack_power {
            AttackModule::set_power(fighter.module_accessor, 0, attack_power, true);
        }
        if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
            if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                    LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
                }
            }
            if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
                let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
                LinkModule::remove_model_constraint(counter_throw_boma, true);
                if LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
                    LinkModule::unlink(counter_throw_boma, *ITEM_LINK_NO_HAVE);
                }
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 0.into();
    }
    
    return 0.into();
}

unsafe extern "C" fn special_n3_throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    CatchModule::catch_cut(fighter.module_accessor, false, false);
    VarModule::off_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_IS_LINK);
    let counter_throw_object_id = VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_GRABBED_OBJECT_ID);
    if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_GRABBED_OBJECT_ID, *BATTLE_OBJECT_ID_INVALID);
    }
    
    return 0.into();
}

unsafe extern "C" fn special_n3_throw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let counter_throw_object_id = VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_GRABBED_OBJECT_ID);
    VarModule::off_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_IS_LINK);
    if counter_throw_object_id != *BATTLE_OBJECT_ID_INVALID {
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        if sv_battle_object::category(counter_throw_object_id as u32) == *BATTLE_OBJECT_CATEGORY_ITEM {
            let counter_throw_boma = sv_battle_object::module_accessor(counter_throw_object_id as u32);
            LinkModule::remove_model_constraint(counter_throw_boma, true);
            if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
                LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
            }
        }
        VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_GRABBED_OBJECT_ID, *BATTLE_OBJECT_ID_INVALID);
    }
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH, special_n3_catch_pre);
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH, special_n3_catch_main);

    agent.status(Init, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, special_n3_throw_init);
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, special_n3_throw_main);
    agent.status(End, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, special_n3_throw_end);
    agent.status(Exit, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, special_n3_throw_exit);
}