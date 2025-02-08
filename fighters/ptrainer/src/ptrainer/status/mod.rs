use super::*;
use globals::*;
// status script import

mod special_lw;

unsafe extern "C" fn on_start(weapon: &mut L2CWeaponCommon) {
    VarModule::off_flag(weapon.battle_object, vars::ptrainer::instance::DISABLE_SPECIAL_LW);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_lw::install(agent);
}