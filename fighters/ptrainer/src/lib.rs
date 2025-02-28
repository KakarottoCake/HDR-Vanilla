#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

//pub mod acmd;

//pub mod opff;
//pub mod status;

pub mod ptrainer;

use smash::{
    lib::{
        L2CValue,
        LuaConst,
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;

pub unsafe fn get_poke_battle_object(boma: *mut BattleObjectModuleAccessor) -> *mut BattleObject {
    let poke_parent_id = LinkModule::get_parent_object_id(boma, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) as u32;
    let poke_object = utils::util::get_battle_object_from_id(poke_parent_id);
    return poke_object;
}

pub unsafe extern "C" fn kill_pledge_effects(poke_object: *mut BattleObject) {
    if poke_object.is_null() {
        return;
    }
    let poke_boma = &mut *(*poke_object).module_accessor;
    if poke_boma.kind() == *FIGHTER_KIND_PZENIGAME {
        let handle = VarModule::get_int(poke_object, vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
        EffectModule::kill(poke_boma, handle, false, false);
        VarModule::set_int(poke_object, vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
    }
    else if poke_boma.kind() == *FIGHTER_KIND_PFUSHIGISOU {
        let handle = VarModule::get_int(poke_object, vars::pfushigisou::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
        EffectModule::kill(poke_boma, handle, false, false);
        VarModule::set_int(poke_object, vars::pfushigisou::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
    }
    else {
        let handle = VarModule::get_int(poke_object, vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
        EffectModule::kill(poke_boma, handle, false, false);
        VarModule::set_int(poke_object, vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
    }
}

pub unsafe extern "C" fn update_pledge_ui(weapon: &mut L2CFighterBase, poke_object: *mut BattleObject) {
    if poke_object.is_null() {
        return;
    }
    let poke_boma = &mut *(*poke_object).module_accessor;
    if !sv_information::is_ready_go() && poke_boma.status_frame() < 1 {
        return;
    }
    let entry_id = poke_boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let pledge_timer = VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
    let swap_timer = VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER);
    let pledge_state = VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
    utils::ui::UiManager::set_ptrainer_meter_enable(entry_id, true);
    utils::ui::UiManager::set_ptrainer_meter_info(
        entry_id,
        pledge_timer as f32,
        900.0,
        swap_timer as f32,
        300.0,
        pledge_state
    );
}

pub fn install() {
    let agent = &mut Agent::new("ptrainer");
    agent.install();

    ptrainer::install();
}