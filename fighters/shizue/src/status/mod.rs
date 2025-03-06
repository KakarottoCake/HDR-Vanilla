use super::*;
use globals::*;
// status script import

mod dead;
mod special_hi;
mod special_lw;
mod special_s;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.object(), vars::shizue::instance::SPECIAL_LW_LLOID_ASYNC);
    VarModule::set_int(fighter.object(), vars::shizue::instance::SPECIAL_LW_LLOID_TIMER, 0);
}

unsafe extern "C" fn fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_AIR_LASSO_HANG, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND]) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_FALL)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    agent.status(Main, *FIGHTER_STATUS_KIND_FALL, fall_main);

    dead::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    special_s::install(agent);
}