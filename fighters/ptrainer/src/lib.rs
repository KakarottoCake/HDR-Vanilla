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

pub unsafe fn get_poke_boma(boma: *mut BattleObjectModuleAccessor) -> &'static mut BattleObjectModuleAccessor {
    let poke_parent_id = LinkModule::get_parent_object_id(boma, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) as u32;
    let poke_object = utils::util::get_battle_object_from_id(poke_parent_id);
    return &mut *(*poke_object).module_accessor;
}

pub fn install() {
    let agent = &mut Agent::new("ptrainer");
    agent.install();

    ptrainer::install();
}