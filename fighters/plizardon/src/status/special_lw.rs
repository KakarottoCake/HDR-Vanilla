use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_LW

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);

    return ret;
}

unsafe extern "C" fn special_lw_out_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let parent_id = LinkModule::get_parent_id(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
    let object = utils::util::get_battle_object_from_id(parent_id);
    let pledge_state = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
    //println!("pledge_state: {}", pledge_state);
    if pledge_state == *PLEDGE_STATE_WATER {
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.object(), vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, handle as i32);
        let water_fx = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_water_landing"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(fighter.module_accessor, water_fx, 0.2, 0.55, 1.0);
        EffectModule::set_scale(fighter.module_accessor, water_fx, &Vector3f::new(0.7, 1.0, 0.7));
        EffectModule::set_rate(fighter.module_accessor, water_fx, 0.7);
    }
    else if pledge_state == *PLEDGE_STATE_GRASS {
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_speed_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.object(), vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, handle as i32);
        for _ in 0..2 {
            let grass_fx = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_grass_landing"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(fighter.module_accessor, grass_fx, 0.5, 2.0, 0.5);
            EffectModule::set_scale(fighter.module_accessor, grass_fx, &Vector3f::new(1.3, 2.2, 1.3));
            EffectModule::set_rate(fighter.module_accessor, grass_fx, 0.6);
        }
    }
    smashline::original_status(Main, fighter, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_LW_OUT)(fighter)
}

unsafe extern "C" fn special_lw_out_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let parent_id = LinkModule::get_parent_id(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
    let object = utils::util::get_battle_object_from_id(parent_id);
    VarModule::off_flag(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER);
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);

    agent.status(Main, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_LW_OUT, special_lw_out_main);
    agent.status(End, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_LW_OUT, special_lw_out_end);
}