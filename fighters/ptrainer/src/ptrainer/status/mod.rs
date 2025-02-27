use super::*;
use globals::*;
// status script import

mod special_lw;

pub unsafe fn GET_POKE_BOMA(boma: *mut BattleObjectModuleAccessor) -> &'static mut BattleObjectModuleAccessor {
    let poke_parent_id = LinkModule::get_parent_object_id(boma, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) as u32;
    let poke_object = utils::util::get_battle_object_from_id(poke_parent_id);
    return &mut *(*poke_object).module_accessor;
}

unsafe extern "C" fn on_start(weapon: &mut L2CWeaponCommon) {
    VarModule::off_flag(weapon.battle_object, vars::ptrainer::instance::DISABLE_SPECIAL_LW);
    VarModule::set_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_lw::install(agent);
}