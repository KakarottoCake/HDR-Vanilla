// opff import
//utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe extern "C" fn kill_pledge_effects(poke_boma: &mut BattleObjectModuleAccessor) {
    if poke_boma.kind() == *FIGHTER_KIND_PZENIGAME {
        let handle = VarModule::get_int(poke_boma.object(), vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
        EffectModule::kill(poke_boma, handle, false, false);
        VarModule::set_int(poke_boma.object(), vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
    }
    else if poke_boma.kind() == *FIGHTER_KIND_PFUSHIGISOU {
        let handle = VarModule::get_int(poke_boma.object(), vars::pfushigisou::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
        EffectModule::kill(poke_boma, handle, false, false);
        VarModule::set_int(poke_boma.object(), vars::pfushigisou::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
    }
    else {
        let handle = VarModule::get_int(poke_boma.object(), vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE) as u32;
        EffectModule::kill(poke_boma, handle, false, false);
        VarModule::set_int(poke_boma.object(), vars::plizardon::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, -1);
    }
}

pub unsafe extern "C" fn pledge_meter(weapon: &mut L2CFighterBase) {
    let poke_boma = get_poke_boma(weapon.module_accessor);
    let pledge_state = VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
    let is_pledge_timer_paused = VarModule::is_flag(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER);
    if pledge_state != *PLEDGE_STATE_NONE && !is_pledge_timer_paused {
        if VarModule::countdown_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 0) {
            kill_pledge_effects(poke_boma);
            VarModule::set_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, *PLEDGE_STATE_NONE);
        }
    }
    if VarModule::countdown_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER, 0) {
        VarModule::off_flag(weapon.battle_object, vars::ptrainer::instance::DISABLE_SPECIAL_LW);
    }

    // Handle UI for pledge state
    if !sv_information::is_ready_go() && poke_boma.status_frame() < 1 {
        return;
    }
    let entry_id = poke_boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let pledge_timer = VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
    let swap_timer = VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER);
    let pledge_state = VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
    utils::ui::UiManager::set_ptrainer_meter_enable(entry_id, true);
    utils::ui::UiManager::set_ptrainer_meter_info(
        dbg!(entry_id),
        dbg!(pledge_timer as f32),
        900.0,
        dbg!(swap_timer as f32),
        300.0,
        dbg!(pledge_state)
    );
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pledge_meter);
}