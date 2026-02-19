use super::*;

// ================================================================================================
// ========================================= SHOT PUT =============================================
// ================================================================================================

unsafe extern "C" fn game_specialn1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article_enable(boma, *FIGHTER_MIIFIGHTER_GENERATE_ARTICLE_IRONBALL, false, -1);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_N1_START_HOLD);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_N1_END_HOLD);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ArticleModule::shoot_exist(boma, *FIGHTER_MIIFIGHTER_GENERATE_ARTICLE_IRONBALL, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(agent, 40.0, 80.0, 20.0);
    frame(lua_state, 80.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialn1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -5, 8.5, -0.5, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 360, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        let eff_handle = VarModule::get_int(agent.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE) as u32;
        if EffectModule::is_exist_effect(boma, eff_handle) {
            EffectModule::kill(boma, eff_handle, true, true);
            VarModule::set_int(agent.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE, -1);
        }
    }
}

unsafe extern "C" fn game_specialn1bowl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 25.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miifighter::status::SPECIAL_N1_END_HOLD);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        ArticleModule::shoot_exist(boma, *FIGHTER_MIIFIGHTER_GENERATE_ARTICLE_IRONBALL, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(agent, 40.0, 80.0, 20.0);
    frame(lua_state, 80.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialn1bowl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 25.0);
    if is_excute(agent) {
        let eff_handle = VarModule::get_int(agent.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE) as u32;
        if EffectModule::is_exist_effect(boma, eff_handle) {
            EffectModule::kill(boma, eff_handle, true, true);
            VarModule::set_int(agent.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE, -1);
        }
    }
}

unsafe extern "C" fn sound_specialn1bowl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miifighter_special_n02"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miifighter_rnd_special_c1_n01"));
    }
}

unsafe extern "C" fn expression_specialn1bowl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrowitem"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 79.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

// ================================================================================================
// ===================================== ULTIMATE UPPERCUT ========================================
// ================================================================================================

