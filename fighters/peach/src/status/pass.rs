use super::*;

unsafe extern "C" fn pass_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Pass_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(pass_main_loop as *const () as _))
}

unsafe extern "C" fn pass_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_cat_flag(Cat1::SpecialLw)
    && ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        return 0.into();
    }
    fighter.status_Pass_Main_sub(L2CValue::Ptr(pass_main_subfunction as *const () as _))
}

unsafe extern "C" fn pass_main_subfunction(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.end_pass_ground()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_PASS, pass_main);
}