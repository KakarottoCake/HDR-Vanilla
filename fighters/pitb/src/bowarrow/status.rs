use super::*;

unsafe extern "C" fn fly_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.is_flag(*WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
        return false.into();
    }

    // use get_team_owner_boma to allow the arrow to be controlled by opponent after reflecting it
    let weapon_boma = weapon.module_accessor;
    let owner_boma = weapon.get_team_owner_boma();
    let (stick_mag, stick_rad) = owner_boma.stick_polar();

    // adjust arrow angle according to stick
    let arrow_deg = weapon.get_float(*WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let arrow_delta = if stick_mag > 1e-5 {
        let mut delta = arrow_deg - stick_rad.to_degrees();
        while delta > 180.0  { delta -= 360.0; }
        while delta < -180.0 { delta += 360.0; }

        let was_reflected = AttackModule::is_infliction_status(weapon_boma, *COLLISION_KIND_MASK_REFLECTOR | *COLLISION_KIND_MASK_PARRY);
        let control_angle_mul = if was_reflected { 2.0 } else { 1.0 }; // when reflected, new owner has increased control over angle
        let control_angle = weapon.get_param_float("param_bowarrow", "control_angle") * control_angle_mul;
        let max_turn = control_angle * stick_mag;

        delta.clamp(-max_turn, max_turn)
    } else {
        0.0
    };

    // apply new velocity vector
    let new_deg = arrow_deg - arrow_delta;
    let speed = weapon.get_float(*WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_SPEED);
    let speed_x = speed * new_deg.to_radians().cos();
    let speed_y = speed * new_deg.to_radians().sin();
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    weapon.set_float(speed, *WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_SPEED);
    weapon.set_float(new_deg, *WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_ANGLE);
    PostureModule::set_rot(weapon_boma, &Vector3f::new(-new_deg, 90.0, 0.0), 0);

    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *WEAPON_PIT_BOWARROW_STATUS_KIND_FLY, fly_exec);
}