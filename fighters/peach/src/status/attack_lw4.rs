use super::*;

// FIGHTER_STATUS_KIND_ATTACK_LW4

unsafe extern "C" fn attack_lw4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    AttackModule::set_power_mul_5th(fighter.module_accessor, 1.0);
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_LW4)(fighter)
}


pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_end);
}