use super::*;

unsafe extern "C" fn game_dead(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 60.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_MURABITO_TREE_STATUS_DISAPPEAR_WORK_FLAG_DEAD);
    }
}

unsafe extern "C" fn effect_dead(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("murabito_leaf3"), Hash40::new("tree"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 2.0);
        FLASH(agent, 0.246, 0.185, 0.068, 0);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        FLASH_FRM(agent, 100, 0.246, 0.185, 0.068, 0.588);
    }
    wait(lua_state, 100.0);
    if is_excute(agent) {
        FLASH(agent, 0.246, 0.185, 0.068, 0.588);
    }
}

unsafe extern "C" fn effect_disappear(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 82.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("murabito_erase_smoke2"), Hash40::new("top"), 0, 7.5, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_dead", game_dead, Priority::Low);
    agent.acmd("effect_dead", effect_dead, Priority::Low);
    agent.acmd("effect_disappear", effect_disappear, Priority::Low);
}
