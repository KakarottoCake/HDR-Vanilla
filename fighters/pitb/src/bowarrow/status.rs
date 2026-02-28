use super::*;

unsafe extern "C" fn fly_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.is_flag(*WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
        return false.into();
    }

    let stick_x = weapon.stick_x();
    let stick_y = weapon.stick_y();
    let magnitude = (stick_x.powi(2) + stick_y.powi(2)).sqrt();
    let angle = weapon.get_float(*WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let mut new_angle = angle;

    // when parried, snap the arrow to one of 8 directions
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR | *COLLISION_KIND_MASK_PARRY) {
        new_angle = (angle / 45.0).round() * 45.0;
        AttackModule::clear_inflict_kind_status(weapon.module_accessor);
    }
    // else, control the arrow direction with the stick
    else if magnitude > 1e-5 {
        let stick_angle_deg = stick_y.atan2(stick_x).to_degrees();
        let mut delta = angle - stick_angle_deg;

        while delta > 180.0  { delta -= 360.0; }
        while delta < -180.0 { delta += 360.0; }

        let control_angle = weapon.get_param_float("param_bowarrow", "control_angle");
        let max_turn = control_angle * magnitude;
        let clamped_delta = delta.clamp(-max_turn, max_turn);
        new_angle = angle - clamped_delta;
    }

    let speed = weapon.get_float(*WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_SPEED);
    let speed_x = speed * new_angle.to_radians().cos();
    let speed_y = speed * new_angle.to_radians().sin();
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    weapon.set_float(speed, *WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_SPEED);
    weapon.set_float(new_angle, *WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_ANGLE);
    PostureModule::set_rot(weapon.module_accessor, &Vector3f::new(-new_angle, 90.0, 0.0), 0);

    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *WEAPON_PIT_BOWARROW_STATUS_KIND_FLY, fly_exec);
}