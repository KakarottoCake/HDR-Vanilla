use super::*;
use globals::*;

mod attack;
mod batwithin;
mod escape;
mod attackair;
mod specialairs;
mod specialn;
mod specials;
mod specialhi;
mod jumpaerial;
mod wait;

/// Shield gives 2nd ABK
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) < 2
    && VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT) < 2 
    && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::SPECIAL_S_WHIFF) {
        true.into()
    } else {
        false.into()
    }
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack::install(agent);
    batwithin::install(agent);
    escape::install(agent);
    attackair::install(agent);
    specialairs::install(agent);
    specialn::install(agent);
    specials::install(agent);
    specialhi::install(agent);
    jumpaerial::install(agent);
    wait::install(agent);
}