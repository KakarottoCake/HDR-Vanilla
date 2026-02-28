// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe extern "C" fn pitb_bowarrow_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    let boma = weapon.boma();
    if StatusModule::status_kind(boma) == *WEAPON_PIT_BOWARROW_STATUS_KIND_FLY
    && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_PARRY) {
        EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
        let magnitude = weapon.get_float(*WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_SPEED);
        let angle = weapon.get_float(*WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_ANGLE);
        let snapped_angle = (angle / std::f32::consts::FRAC_PI_4).round() * std::f32::consts::FRAC_PI_4;
        let new_speed_x = magnitude * snapped_angle.cos();
        let new_speed_y = magnitude * snapped_angle.sin();
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, new_speed_x, new_speed_y);
        weapon.set_float(magnitude, *WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_SPEED);
        weapon.set_float(snapped_angle, *WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_ANGLE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pitb_bowarrow_frame);
}
