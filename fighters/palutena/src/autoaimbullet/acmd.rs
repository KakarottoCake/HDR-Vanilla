use super::*;

unsafe extern "C" fn game_shot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let owner_module_accessor = boma.get_owner_boma();
        if owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA {
            let damage = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) {7.0} else {4.0};
            let paralyze = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) {0.42} else {0.22}; // para cap of 90, stun is kb * hitlag mul + 1
            ATTACK(agent, 0, 0, Hash40::new("top"), damage, 65, 40, 0, 75, 2.3, 0.0, 0.0, 0.0, None, None, None, paralyze, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 36, 53, 0, 61, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beamss"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn effect_shot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = boma.get_owner_boma();
    let palutena = owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA;
    if is_excute(agent) {
        if palutena {
            let red = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) {1.25} else {1.0};
            let green = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) {0.7} else {0.85};
            EFFECT_FOLLOW(agent, Hash40::new("palutena_bullet_grey"), Hash40::new("top"), 0, 0, 0.0, 0, 0, 0, 1.0, false);
            LAST_EFFECT_SET_COLOR(agent, red, green, 0.025);
            LAST_EFFECT_SET_SCALE_W(agent, 1.05, 0.85, 1.05);
            EFFECT(agent, Hash40::new("palutena_bullet_shot_grey"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 1.25, 1.00, 0.025);
            LAST_EFFECT_SET_RATE(agent, 1.5);
            LAST_EFFECT_SET_ALPHA(agent, 0.75);
        } else {
            EFFECT(agent, Hash40::new("palutena_bullet_shot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    if palutena {
        wait(lua_state, 2.0);
        if is_excute(agent) {
            if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) {
                EFFECT_FOLLOW(agent, Hash40::new("palutena_elec"), Hash40::new("top"), 0.0, 0.0, -1.0, 0, 0, 0, 0.35, true);
                LAST_EFFECT_SET_SCALE_W(agent, 0.75, 0.4, 0.45);
                LAST_EFFECT_SET_COLOR(agent, 1.25, 0.45, 0.025);
                LAST_EFFECT_SET_ALPHA(agent, 0.25);
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shot", game_shot, Priority::Low);
    agent.acmd("effect_shot", effect_shot, Priority::Low);
}