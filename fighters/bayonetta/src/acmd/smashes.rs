use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    app::sv_animcmd::execute(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 5.0);
    app::sv_animcmd::execute(lua_state, 5.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
            ArticleModule::generate_article(boma, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVEARM, false, -1);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 2, 5, 0, true);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::set_power_mul_status(boma, 1.0);
        CHECK_BA(agent, true);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 1.75);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    app::sv_animcmd::execute(lua_state, 5.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        if agent.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
            ArticleModule::generate_article(boma, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVEARM, false, -1); // f17 ingame
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, false, true, 10, 2, 5, 0, true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, true, 10);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::set_power_mul_status(boma, 1.0);
        CHECK_BA(agent, true);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE_RANGE(agent, 30.0, 65.0, 32.0); // makeup for slowed start
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2aa2fe5e2f), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if agent.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            ArticleModule::generate_article(boma, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, false, -1);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, false, 10, 10, 10, 0, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, false, 10);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE_RANGE(agent, 12.0, 16.0, 3.0);
    frame(lua_state, 16.0);
    FT_MOTION_RATE_RANGE(agent, 16.0, 20.0, 2.0);
    frame(lua_state, 20.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 0, 100, 20, 0, 4.0, 0.0, 3.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        //ATK_NO_REACTION_SEARCH_WITCH_TIME(agent, 0); ?
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        CHECK_BA(agent, true);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Low);
    agent.acmd("game_attacks4hi", game_attacks4, Priority::Low);
    agent.acmd("game_attacks4lw", game_attacks4, Priority::Low);

    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
}
