use super::*;
use globals::*;

unsafe extern "C" fn game_specialnstarth(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    let startup_frame = 14.0;
    FT_MOTION_RATE_RANGE(agent,1.0,30.0, startup_frame);//match to bayo
    if VarModule::is_flag(agent.battle_object, vars::common::instance::WAS_PREV_STATUS_CANCELABLE) {
        VarModule::off_flag(agent.battle_object, vars::bayonetta::instance::WAS_CANCEL);//should maybe hopefully disable the flag if she didnt cancel into it w/o messing with end statuses
    }
}

unsafe extern "C" fn game_specialnchargeh(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if VarModule::is_flag(agent.battle_object, vars::bayonetta::instance::WAS_CANCEL) {
        MotionModule::set_rate(boma, (15.0 - 1.0)/18.0);//van - 5, 34f total
        //FT_MOTION_RATE_RANGE(agent,1.0,15.0, 18.0);//van - 5, 34f total
    } else {
        MotionModule::set_rate(boma, (15.0 - 1.0)/28.0);//van + 4, 44f total
        //FT_MOTION_RATE_RANGE(agent,1.0,15.0, 28.0);//van + 4, 44f total
    }
}

unsafe extern "C" fn game_specialnendh(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    let max_repeat = agent.get_param_int("param_special_n", "add_fire_max");
    let remaining_repeats = agent.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_ADD_FIRE_COUNT);
    let used_rounds = (max_repeat - remaining_repeats) as f32;
    let lag_per_round = 5.0;//kirby
    let base_endlag = 24.0;//hardcode, update to match any changes
    if agent.is_status(statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL) {
        MotionModule::set_rate(boma, (65.0 - 1.0)/base_endlag);

        //FT_MOTION_RATE_RANGE(agent,1.0,58.0, base_endlag);
    } else {
        MotionModule::set_rate(boma, (65.0 - 1.0)/base_endlag+ lag_per_round*used_rounds);

        //FT_MOTION_RATE_RANGE(agent,1.0,58.0, base_endlag + lag_per_round*used_rounds);
    }
}

unsafe extern "C" fn game_specialnendf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    let max_repeat = agent.get_param_int("param_special_n", "add_fire_max");
    let remaining_repeats = agent.get_int(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_N_INT_ADD_FIRE_COUNT);
    let used_rounds = (max_repeat - remaining_repeats) as f32;
    let lag_per_round = 5.0;
    let base_endlag = 24.0;
    if agent.is_status(statuses::kirby::BAYONETTA_SPECIAL_N_CANCEL) {
        //MotionModule::set_rate(boma, (65.0 - 1.0)/base_endlag);
        FT_MOTION_RATE_RANGE(agent,1.0,48.0, base_endlag);
        
    } else {
        FT_MOTION_RATE_RANGE(agent,1.0,48.0, base_endlag + lag_per_round*used_rounds);
        //MotionModule::set_rate(boma, (65.0 - 1.0)/base_endlag);//+ lag_per_round*used_rounds);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_bayonettaspecialnstarth", game_specialnstarth, Priority::Low);
    agent.acmd("game_bayonettaspecialnstartf", game_specialnstarth, Priority::Low);
    agent.acmd("game_bayonettaspecialairnstarth", game_specialnstarth, Priority::Low);
    agent.acmd("game_bayonettaspecialairnstartf", game_specialnstarth, Priority::Low);
    agent.acmd("game_bayonettaspecialnchargeh", game_specialnchargeh, Priority::Low);
    agent.acmd("game_bayonettaspecialnchargef", game_specialnchargeh, Priority::Low);
    agent.acmd("game_bayonettaspecialairnchargeh", game_specialnchargeh, Priority::Low);
    agent.acmd("game_bayonettaspecialairnchargef", game_specialnchargeh, Priority::Low);
    //agent.acmd("game_bayonettaspecialnendh", game_specialnendh, Priority::Low);
    //agent.acmd("game_bayonettaspecialnendf", game_specialnendf, Priority::Low);
    //agent.acmd("game_bayonettaspecialairnendh", game_specialnendh, Priority::Low);
    //agent.acmd("game_bayonettaspecialairnendf", game_specialnendf, Priority::Low);
}
