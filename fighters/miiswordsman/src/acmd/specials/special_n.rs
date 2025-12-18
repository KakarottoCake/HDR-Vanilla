use super::*;

// ================================================================================================
// ======================================= SOARING ZEPHYR =========================================
// ================================================================================================

unsafe extern "C" fn game_specialn1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 17.0, 14.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_TORNADOSHOT, false, 0);
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 48.0, 17.0);
    frame(lua_state, 48.0);
    FT_MOTION_RATE(agent, 1.0);
}

// ================================================================================================
// ======================================== WARRIOR WAVE ==========================================
// ================================================================================================

unsafe extern "C" fn game_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 14.0, 9.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
        }
        else {
            let x_vel = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            agent.clear_lua_stack();
            lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, x_vel * 0.6, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(lua_state);
            let air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);
            sv_kinetic_energy!(set_stable_speed, agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * 0.5, 0.0);
        }
    }
    frame(lua_state, 14.0);
    FT_MOTION_RATE(agent, 1.0);
    if boma.is_button_on(Buttons::Special) {
        FT_MOTION_RATE_RANGE(agent, 14.0, 17.75, 13.0);
    }
    else {
        FT_MOTION_RATE_RANGE(agent, 14.0, 17.75, 3.0);
    }
    if is_excute(agent) {
        if boma.is_button_on(Buttons::Special) {
            VarModule::on_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
    frame(lua_state, 17.75);
    FT_MOTION_RATE_RANGE(agent, 17.75, 18.0, 1.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
                agent.clear_lua_stack();
                lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.8);
                app::sv_kinetic_energy::set_speed(lua_state);
            }
        }
        ATTACK(agent, 0, 0, Hash40::new("armr"), 10.0, 361, 100, 0, 39, 3.5, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 361, 100, 0, 39, 3.5, 0.0, 3.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 12.0, 361, 100, 0, 39, 3.5, 0.0, 8.75, -2.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            ArticleModule::generate_article(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, false, 0);
            ArticleModule::shoot_exist(boma, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 33.0);
    if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        FT_MOTION_RATE_RANGE(agent, 33.0, 36.0, 10.0);
    }
    frame(lua_state, 36.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_miiswordsman_sword03"), Hash40::new("tex_miiswordsman_sword04"), 8, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 10.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_Y, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 14.2);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 7.5, 0.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 16.5);
    if is_excute(agent) {
        if !VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), -1, 10, 2, -10.6, -140, 90.0, 1.3, true);
            LAST_EFFECT_SET_RATE(agent, 1.4);
            LAST_EFFECT_SET_SCALE_W(agent, 1.3, 1.3, 1.0);
        }
    }
    frame(lua_state, 17.25);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_counter_arc"), Hash40::new("top"), -1, 10, 2, -10.6, -140, 90.0, 1.3, true);
            LAST_EFFECT_SET_RATE(agent, 1.4);
            LAST_EFFECT_SET_SCALE_W(agent, 1.3, 1.3, 1.0);
        }
    }
    frame(lua_state, 17.75);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4.0);
    }
}

unsafe extern "C" fn sound_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 16.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_special_c2_l01"));
        PLAY_SE(agent, Hash40::new("se_miiswordsman_special_l03"));
    }
}

unsafe extern "C" fn expression_specialn2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x26769bd1de), 0, 0, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        } else {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x26769bd1de), 0, 30, 20);
    }
}

// ================================================================================================
// ======================================== BLURRING BLADE ========================================
// ================================================================================================

unsafe extern "C" fn game_specialn3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        SET_SPEED_EX(agent, 0.5, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(lua_state, 6.0);
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        for _ in 0..4 {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("haver"), 1.0, 180, 100, 7, 0, 3.5, 0.0, 10.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                ATTACK(agent, 1, 0, Hash40::new("haver"), 1.0, 0, 100, 5, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
            wait(lua_state, 2.0);
            if is_excute(agent) {
                AttackModule::clear_all(boma);
            }
            wait(lua_state, 2.0);
        }
        if is_excute(agent) {
            ATTACK(agent, 1, 0, Hash40::new("haver"), 1.0, 91, 100, 21, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
            AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
        }
    }
    else {
        for _ in 0..4 {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("haver"), 1.0, 366, 100, 10, 0, 3.5, 0.0, -2.0, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
            wait(lua_state, 2.0);
            if is_excute(agent) {
                AttackModule::clear_all(boma);
            }
            wait(lua_state, 2.0);
        }
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("haver"), 1.0, 120, 100, 20, 0, 4.0, 0.0, 8.2, 0.0, None, None, None, 0.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("haver"), 1.0, 45, 100, 25, 0, 4.0, 0.0, -1.2, 0.0, Some(0.0), Some(12.0), Some(0.0), 0.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 2.0);
            AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW3_CHECK_INPUT);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::miiswordsman::status::SPECIAL_LW3_CHECK_INPUT);
    }
}

unsafe extern "C" fn effect_specialn3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 4, 0, -5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        if agent.is_motion_one_of(&[Hash40::new("special_n3_end_max"), Hash40::new("special_air_n3_max")]) {
            LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        let tex_sword = agent.get_int64(*FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD) as u64;
        let sword_add = agent.get_int64(*FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD) as u64;
        let sword_flare = agent.get_int64(*FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE) as u64;
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(tex_sword), Hash40::new_raw(sword_add), 3, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, Hash40::new_raw(sword_flare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 2.0, 0.2);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_rapid_slash_wind_s"), Hash40::new("top"), -0.0, 5.5, 12, 0, 0, 0, 1, true);
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_wind_s"), false, false);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn sound_specialn3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_miiswordsman_special_s01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_miiswordsman_rnd_special_c3_n01"));
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miiswordsman_special_c3_n01"));
    }
}

