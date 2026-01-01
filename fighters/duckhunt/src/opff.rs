// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn gunman_timer(fighter: &mut L2CFighterCommon) {
    if VarModule::countdown_int(fighter.battle_object, vars::duckhunt::instance::SPECIAL_LW_GUNMAN_TIMER, 0) {
        gimmick_flash(fighter);
    }
}

pub extern "C" fn duckhunt_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        gunman_timer(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, duckhunt_frame_wrapper);
}