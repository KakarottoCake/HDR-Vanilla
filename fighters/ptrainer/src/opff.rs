// opff import
//utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn pledge_timer(fighter: &mut L2CFighterCommon) {
    if VarModule::get_int(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE) != 0 {
        //println!("pledge active!");
        if !VarModule::is_flag(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER)
        && VarModule::countdown_int(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 0) {
            //println!("pledge no longer active");
            VarModule::set_int(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, 0);
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
    pledge_timer(fighter);
}

pub extern "C" fn ptrainer_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
		ptrainer_frame(fighter)
    }
}

pub unsafe fn ptrainer_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, ptrainer_frame_wrapper);
}