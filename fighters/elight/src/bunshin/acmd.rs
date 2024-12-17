use super::*;

unsafe extern "C" fn game_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    if IS_EXIST_ARTICLE(agent, *WEAPON_ELIGHT_BUNSHIN_GENERATE_ARTICLE_ESWORD) {
        if is_excute(agent) {
            ArticleModule::add_motion_partial(
                boma,
                *WEAPON_ELIGHT_BUNSHIN_GENERATE_ARTICLE_ESWORD,
                *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE,
                Hash40::new("to_open"),
                10.0,
                10.0,
                false,
                false,
                0.0,
                false,
                true,
                false
            );
        }
    }
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 17.0 / 11.0);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 21.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
}

unsafe extern "C" fn game_specials2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    if IS_EXIST_ARTICLE(agent, *WEAPON_ELIGHT_BUNSHIN_GENERATE_ARTICLE_ESWORD) {
        if is_excute(agent) {
            ArticleModule::add_motion_partial(
                boma,
                *WEAPON_ELIGHT_BUNSHIN_GENERATE_ARTICLE_ESWORD,
                *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE,
                Hash40::new("to_open"),
                10.0,
                10.0,
                false,
                false,
                0.0,
                false,
                true,
                false
            );
        }
    }
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 15.0 / 9.0);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, true);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 19.0);
    if is_excute(agent) {
        VisibilityModule::set_whole(boma, false);
    }
}

unsafe extern "C" fn effect_specials1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    effect_specials_inner(agent, 0.0);
}

unsafe extern "C" fn effect_specials2(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    effect_specials_inner(agent, -2.0);
}

unsafe extern "C" fn effect_specials_inner(agent: &mut L2CAgentBase, offset_frame: f32) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0 + offset_frame);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("elight_photon_body_lihgt"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("elight_photon_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -1);
        EFFECT(agent, Hash40::new("elight_photon_appear"), Hash40::new("hip"), 0, 0, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0 + offset_frame);
    if is_excute(agent) {
        // EFFECT(agent, Hash40::new("elight_photon_speedline"), Hash40::new("throw"), 0, 0, 0, 0, -24, -43, 0.4, 0, 0, 0, 0, 0, 0, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_elight_sword4"), Hash40::new("tex_elight_sword2"), 5, Hash40::new("sword1"), 0.0, 0.0, -0.08, Hash40::new("sword1"), 19.5, 0.0, -0.08, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(lua_state, 16.0 + offset_frame);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 18.0 + offset_frame);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("elight_photon_vanish"), Hash40::new("hip"), 0, 0, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_photon_body_lihgt"), false, true);
        EFFECT_OFF_KIND(agent, Hash40::new("elight_photon_sword"), true, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials1", game_specials1, Priority::Low);
    agent.acmd("effect_specials1", effect_specials1, Priority::Low);
    
    agent.acmd("game_specialairs1", game_specials1, Priority::Low);
    agent.acmd("effect_specialairs1", effect_specials1, Priority::Low);

    agent.acmd("game_specials2", game_specials2, Priority::Low);
    agent.acmd("effect_specials2", effect_specials2, Priority::Low);
    
    agent.acmd("game_specialairs2", game_specials2, Priority::Low);
    agent.acmd("effect_specialairs2", effect_specials2, Priority::Low);
}
