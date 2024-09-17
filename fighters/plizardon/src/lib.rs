#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

mod rock;
mod rockstone;

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
#[macro_use] extern crate smash_script;

pub fn install() {
    let agent = &mut Agent::new("plizardon");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    rock::install();
    rockstone::install();

    smashline::clone_weapon("link", *WEAPON_KIND_LINK_BOOMERANG, "plizardon", "rock", false);
    smashline::clone_weapon("sheik", *WEAPON_KIND_SHEIK_NEEDLE, "plizardon", "rockstone", false);
}