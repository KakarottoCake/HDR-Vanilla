use super::*;

pub unsafe extern "C" fn special_n_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::remove_all_after_image(fighter.module_accessor, 0, 0);
    return smashline::original_status(End, fighter, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END)(fighter)
}

pub unsafe extern "C" fn special_n_end2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::remove_all_after_image(fighter.module_accessor, 0, 0);
    return smashline::original_status(End, fighter, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2)(fighter)
}

pub unsafe extern "C" fn special_n_end3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::remove_all_after_image(fighter.module_accessor, 0, 0);
    return smashline::original_status(End, fighter, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3)(fighter)
}

pub unsafe extern "C" fn special_n_end_max_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::remove_all_after_image(fighter.module_accessor, 0, 0);
    return smashline::original_status(End, fighter, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END, special_n_end_end);
    agent.status(End, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2, special_n_end2_end);
    agent.status(End, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3, special_n_end3_end);
    agent.status(End, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX, special_n_end_max_end);
}