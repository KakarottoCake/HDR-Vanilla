use super::*;

unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((agent.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(agent) {
        let lr = PostureModule::lr(owner_module_accessor);
        KineticModule::reflect_speed(boma, &Vector3f{x: 0.66262, y: lr * 0.90631, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 82, 30, 0, 67, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_palutena_bullet"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

unsafe extern "C" fn effect_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("elight_lay_dust"), Hash40::new("top"), 0, 0, 0, -90, 0, 0, 0.65, false);
        LAST_EFFECT_SET_COLOR(agent, 0.75, 0.01, 0.35);
    }
}

unsafe extern "C" fn sound_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_n02"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_move", game_move, Priority::Low);
    agent.acmd("effect_move", effect_move, Priority::Low);
    agent.acmd("sound_move", sound_move, Priority::Low);
}