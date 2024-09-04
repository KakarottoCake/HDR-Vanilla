#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod autoaimbullet;
mod explosiveflame;
mod reflectionboard;
mod meteor;

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

pub const FIGHTER_PALUTENA_GENERATE_ARTICLE_METEOR: i32 = articles::palutena::METEOR;
pub const WEAPON_PALUTENA_METEOR_STATUS_KIND_MOVE: i32 = statuses::palutena_meteor::MOVE;

pub fn install() {
    let agent = &mut Agent::new("palutena");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    autoaimbullet::install();
    explosiveflame::install();
    reflectionboard::install();

    meteor::install();
    smashline::clone_weapon("ryu", "hadoken", "palutena", "meteor", false);
}