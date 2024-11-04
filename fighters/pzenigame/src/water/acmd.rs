use super::*;

unsafe extern "C" fn game_regular(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 366, 100, 20, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn effect_regular(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_sscope_bullet_max"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, false);
        LAST_EFFECT_SET_COLOR(agent, 0.611, 0.862, 122.866);
    }
    for i in 1..=50 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_splash"), Hash40::new("top"), 0, 0, 0, 0, 0, 90, 0.1, false);
            EFFECT_FOLLOW(agent, Hash40::new("sys_splash"), Hash40::new("top"), 0, 0, 0, -90, 0, 0, 0.3, false);
        }
        wait(lua_state, 6.0);
    }
}

unsafe extern "C" fn game_clashpledgeg(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 366, 100, 20, 0, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 1, Hash40::new("top"), 3.0, 54, 100, 0, 15, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_clashpledgeg(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_mizuteppo_hit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_clashpledgef(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 60, 120, 0, 35, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WATER);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_clashpledgef(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pzenigame_mizuteppo_hit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_die(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pzenigame_takinobori_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, true);
        EFFECT_DETACH_KIND(agent, Hash40::new("pzenigame_takinobori_end"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_splash"), -1);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_regular", game_regular, Priority::Low);
    agent.acmd("effect_regular", effect_regular, Priority::Low);

    agent.acmd("game_clashpledgeg", game_clashpledgeg, Priority::Low);
    agent.acmd("effect_clashpledgeg", effect_clashpledgeg, Priority::Low);
    agent.acmd("sound_clashpledgeg", acmd_stub, Priority::Low);

    agent.acmd("game_clashpledgef", game_clashpledgef, Priority::Low);
    agent.acmd("effect_clashpledgef", effect_clashpledgef, Priority::Low);
    agent.acmd("sound_clashpledgef", acmd_stub, Priority::Low);

    agent.acmd("effect_die", effect_die, Priority::Low);
}