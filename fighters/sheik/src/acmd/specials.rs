use super::*;

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_THROW);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 80, 0, 70, 9.0, 0.0, 10.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        //ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 100, 50, 0, 11.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 70, 80, 0, 70, 9.0, 0.0, 10.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        //ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 361, 100, 50, 0, 11.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FT_SHEIK_STATUS_SPECIAL_HI_FLAG_FALL);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        let air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);
        let fall_x_mul = ParamModule::get_float(boma.object(), ParamType::Agent, "param_special_hi.fall_max_x_mul");
        sv_kinetic_energy!(
            set_stable_speed,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * fall_x_mul,
            0.0
        );
    }
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), -5.5, 0.5, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sheik_fushin_start"), Hash40::new("top"), -5.5, 0, -1.5, 0, 0, 0, 0.77, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("sys_bomb_b"), Hash40::new("top"), -5.5, 0, -1.5, 0, 0, 0, 0.84, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -5.5, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 54.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_killer_end"), Hash40::new("top"), 0, 8.5, 3, 0, 0, 0, 0.55, true);
        LAST_EFFECT_SET_RATE(agent, 1.25);
        LAST_EFFECT_SET_COLOR(agent, 0.58, 0.55, 0.5);
        LAST_EFFECT_SET_ALPHA(agent, 0.4);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_killer_end"), Hash40::new("top"), 0, 8.5, 3, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_COLOR(agent, 0.58, 0.55, 0.5);
        LAST_EFFECT_SET_ALPHA(agent, 0.4);
    }
}

unsafe extern "C" fn game_speciallwattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 2.5, 2.0);
    frame(lua_state, 2.5);
    FT_MOTION_RATE_RANGE(agent, 2.5, 13.0, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 13.0, 49, 99, 0, 24, 3.5, 5.5, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 11.0, 49, 99, 0, 24, 3.0, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
        AttackModule::set_attack_keep_rumble(boma, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE_RANGE(agent, 13.0, 18.0, 8.0); 
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.75); // 6f later
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 18.0); // return to air idle frames
    FT_MOTION_RATE_RANGE(agent, 18.0, 28.0, 45.0);
    frame(lua_state, 28.0); // idle frames
    FT_MOTION_RATE(agent, 1.0); // 2 + 45 + 8 == 55
}

unsafe extern "C" fn effect_speciallwattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.5);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sheik_chougyo_arc"), Hash40::new("top"), 0, 0, 0, 180, -48, 90, 1.1, true);
        LAST_EFFECT_SET_RATE(agent, 9.0/11.0); // match to activity
    }
}

unsafe extern "C" fn game_speciallwattackreturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 2.5, 2.0);
    frame(lua_state, 2.5);
    FT_MOTION_RATE_RANGE(agent, 2.5, 13.0, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 13.0, 49, 99, 0, 24, 3.5, 5.5, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 11.0, 49, 99, 0, 24, 3.0, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_keep_rumble(boma, 0, true);
        AttackModule::set_attack_keep_rumble(boma, 1, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE_RANGE(agent, 13.0, 18.0, 8.0); 
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.75); // 6f later
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 18.0); // return to air idle frames
    FT_MOTION_RATE_RANGE(agent, 18.0, 28.0, 28.0);
    frame(lua_state, 28.0); // idle frames
    FT_MOTION_RATE(agent, 1.0); // 2 + 28 + 8 == 38
}

unsafe extern "C" fn game_speciallwreturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("game_specialairs", game_specials, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialairhi, Priority::Low);
    agent.acmd("effect_specialhistart", effect_specialhistart, Priority::Low);
    agent.acmd("effect_specialairhistart", effect_specialhistart, Priority::Low);

    agent.acmd("game_speciallwattack", game_speciallwattack, Priority::Low);
    agent.acmd("effect_speciallwattack", effect_speciallwattack, Priority::Low);

    agent.acmd("game_speciallwattackreturn", game_speciallwattackreturn, Priority::Low);
    agent.acmd("effect_speciallwattackreturn", effect_speciallwattack, Priority::Low);

    agent.acmd("game_speciallwreturn", game_speciallwreturn, Priority::Low);
}