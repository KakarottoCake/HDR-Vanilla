use super::*;

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 0.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                KineticModule::add_speed(boma, &Vector3f{x: 0.3, y: -1.0, z: 0.0});
                WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        } else {
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                KineticModule::add_speed(boma, &Vector3f{x: 1.3, y: -1.5, z: 0.0});
                WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
    }

    frame(lua_state, 1.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -0.3, z: 0.0});
        } else {
                KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -1.0, z: 0.0});
        }
    }

    frame(lua_state, 2.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -0.5, z: 0.0});
        }
    }

    frame(lua_state, 3.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 70, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            } else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 50, 70, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        } else {
            if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 50, 65, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            } else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 50, 75, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            }
        }
    }
    
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            } else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        } else {
            if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            } else {
                ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 310, 95, 0, 30, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            } 
        }
    
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            MotionModule::set_rate(boma, 1.2);
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            MotionModule::set_rate(boma, 1.0);
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 6.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 8.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 10.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.05, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }
    
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 12.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 13.0);
    if is_excute(agent) {
        if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        } else {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.2, z: 0.0});
        }
    }

    frame(lua_state, 14.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            if agent.get_int(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
                ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }

    frame(lua_state, 15.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 10.0);
            MotionModule::set_rate(boma, 10.0);
    }

    frame(lua_state, 35.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_DOLLY_SPECIAL_LW_FALL);
        MotionModule::set_rate(boma, 1.0);
    }

    frame(lua_state, 37.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY);
    }
}

unsafe extern "C" fn game_specialairlwrise(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        agent.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::clear_speed_all(boma);
        agent.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        if agent.is_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 85, 100, 40, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    
    frame(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }

    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 65, 100, 50, 20, 7.0, 0.0, 9.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 40, 100, 30, 10, 7.0, 0.0, 9.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }

    frame(lua_state, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 40, 100, 30, 10, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }

    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        agent.off_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }

    frame(lua_state, 28.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::add_speed(boma, &Vector3f{x: 0.3, y: -1.5, z: 0.0});
        agent.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
}

unsafe extern "C" fn game_specialairlwrisew(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        agent.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::clear_speed_all(boma);
        agent.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        if agent.is_flag(*FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 90, 100, 50, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 65, 100, 40, 10, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 14, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }

    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 50, 100, 30, 10, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }

    frame(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        agent.off_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }

    frame(lua_state, 22.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::add_speed(boma, &Vector3f{x: 5.0, y: -1.5, z: 0.0});
        agent.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
}

unsafe extern "C" fn game_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        SET_SPEED_EX(agent, 1, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::dolly::status::INHERIT_FINAL_CANCEL_ON_END);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);
    agent.acmd("game_specialairlwrise", game_specialairlwrise, Priority::Low);
    agent.acmd("game_specialairlwrisew", game_specialairlwrisew, Priority::Low);
    agent.acmd("game_speciallwend", game_speciallwend, Priority::Low);
}
