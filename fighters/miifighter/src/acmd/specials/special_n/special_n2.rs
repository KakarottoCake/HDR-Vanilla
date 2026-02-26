use super::*;

// ================================================================================================
// ======================================== ULTIMATE UPPERCUT =====================================
// ================================================================================================

unsafe extern "C" fn game_specialn2start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 12.0);
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialn2attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let ground_start = boma.is_situation(*SITUATION_KIND_GROUND);
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        let charge = VarModule::get_int(agent.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT);
        let damage = 10.0 + (0.1 * charge as f32);
        let kbg = if ground_start { 58 } else { 44 };
        let sound_lvl = if charge <= 100 { *ATTACK_SOUND_LEVEL_M } else { *ATTACK_SOUND_LEVEL_L };
        ATTACK(agent, 0, 0, Hash40::new("handr"), damage, 90, kbg, 0, 57, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_lvl, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), damage, 90, kbg, 0, 57, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), sound_lvl, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        let charge = VarModule::get_int(agent.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT);
        let damage = 8.0 + (0.05 * charge as f32);
        let kbg = if ground_start { 58 } else { 44 };
        ATTACK(agent, 0, 0, Hash40::new("handr"), damage, 90, kbg, 0, 57, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderr"), damage, 90, kbg, 0, 57, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn2start", game_specialn2start, Priority::Low);
    agent.acmd("game_specialairn2start", game_specialn2start, Priority::Low);
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

    agent.acmd("game_specialn2attack", game_specialn2attack, Priority::Low);
    agent.acmd("game_specialairn2attack", game_specialn2attack, Priority::Low);
    agent.acmd("effect_specialn2attack", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairn2attack", acmd_stub, Priority::Low);
    agent.acmd("sound_specialn2attack", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairn2attack", acmd_stub, Priority::Low);
    agent.acmd("expression_specialn2attack", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairn2attack", acmd_stub, Priority::Low);

    agent.acmd("game_specialn2landing", acmd_stub, Priority::Low);
    agent.acmd("game_specialairn2landing", acmd_stub, Priority::Low);
    agent.acmd("effect_specialn2landing", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairn2landing", acmd_stub, Priority::Low);
    agent.acmd("sound_specialn2landing", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairn2landing", acmd_stub, Priority::Low);
    agent.acmd("expression_specialn2landing", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairn2landing", acmd_stub, Priority::Low);
}