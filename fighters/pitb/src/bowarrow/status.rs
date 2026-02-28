use super::*;

unsafe extern "C" fn fly_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.is_flag(*WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
        return false.into();
    }

    let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor) as u32;
    let owner = util::get_battle_object_from_id(team_owner_id);
    let mut owner_boma = &mut *(*owner).module_accessor;
    let stick_x = owner_boma.stick_x();
    let stick_y = owner_boma.stick_y();
    let magnitude = (stick_x.powi(2) + stick_y.powi(2)).sqrt();
    let angle = weapon.get_float(*WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let mut new_angle = angle;

    if magnitude > 1e-5 {
        let stick_angle_deg = stick_y.atan2(stick_x).to_degrees();
        let mut delta = angle - stick_angle_deg;

        while delta > 180.0  { delta -= 360.0; }
        while delta < -180.0 { delta += 360.0; }

        // when reflected, new owner has increased control over angle
        let control_angle_mul = if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR | *COLLISION_KIND_MASK_PARRY) {
            2.0
        } else {
            1.0
        };
        let control_angle = weapon.get_param_float("param_bowarrow", "control_angle") * control_angle_mul;
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