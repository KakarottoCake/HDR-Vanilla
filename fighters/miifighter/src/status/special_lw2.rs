use super::*;

pub unsafe extern "C" fn special_lw2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.check_hold_input(*CONTROL_PAD_BUTTON_SPECIAL);
    if !VarModule::is_flag(fighter.battle_object, vars::common::status::CHECK_HOLD_INPUT) {
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }

    return 0.into();
}

unsafe extern "C" fn special_lw2_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return original_status(Main, fighter, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START)(fighter);
}

pub fn install (agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START, special_lw2_start_main);
}