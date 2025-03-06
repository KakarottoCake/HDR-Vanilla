use super::*;
use globals::*;
// status script import
 
mod attack_s4;
mod special_hi;
mod special_lw;

unsafe extern "C" fn fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_AIR_LASSO_HANG, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND]) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_FALL)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_FALL, fall_main);

    attack_s4::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
}