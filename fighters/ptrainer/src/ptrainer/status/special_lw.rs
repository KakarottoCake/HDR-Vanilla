use super::*;

unsafe extern "C" fn special_lw_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if !weapon.is_flag(*WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_FLAG_OUTFIELD_INVISIBLE) {
        VisibilityModule::set_whole(weapon.module_accessor, true);
    }
    let ptrainer = weapon.global_table[0x4].get_ptr() as *mut Weapon;
    if smash::app::WeaponSpecializer_PTrainerPTrainer::request_change_pokemon(ptrainer) != 0 {
        weapon.set_int(*FIGHTER_COMMON_START_KIND_CHANGE, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_START_KIND);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("p_change"), 0.0, 1.0, false, 0.0, false, false);
        weapon.on_flag(*WEAPON_PTRAINER_PTRAINER_STATUS_WORK_FLAG_ON_CHANGE);
        sub_special_lw(weapon);
    }
    else {
        if !weapon.is_motion(Hash40::new("hold")) {
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("hold"), 0.0, 1.0, false, 0.0, false, false);
        }
    }

    weapon.fastshift(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn sub_special_lw(weapon: &mut L2CWeaponCommon) {
    if !weapon.is_situation(*SITUATION_KIND_OUTFIELD) {
        if !weapon.is_flag(*WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_FLAG_MBALL_UPPER) {
            CameraModule::set_whole(weapon.module_accessor, true);
        }
    }
}

unsafe extern "C" fn special_lw_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if MotionModule::is_end(weapon.module_accessor) {
        weapon.change_status(WEAPON_PTRAINER_PTRAINER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }
    if weapon.is_flag(*WEAPON_PTRAINER_PTRAINER_STATUS_WORK_FLAG_VOICE) {
        // no more smash::app::WeaponSpecializer_PtrainerPTrainer::play_voice
        // let poke_parent_id = LinkModule::get_parent_object_id(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) as u32;
        // let poke_object = utils::util::get_battle_object_from_id(poke_parent_id);
        // let poke_boma = &mut *(*poke_object).module_accessor;
        // if poke_boma.kind() == *FIGHTER_KIND_PZENIGAME {
        //     // play randomized voice lines here
        // }
        weapon.off_flag(*WEAPON_PTRAINER_PTRAINER_STATUS_WORK_FLAG_VOICE);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_SPECIAL_LW, special_lw_main);
}