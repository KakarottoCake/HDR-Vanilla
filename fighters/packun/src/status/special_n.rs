use super::*;

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    
    if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 {
        VarModule::set_float(fighter.battle_object, vars::packun::instance::SPECIAL_N_PTOOIE_SCALE, 1.3);
    }
    else {
        VarModule::set_float(fighter.battle_object, vars::packun::instance::SPECIAL_N_PTOOIE_SCALE, 1.0);
    }

    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
}