unsafe extern "C" fn game_specialn2start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 11.0, 5.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 190, 100, 35, 0, 3.0, 0.0, 7.5, 16.5, Some(0.0), Some(8.5), Some(16.5), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 80, 100, 50, 0, 2.0, 0.0, 4.5, 9.5, Some(0.0), Some(4.5), Some(15.5), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 366, 100, 30, 0, 2.0, 0.0, 10.2, 9.5, Some(0.0), Some(10.2), Some(16.5), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 60, 100, 110, 0, 3.0, 0.0, 11.2, 10.5, Some(0.0), Some(11.2), Some(14.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 366, 100, 20, 0, 4.0, 0.0, 5.0, 15.5, Some(0.0), Some(7.0), Some(15.5), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 80, 100, 50, 0, 2.0, 0.0, 4.5, 9.5, Some(0.0), Some(4.5), Some(15.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 366, 100, 30, 0, 2.0, 0.0, 11.2, 9.5, Some(0.0), Some(11.2), Some(15.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 60, 100, 110, 0, 3.0, 0.0, 11.2, 10.5, Some(0.0), Some(11.2), Some(15.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 90, 100, 20, 0, 2.0, 0.0, 3.0, 15.5, Some(0.0), Some(9.0), Some(15.5), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 80, 100, 50, 0, 2.0, 0.0, 4.5, 9.5, Some(0.0), Some(4.5), Some(13.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 366, 100, 30, 0, 2.0, 0.0, 11.2, 9.5, Some(0.0), Some(11.2), Some(13.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 60, 100, 110, 0, 3.0, 0.0, 11.2, 10.5, Some(0.0), Some(11.2), Some(13.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 90, 100, 20, 0, 2.0, 0.0, 3.5, 15.5, Some(0.0), Some(9.5), Some(15.5), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 80, 100, 50, 0, 2.0, 0.0, 4.5, 9.5, Some(0.0), Some(4.5), Some(13.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.4, 366, 100, 30, 0, 2.0, 0.0, 11.2, 9.5, Some(0.0), Some(11.2), Some(13.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 60, 100, 110, 0, 3.0, 0.0, 11.2, 10.5, Some(0.0), Some(11.2), Some(13.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    FT_MOTION_RATE(agent, 1.1);
    frame(lua_state, 24.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_MACH_PUNCH_FLAG_PULL_FINISH_ATTACK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.4, 366, 100, 20, 0, 2.5, 0.0, 3.5, 17.0, Some(0.0), Some(9.5), Some(17.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.4, 366, 100, 40, 0, 6.0, 0.0, 6.5, 11.8, Some(0.0), Some(8.3), Some(11.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 60, 100, 40, 0, 4.0, 0.0, 4.5, 8.8, Some(0.0), Some(10.3), Some(8.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialn2finish(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        let kbg = if boma.is_situation(*SITUATION_KIND_GROUND) { 106 } else { 98 };
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.5, 80, kbg, 0, 60, 5.5, 0.0, 7.5, 12.5, Some(0.0), Some(17.0), Some(12.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 49.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_MACH_PUNCH_FLAG_SET_FALL_SPEED);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 68.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialn2miss(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_MACH_PUNCH_FLAG_SET_FALL_SPEED);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE_RANGE(agent, 18.0, 31.0, 21.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 31.0);
    FT_MOTION_RATE(agent, 1.0);
}

// ================================================================================================
// ============================================ WILD THROW ========================================
// ================================================================================================

unsafe extern "C" fn game_specialn3catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 3.5, 7.0);
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 3.5);
    FT_MOTION_RATE_RANGE(agent, 3.5, 6.0, 3.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
        CATCH(agent, 2, Hash40::new("top"), 5.0, 0.0, 7.0, 9.0, Some(0.0), Some(7.0), Some(9.5), *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.2);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            let air_accel_y = agent.get_param_float("air_accel_y", "");
            let air_speed_y_stable = agent.get_param_float("air_speed_y_stable", "");
            sv_kinetic_energy!(set_accel, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
            sv_kinetic_energy!(set_limit_speed, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
        }
    }
}

unsafe extern "C" fn effect_specialn3catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_guard_mark"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("miifighter_counter_success"), Hash40::new("top"), -1, 7, 8, 0, 90, 0, 0.6, false);
    }
}

unsafe extern "C" fn game_specialn3throw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        REVERSE_LR(agent);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 14.0, 275, 100, 25, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_CATCH_STOP(agent, 7, 1);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 7.0, 10.0);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("armr"), 9.0, 361, 85, 0, 80, 5.0, 6.0, 0.0, 0.0, Some(6.0), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 9.0, 361, 85, 0, 80, 4.0, 0.0, 3.0, -5.0, Some(0.0), Some(3.0), Some(-11.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        let opponent_boma = agent.get_grabbed_opponent_boma();
        if opponent_boma.is_fighter() && VarModule::has_var_module(opponent_boma.object()) {
            VarModule::on_flag(opponent_boma.object(), vars::common::instance::FORCE_TUMBLE_NO_BOUNCE);
        }
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 47.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairn3throw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING);
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        REVERSE_LR(agent);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 14.0, 280, 28, 0, 33, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_CATCH_STOP(agent, 7, 1);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 7.0, 10.0);
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("armr"), 9.0, 361, 85, 0, 80, 5.0, 6.0, 0.0, 0.0, Some(6.0), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        let opponent_boma = agent.get_grabbed_opponent_boma();
        if opponent_boma.is_fighter() {
            VarModule::on_flag(opponent_boma.object(), vars::common::instance::FORCE_TUMBLE_NO_BOUNCE);
        }
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_specialairn3throw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        COL_PRI(agent, 101);
        FLASH(agent, 1, 1, 1, 0);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_catch"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("miifighter_counter_arc"), Hash40::new("miifighter_counter_arc"), Hash40::new("top"), -1, 8, 1, 0, 112, 90, 0.8, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn1", game_specialn1, Priority::Low);
    agent.acmd("game_specialairn1", game_specialn1, Priority::Low);
    agent.acmd("effect_specialn1", effect_specialn1, Priority::Low);
    agent.acmd("effect_specialairn1", effect_specialn1, Priority::Low);

    agent.acmd("game_specialn1bowl", game_specialn1bowl, Priority::Low);
    agent.acmd("game_specialairn1bowl", game_specialn1bowl, Priority::Low);
    agent.acmd("effect_specialn1bowl", effect_specialn1bowl, Priority::Low);
    agent.acmd("effect_specialairn1bowl", effect_specialn1bowl, Priority::Low);
    agent.acmd("sound_specialn1bowl", sound_specialn1bowl, Priority::Low);
    agent.acmd("sound_specialairn1bowl", sound_specialn1bowl, Priority::Low);
    agent.acmd("expression_specialn1bowl", expression_specialn1bowl, Priority::Low);
    agent.acmd("expression_specialairn1bowl", expression_specialn1bowl, Priority::Low);

    agent.acmd("game_specialn2start", acmd_stub, Priority::Low);
    agent.acmd("game_specialairn2start", acmd_stub, Priority::Low);
    agent.acmd("effect_specialn2start", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairn2start", acmd_stub, Priority::Low);
    agent.acmd("sound_specialn2start", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairn2start", acmd_stub, Priority::Low);
    agent.acmd("expression_specialn2start", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairn2start", acmd_stub, Priority::Low);

    agent.acmd("game_specialn2hold", acmd_stub, Priority::Low);
    agent.acmd("game_specialairn2hold", acmd_stub, Priority::Low);
    agent.acmd("effect_specialn2hold", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairn2hold", acmd_stub, Priority::Low);
    agent.acmd("sound_specialn2hold", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairn2hold", acmd_stub, Priority::Low);
    agent.acmd("expression_specialn2hold", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairn2hold", acmd_stub, Priority::Low);

    agent.acmd("game_specialn2end", acmd_stub, Priority::Low);
    agent.acmd("game_specialairn2end", acmd_stub, Priority::Low);
    agent.acmd("effect_specialn2end", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairn2end", acmd_stub, Priority::Low);
    agent.acmd("sound_specialn2end", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairn2end", acmd_stub, Priority::Low);
    agent.acmd("expression_specialn2end", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairn2end", acmd_stub, Priority::Low);

    agent.acmd("game_specialn2attack", acmd_stub, Priority::Low);
    agent.acmd("game_specialairn2attack", acmd_stub, Priority::Low);
    agent.acmd("effect_specialn2attack", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairn2attack", acmd_stub, Priority::Low);
    agent.acmd("sound_specialn2attack", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairn2attack", acmd_stub, Priority::Low);
    agent.acmd("expression_specialn2attack", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairn2attack", acmd_stub, Priority::Low);

    agent.acmd("game_specialn2max", acmd_stub, Priority::Low);
    agent.acmd("game_specialairn2max", acmd_stub, Priority::Low);
    agent.acmd("effect_specialn2max", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairn2max", acmd_stub, Priority::Low);
    agent.acmd("sound_specialn2max", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairn2max", acmd_stub, Priority::Low);
    agent.acmd("expression_specialn2max", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairn2max", acmd_stub, Priority::Low);

    agent.acmd("game_specialn2landing", acmd_stub, Priority::Low);
    agent.acmd("game_specialairn2landing", acmd_stub, Priority::Low);
    agent.acmd("effect_specialn2landing", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairn2landing", acmd_stub, Priority::Low);
    agent.acmd("sound_specialn2landing", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairn2landing", acmd_stub, Priority::Low);
    agent.acmd("expression_specialn2landing", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairn2landing", acmd_stub, Priority::Low);

    agent.acmd("game_speciallw3catch", game_specialn3catch, Priority::Low);
    agent.acmd("game_specialairlw3catch", game_specialn3catch, Priority::Low);
    agent.acmd("effect_speciallw3catch", effect_specialn3catch, Priority::Low);
    agent.acmd("effect_specialairlw3catch", effect_specialn3catch, Priority::Low);
    
    agent.acmd("game_speciallw3throw", game_specialn3throw, Priority::Low);
    agent.acmd("game_specialairlw3throw", game_specialairn3throw, Priority::Low);
    agent.acmd("effect_specialairlw3throw", effect_specialairn3throw, Priority::Low);
}