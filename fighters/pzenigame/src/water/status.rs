use super::*;
use globals::*;

// pub unsafe extern "C" fn regular_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
//     MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
//     if !StopModule::is_stop(weapon.module_accessor) {
//         WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
//     }
//     weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(regular_substatus as *const () as _));
//     regular_func(weapon);

//     weapon.fastshift(L2CValue::Ptr(regular_main_loop as *const () as _))
// }

// pub unsafe extern "C" fn regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
//     if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
//         println!("clash!");
//         weapon.change_status(statuses::pzenigame_water::CLASH_PLEDGE.into(), false.into());
//         return 0.into();
//     }
//     if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) == 0 {
//         println!("die!");
//         weapon.change_status(WEAPON_PZENIGAME_WATER_STATUS_KIND_DIE.into(), false.into());
//     }
//     regular_func(weapon);

//     return 0.into();
// }

// pub unsafe extern "C" fn regular_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
//     if param_1.get_bool() {
//         WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
//     }

//     return 0.into();
// }

// pub unsafe extern "C" fn regular_func(weapon: &mut L2CWeaponCommon) {
//     weapon.clear_lua_stack();
//     weapon.push_lua_stack(&mut L2CValue::I32(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL));
//     let speed_x = app::sv_kinetic_energy::get_speed_x(weapon.lua_state_agent);
//     weapon.clear_lua_stack();
//     weapon.push_lua_stack(&mut L2CValue::I32(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL));
//     let speed_y = app::sv_kinetic_energy::get_speed_y(weapon.lua_state_agent);
//     //println!("speed_x: {}", speed_x);
//     //println!("speed_y: {}", speed_y);
//     let facing = weapon.lr();
//     let angle = speed_y.atan2(speed_x * facing).to_degrees();
//     //println!("setting angle to {}", angle);
//     PostureModule::set_rot(weapon.module_accessor, &Vector3f::new(angle, 0.0, 0.0), 0);
// }

pub unsafe extern "C" fn regular_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //let life = weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    //weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    //let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = weapon.get_owner_boma();//&mut *(*utils::util::get_battle_object_from_id(owner_id)).module_accessor;
    if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PLIZARDON].contains(&owner_boma.kind()) {
        println!("owner is a pokemon, we can set the pledge state properly");
        let parent_id = LinkModule::get_parent_id(owner_boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        let pledge_state = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
        VarModule::set_int(weapon.battle_object, vars::pzenigame_water::instance::PLEDGE_TYPE, pledge_state);
    }
    else {
        println!("ERROR: owner is not a Pokemon, things will probably crash without this failsafe");
        VarModule::set_int(weapon.battle_object, vars::pzenigame_water::instance::PLEDGE_TYPE, 2);
    }
    
    return 0.into();
}

unsafe extern "C" fn clash_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn clash_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //weapon.set_int(4, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pledge_type = VarModule::get_int(weapon.battle_object, vars::pzenigame_water::instance::PLEDGE_TYPE);
    let motion = if pledge_type == 2 { Hash40::new("clash_pledge_g") } else { Hash40::new("clash_pledge_f") };
    MotionModule::change_motion(weapon.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);

    // if !StopModule::is_stop(weapon.module_accessor) {
    //     weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    // }
    // weapon.global_table[globals::SUB_STATUS].assign(&L2CValue::Ptr(clash_main_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(clash_main_loop as *const () as _))
}

unsafe extern "C" fn clash_main_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }

    return 0.into();
}

pub unsafe extern "C" fn clash_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
    //     notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    // }

    return 0.into();
}

unsafe extern "C" fn clash_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *WEAPON_PZENIGAME_WATER_STATUS_KIND_REGULAR, regular_init);

    agent.status(Pre, *WEAPON_PZENIGAME_WATER_STATUS_KIND_CLASH, clash_pre);
    agent.status(Main, *WEAPON_PZENIGAME_WATER_STATUS_KIND_CLASH, clash_main);
    agent.status(End, *WEAPON_PZENIGAME_WATER_STATUS_KIND_CLASH, clash_end);
}