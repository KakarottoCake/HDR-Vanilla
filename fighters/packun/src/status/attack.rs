use super::*;

unsafe extern "C" fn attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_S3)(fighter);

    if fighter.is_motion(Hash40::new("attack_13"))
    && VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 1 {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_100, false);
        return 0.into();
    }

    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK, attack_main);
}