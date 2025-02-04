use super::*;

unsafe extern "C" fn special_n_loop_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_status_kind_interrupt(*FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N_END);
    return 1.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N_LOOP, special_n_loop_pre);
}