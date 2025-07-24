use super::*;

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    FT_MOTION_RATE_RANGE(agent, 2.0, 52.0, 17.0);
    frame(lua_state, 52.0);
    FT_MOTION_RATE(agent, 16.0 / 3.0);
    frame(lua_state, 55.0);
    FT_MOTION_RATE(agent, 10.0 / 7.0);
    frame(lua_state, 62.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 63.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 20.0, 51, 83, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 0.0);
    }
    frame(lua_state, 67.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
}