unsafe extern "C" fn expression_specialn3end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_motion_one_of(&[Hash40::new("special_n3_end_max"), Hash40::new("special_air_n3_max")]) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x26769bd1de), 0, 30, 3);
        }
        else {
            AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_NONE), AttackDirectionAxis(*ATTACK_DIRECTION_NONE));
        }
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashss"), 4);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallw3end2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::add_speed(boma, &Vector3f::new(0.0, 2.0, 0.0));
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        let sfx = if agent.is_motion_one_of(&[Hash40::new("special_n3_end2_max"), Hash40::new("special_air_n3_end2_max")]) { *COLLISION_SOUND_ATTR_FIRE } else { *COLLISION_SOUND_ATTR_KICK };
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 70, 70, 0, 80, 10.0, 0.0, 10.0, 9.5, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, sfx, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(agent, 15.0, 55.0, 26.0);
    frame(lua_state, 55.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallw3end2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("miiswordsman_rapid_slash_sword"), Hash40::new("haver"), 0, -0.5, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("miiswordsman_rapid_slash_arc"), Hash40::new("top"), 0.75, 11, 1, -20, -70, 90, 1.2, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        }
        EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("haver"), -0.0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_rapid_slash_sword"), false, true);
    }
}

unsafe extern "C" fn sound_speciallw3end2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_miiswordsman_special_c3_n02"));
    }
}

unsafe extern "C" fn expression_speciallw3end2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if agent.is_motion_one_of(&[Hash40::new("special_n3_end2_max"), Hash40::new("special_air_n3_end2_max")]) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        }
        else {
            RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x26769bd1de), 0, 30, 10);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn1", game_specialn1, Priority::Low);
    agent.acmd("game_specialairn1", game_specialn1, Priority::Low);

    agent.acmd("game_specialn2", game_specialn2, Priority::Low);
    agent.acmd("game_specialairn2", game_specialn2, Priority::Low);
    agent.acmd("effect_specialn2", effect_specialn2, Priority::Low);
    agent.acmd("effect_specialairn2", effect_specialn2, Priority::Low);
    agent.acmd("sound_specialn2", sound_specialn2, Priority::Low);
    agent.acmd("sound_specialairn2", sound_specialn2, Priority::Low);
    agent.acmd("expression_specialn2", expression_specialn2, Priority::Low);
    agent.acmd("expression_specialairn2", expression_specialn2, Priority::Low);

    agent.acmd("game_specialn3end", game_specialn3end, Priority::Low);
    agent.acmd("game_specialairn3end", game_specialn3end, Priority::Low);
    agent.acmd("effect_specialn3end", effect_specialn3end, Priority::Low);
    agent.acmd("effect_specialairn3end", effect_specialn3end, Priority::Low);
    agent.acmd("sound_specialn3end", sound_specialn3end, Priority::Low);
    agent.acmd("sound_specialairn3end", sound_specialn3end, Priority::Low);
    agent.acmd("expression_specialn3end", expression_specialn3end, Priority::Low);
    agent.acmd("expression_specialairn3end", expression_specialn3end, Priority::Low);
    
    agent.acmd("game_specialn3endmax", game_specialn3end, Priority::Low);
    agent.acmd("game_specialairn3endmax", game_specialn3end, Priority::Low);
    agent.acmd("effect_specialn3endmax", effect_specialn3end, Priority::Low);
    agent.acmd("effect_specialairn3endmax", effect_specialn3end, Priority::Low);
    agent.acmd("sound_specialn3endmax", sound_specialn3end, Priority::Low);
    agent.acmd("sound_specialairn3endmax", sound_specialn3end, Priority::Low);
    agent.acmd("expression_specialn3endmax", expression_specialn3end, Priority::Low);
    agent.acmd("expression_specialairn3endmax", expression_specialn3end, Priority::Low);

    agent.acmd("game_speciallw3end2", game_speciallw3end2, Priority::Low);
    agent.acmd("game_specialairlw3end2", game_speciallw3end2, Priority::Low);
    agent.acmd("effect_speciallw3end2", effect_speciallw3end2, Priority::Low);
    agent.acmd("effect_specialairlw3end2", effect_speciallw3end2, Priority::Low);
    agent.acmd("sound_speciallw3end2", sound_speciallw3end2, Priority::Low);
    agent.acmd("sound_specialairlw3end2", sound_speciallw3end2, Priority::Low);
    agent.acmd("expression_speciallw3end2", expression_speciallw3end2, Priority::Low);
    agent.acmd("expression_specialairlw3end2", expression_speciallw3end2, Priority::Low);

    agent.acmd("game_speciallw3end2max", game_speciallw3end2, Priority::Low);
    agent.acmd("game_specialairlw3end2max", game_speciallw3end2, Priority::Low);
    agent.acmd("effect_speciallw3end2max", effect_speciallw3end2, Priority::Low);
    agent.acmd("effect_specialairlw3end2max", effect_speciallw3end2, Priority::Low);
    agent.acmd("sound_speciallw3end2max", sound_speciallw3end2, Priority::Low);
    agent.acmd("sound_specialairlw3end2max", sound_speciallw3end2, Priority::Low);
    agent.acmd("expression_speciallw3end2max", expression_speciallw3end2, Priority::Low);
    agent.acmd("expression_specialairlw3end2max", expression_speciallw3end2, Priority::Low);
}