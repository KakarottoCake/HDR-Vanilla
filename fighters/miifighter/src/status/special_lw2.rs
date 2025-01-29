use super::*;

pub unsafe extern "C" fn special_lw2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }

    return 0.into();
}