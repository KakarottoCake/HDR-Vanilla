use super::*;

// unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
//     let rand = sv_math::rand(hash40("weapon"), 90);
//     weapon.set_int(rand, *WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_INT_RANDOM_ROT_Z);
//     PostureModule::set_rot(weapon.module_accessor, &Vector3f::new(0.0, 0.0, rand as f32), 0);
//     weapon.off_flag(*WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_FLAG_FLASH_START);
//     //MotionModule::change_motion(module_accessor, arg2, arg3, arg4, arg5, arg6, arg7, arg8)

//     weapon.fastshift(L2CValue::Ptr(fly_main_loop as *const () as _))
// }

unsafe extern "C" fn fly_check_damage(weapon: &mut L2CWeaponCommon, param_1: &L2CValue) -> L2CValue {
    // let table = param_1.get_table() as *mut smash_rs::lib::L2CTable;
    // if utils::util::get_table_value(table, "object_category_").try_integer().unwrap() as i32 == *BATTLE_OBJECT_CATEGORY_FIGHTER {
    //     let object_id = utils::util::get_table_value(table, "object_id_").try_integer().unwrap() as u32;
    //     let opponent_boma = sv_battle_object::module_accessor(object_id);
    //     let attacker_team_no = TeamModule::hit_team_no(opponent_boma) as i32;
    //     let owner_id = attacker_boma.battle_object_id;
    //     TeamModule::set_team_owner_id(weapon.module_accessor, owner_id);
    //     TeamModule::set_hit_team(weapon.module_accessor, attacker_team_no);
    // }

    return true.into(); // Believe this determines if you actually take knockback or not after checking damage, used in like throws for throw armor
}

pub fn install(agent: &mut Agent) {
    //agent.status(CheckDamage, *WEAPON_MIIGUNNER_GROUNDBOMB_STATUS_KIND_FLY, fly_check_damage);
}