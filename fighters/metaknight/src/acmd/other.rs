use super::*;

unsafe extern "C" fn sound_damagefly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(boma) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_metaknight_rnd_futtobi01"), Hash40::new("seq_metaknight_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(agent, Hash40::new("seq_metaknight_rnd_futtobi01"), Hash40::new("seq_metaknight_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn sound_damageflyroll(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !StopModule::is_stop(boma) {
            PLAY_FLY_VOICE(agent, Hash40::new("seq_metaknight_rnd_futtobi01"), Hash40::new("seq_metaknight_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(agent) {
        PLAY_FLY_VOICE(agent, Hash40::new("seq_metaknight_rnd_futtobi01"), Hash40::new("seq_metaknight_rnd_futtobi02"));
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let dash_sfx_handle = SoundModule::play_se(boma, Hash40::new("se_metaknight_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_metaknight_step_right_l"));
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_metaknight_step_left_l"));
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, 29.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    
    frame(lua_state, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn sound_metaquicksummon(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        // plays the sword slash effect
        PLAY_SE(fighter, Hash40::new("se_metaknight_final01"));

        if VarModule::is_flag(fighter.battle_object, vars::metaknight::instance::META_QUICK_PLAY_VC) {
            PLAY_SE(fighter, Hash40::new("vc_metaknight_final02"));
        }
    }
}

unsafe extern "C" fn effect_metaquicksummon(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        VarModule::set_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_CHARGE_EFFECT_HANDLE, -1);
        lua_args! {
            fighter,
            Hash40::new("sys_bg_black"),
            0,
            0,
            0,
            0,
            0,
            0,
            1
        };
        smash::app::sv_animcmd::EFFECT_GLOBAL_BACK_GROUND(fighter.lua_state_agent);
        let handle = EffectModule::req_follow(
            fighter.boma(),
            Hash40::new("sys_aura_light"),
            Hash40::new("head"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            6.0,
            true,
            0,
            0,
            0,
            0,
            0,
            true,
            true
        ) as u32;

        let handle2 = EffectModule::req_follow(
            fighter.boma(),
            Hash40::new("sys_aura_light"),
            Hash40::new("head"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            6.0,
            true,
            0,
            0,
            0,
            0,
            0,
            true,
            true
        ) as u32;

        EffectModule::set_alpha(fighter.module_accessor, handle, 1.0);
        EffectModule::set_rgb(fighter.module_accessor, handle, 101.0 / 255.0, 32.0 / 255.0, 153.0 / 255.0);
        EffectModule::set_alpha(fighter.module_accessor, handle2, 1.0);
        EffectModule::set_rgb(fighter.module_accessor, handle2, 101.0 / 255.0, 32.0 / 255.0, 153.0 / 255.0);
        VarModule::set_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_EFFECT_HANDLE, handle as i32);
        VarModule::set_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_EFFECT_HANDLE2, handle2 as i32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffescape", acmd_stub, Priority::Low);

    agent.acmd("sound_damageflyhi", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflylw", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyn", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflytop", sound_damagefly, Priority::Low);
    agent.acmd("sound_damageflyroll", sound_damageflyroll, Priority::Low);
    
    agent.acmd("game_dash", game_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("game_turndash", game_turndash, Priority::Low);
    
    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("effect_metaquicksummon", effect_metaquicksummon, Priority::Low);
    agent.acmd("sound_metaquicksummon", sound_metaquicksummon, Priority::Low);
}
