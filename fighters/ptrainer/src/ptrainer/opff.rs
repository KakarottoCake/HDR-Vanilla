// opff import
//utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe extern "C" fn pledge_timer(weapon: &mut L2CFighterBase) {
    if VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE) != 0 {
        if !VarModule::is_flag(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER)
        && VarModule::countdown_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 0) {
            VarModule::set_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, 0);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pledge_timer);
}