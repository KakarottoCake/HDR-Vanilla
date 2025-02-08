// opff import
//utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe extern "C" fn pledge_timer(weapon: &mut L2CFighterBase) {
    if VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE) != 0 {
        if !VarModule::is_flag(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER)
        && VarModule::countdown_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 0) {
            let poke_parent_id = LinkModule::get_parent_object_id(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) as u32;
            let poke_object = utils::util::get_battle_object_from_id(poke_parent_id);
            let poke_boma = &mut *(*poke_object).module_accessor;
            if poke_boma.kind() == *FIGHTER_KIND_PZENIGAME {
                let handle = VarModule::get_int(poke_object, vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
                EffectModule::kill(poke_boma, handle, false, false);
                VarModule::set_int(poke_object, vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
            }
            else if poke_boma.kind() == *FIGHTER_KIND_PFUSHIGISOU {
                let handle = VarModule::get_int(poke_object, vars::pfushigisou::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
                EffectModule::kill(poke_boma, handle, false, false);
                VarModule::set_int(poke_object, vars::pfushigisou::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
            }
            else {
                let handle = VarModule::get_int(poke_object, vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
                EffectModule::kill(poke_boma, handle, false, false);
                VarModule::set_int(poke_object, vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
            }
            
            VarModule::set_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, 0);
            VarModule::off_flag(weapon.battle_object, vars::ptrainer::instance::DISABLE_SPECIAL_LW);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pledge_timer);
}