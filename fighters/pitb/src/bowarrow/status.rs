use super::*;

unsafe extern "C" fn fly_check_attack(weapon: &mut L2CWeaponCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    println!("running");
    // force knockdown
    if (&param_3["kind_"]).get_i32() == *COLLISION_KIND_SHIELD
    // && AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_PARRY) 
    {
        EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
        println!("hit a shield");
        let speed_x = weapon.get_speed_x(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        let speed_y = weapon.get_speed_y(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        let magnitude = speed_x.hypot(speed_y);
        let angle = speed_y.atan2(speed_x);
        let snapped_angle = (angle / (std::f32::consts::PI / 4.0)).round() * (std::f32::consts::PI / 4.0);
        let new_speed_x = magnitude * snapped_angle.cos();
        let new_speed_y = magnitude * snapped_angle.sin();
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, new_speed_x, new_speed_y);
    }
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(CheckAttack, *WEAPON_PIT_BOWARROW_STATUS_KIND_FLY, fly_check_attack);
}