use super::*;

unsafe extern "C" fn effect_specials4hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0.24, 1, 0.7);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_blue"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_4hi"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_blue"), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn effect_specials4s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0.05, 0.7);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_red"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_4s"), Hash40::new("top"), 0, 0, 2.75, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_red"), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn game_specials4lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    for _ in 0..4 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 80, 20, 0, 2, 5.0, 0.0, 6.0, 11.0, None, None, None, 0.5, 1.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 80, 20, 0, 2, 4.0, 0.0, 6.0, 16.0, None, None, None, 0.5, 1.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 80, 20, 0, 2, 4.5, 0.0, 6.0, 6.0, None, None, None, 0.5, 1.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.275, 361, 120, 0, 50, 4.5, 0.0, 6.0, 5.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.275, 361, 120, 0, 50, 5.0, 0.0, 7.0, 11.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 4.275, 361, 120, 0, 50, 3.0, 0.0, 8.0, 18.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specials4lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0.93, 0.03, 0.7);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_green"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.20000005, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -3.5, 0, -90, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.400000006, 1, 0.300000012);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -3.5, 0, -90, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.400000006, 1, 0.300000012);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.20000005, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -3.5, 0, -90, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.400000006, 1, 0.300000012);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -3.5, 0, -90, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.400000006, 1, 0.300000012);
        LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, -3.5, 0, -90, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, true);
        LAST_PARTICLE_SET_COLOR(agent, 0.400000006, 1, 0.300000012);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_green"), false, true);
        COL_NORMAL(agent);
    }
}

pub fn install(agent: &mut Agent) {

    agent.acmd("effect_specials4hi", effect_specials4hi, Priority::Low);
    agent.acmd("effect_specialairs4hi", effect_specials4hi, Priority::Low);

    agent.acmd("effect_specials4s", effect_specials4s, Priority::Low);
    agent.acmd("effect_specialairs4s", effect_specials4s, Priority::Low);

    agent.acmd("game_specials4lw", game_specials4lw, Priority::Low);
    agent.acmd("game_specialairs4lw", game_specials4lw, Priority::Low);
    agent.acmd("effect_specials4lw", effect_specials4lw, Priority::Low);
    agent.acmd("effect_specialairs4lw", effect_specials4lw, Priority::Low);
}