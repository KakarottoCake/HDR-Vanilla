use super::*;
use globals::*;

pub unsafe extern "C" fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = &mut *(*utils::util::get_battle_object_from_id(owner_id)).module_accessor;
    let stick_x = owner_boma.stick_x();
    KineticModule::clear_speed_all(weapon.module_accessor);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 1.0 * stick_x, 3.0);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -0.05);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(move_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(move_main_loop as *const () as _))
}

pub unsafe extern "C" fn move_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }

    return 0.into();
}

pub unsafe extern "C" fn move_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = &mut *(*utils::util::get_battle_object_from_id(owner_id)).module_accessor;
    if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PLIZARDON].contains(&owner_boma.kind()) {
        //println!("owner is a pokemon, we can set the pledge state properly");
        let parent_id = LinkModule::get_parent_id(owner_boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        let pledge_state = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
        VarModule::set_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE, pledge_state);
    }
    else {
        //println!("ERROR: owner is not a Pokemon, things will probably crash without this failsafe");
        VarModule::set_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE, 2);
    }
    
    return 0.into();
}

pub unsafe extern "C" fn move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.change_status(WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH_GROUND.into(), false.into());
        return 0.into();
    }
    if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) == 0 {
        if VarModule::get_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE) == 3 {
            weapon.change_status(WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH.into(), false.into());
            return 0.into();
        }
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }

    return 0.into();
}

// unsafe extern "C" fn clash_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
//     StatusModule::init_settings(
//         weapon.module_accessor,
//         SituationKind(*SITUATION_KIND_NONE),
//         *WEAPON_KINETIC_TYPE_RESET,
//         *GROUND_CORRECT_KIND_NONE as u32,
//         GroundCliffCheckKind(0),
//         false,
//         0,
//         0,
//         0,
//         0
//     );

//     return 0.into();
// }

pub unsafe extern "C" fn clash_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let pledge_type = VarModule::get_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE);
    let motion = if pledge_type == 1 { Hash40::new("clash_pledge_w") } else { Hash40::new("clash_pledge_f") };
    MotionModule::change_motion(weapon.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(clash_main_loop as *const () as _))
}

// unsafe extern "C" fn clash_main_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
//     if !param_1.get_bool() {
//         weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
//     }

//     return 0.into();
// }

pub unsafe extern "C" fn clash_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
    //     notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    // }

    return 0.into();
}

// unsafe extern "C" fn clash_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
//     EffectModule::detach_kind(weapon.module_accessor, Hash40::new("pfushigisou_tanemg_tama"), 5);
//     return 0.into();
// }

pub unsafe extern "C" fn clash_ground_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::get_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE) == 3 {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("clash_pledge_f"), 0.0, 1.0, false, 0.0, false, false);
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_bomb_a"), &Vector3f::zero(), &Vector3f::zero(), 1.0, 0, -1, false, 0);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("clash_ground"), 0.0, 1.0, false, 0.0, false, false);
        EffectModule::req(weapon.module_accessor, Hash40::new("pfushigisou_tanemg_hit"), &Vector3f::zero(), &Vector3f::zero(), 1.0, 0, -1, false, 0);
    }
    VisibilityModule::set_whole(weapon.module_accessor, false);

    weapon.fastshift(L2CValue::Ptr(clash_ground_main_loop as *const () as _))
}

pub unsafe extern "C" fn clash_ground_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::get_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE) != 3 {
        if weapon.global_table[CURRENT_FRAME].get_i32() == 2 {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        }
    }
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_MOVE, move_main);
    agent.status(Init, *WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_MOVE, move_init);

    agent.status(Main, *WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH, clash_main);

    agent.status(Main, *WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH_GROUND, clash_ground_main);
}