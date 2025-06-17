use super::*;

unsafe extern "C" fn effect_specials3s(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0, 0.05, 0.7);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_sword_red"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("chrom_mc_3s"), Hash40::new("top"), 0, 0, 4, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("chrom_sword_red"), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn game_specials3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let rush_speed = 8.0;
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 7.0/(4.0-1.0));
    frame(lua_state, 4.0);
    let attack_rate = 4.0/(5.0-4.0);
    FT_MOTION_RATE(agent, attack_rate);
    if is_excute(agent) {
        //KineticModule::add_speed(boma, &Vector3f::new(rush_speed, 0.0, 0.0));
    }
    frame(lua_state, 4.0 + 1.0/attack_rate);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 5.0, 10, 1, 0, 90, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 5.0, 10, 1, 0, 90, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("sword1"), 5.0, 10, 1, 0, 90, 4.2, 0.0, 0.0, 1.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("sword1"), 5.0, 10, 1, 0, 90, 4.2, 0.0, 0.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(boma, 0, -15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, -15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, -15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, -15.0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        //KineticModule::add_speed(boma, &Vector3f::new(-(1.0/4.0) * rush_speed, 0.0, 0.0));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //KineticModule::add_speed(boma, &Vector3f::new(-(1.0/4.0) * rush_speed, 0.0, 0.0));
        AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT){
            ATTACK(agent, 4, 1, Hash40::new("top"), 0.0, 180, 100, 25, 0, 9.0, 0.0, 9.0, 8.0, Some(0.0), Some(9.0), Some(35.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        //KineticModule::add_speed(boma, &Vector3f::new(-(1.0/4.0) * rush_speed, 0.0, 0.0));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        //KineticModule::add_speed(boma, &Vector3f::new(-(1.0/4.0) * rush_speed, 0.0, 0.0));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT){
            WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

unsafe extern "C" fn effect_specials3lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let attack_rate = 4.0/(5.0-4.0);
    frame(lua_state, 3.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0.93, 0.03, 0.7);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new_raw(0x115a027923), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 4.0 + 2.0/attack_rate);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new_raw(0x0cf989ad1e), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new_raw(0x115a027923), false, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_specials3s", effect_specials3s, Priority::Low);
    agent.acmd("effect_specialairs3s", effect_specials3s, Priority::Low);

    agent.acmd("game_specials3lw", game_specials3lw, Priority::Low);
    agent.acmd("game_specialairs3lw", game_specials3lw, Priority::Low);
    agent.acmd("effect_specials3lw", effect_specials3lw, Priority::Low);
    agent.acmd("effect_specialairs3lw", effect_specials3lw, Priority::Low);
}