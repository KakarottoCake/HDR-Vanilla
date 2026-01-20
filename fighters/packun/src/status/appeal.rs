use super::*;

unsafe extern "C" fn appeal_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_motion(Hash40::new("appeal_hi_2"))
    && fighter.motion_frame() == 92.0
    && fighter.is_button_on(Buttons::AppealSL) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_2"), 45.0, 1.0, false, 0.0, false, false);
    }
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_APPEAL, appeal_exec);
}