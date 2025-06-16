use super::*;

unsafe extern "C" fn game_specials2lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 2.0, 3.0);
    if is_excute(agent) {
        agent.on_flag( *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(lua_state, 2.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.85, 361, 25, 0, 30, 5.5, 0.0, 9.0, 8.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.85, 361, 25, 0, 30, 6.5, 0.0, 9.0, 13.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.85, 361, 25, 0, 30, 5.5, 0.0, 9.0, 18.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        agent.on_flag( *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE_RANGE(agent, 12.0, 38.0, 16.0);
    frame(lua_state, 38.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        agent.off_flag( *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

unsafe extern "C" fn effect_specials2lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0.05, 0.7);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_red"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_2lw"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_red"), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials2lw", game_specials2lw, Priority::Low);
    agent.acmd("game_specialairs2lw", game_specials2lw, Priority::Low);
    agent.acmd("effect_specials2lw", effect_specials2lw, Priority::Low);
    agent.acmd("effect_specialairs2lw", effect_specials2lw, Priority::Low);
}