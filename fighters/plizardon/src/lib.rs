#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub const FIGHTER_PLIZARDON_GENERATE_ARTICLE_LAST: i32 = 2;
pub static mut FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCK: i32 = FIGHTER_PLIZARDON_GENERATE_ARTICLE_LAST+1;
//pub const ROCK_STATUS_KIND_START: i32 = 0;
pub static mut FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCKSTONE: i32 = FIGHTER_PLIZARDON_GENERATE_ARTICLE_LAST+2;
//pub const ROCKSTONE_STATUS_KIND_START: i32 = 0;
//pub const ROCKSTONE_STATUS_KIND_MOVE: i32 = 2;

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

pub trait PokeExt {
    unsafe fn play_pledge_effect(&mut self, state: i32);
}
impl PokeExt for app::BattleObjectModuleAccessor {
    unsafe fn play_pledge_effect(&mut self, state: i32) {
        match state {
            1 /* WATER */ => {
                let water_fx = EffectModule::req_follow(self, Hash40::new("sys_water_landing"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(self, water_fx, 0.2, 0.55, 1.0);
                EffectModule::set_scale(self, water_fx, &Vector3f::new(0.7, 1.0, 0.7));
                EffectModule::set_rate(self, water_fx, 0.7);
            }
            2 /* GRASS */ => {
                for _ in 0..2 {
                    let grass_fx = EffectModule::req_follow(self, Hash40::new("sys_grass_landing"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(self, grass_fx, 0.5, 2.0, 0.5);
                    EffectModule::set_scale(self, grass_fx, &Vector3f::new(1.3, 2.2, 1.3));
                    EffectModule::set_rate(self, grass_fx, 0.6);
                }
            }
            _ => println!("Invalid pledge state provided.")
        }
    }
}

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