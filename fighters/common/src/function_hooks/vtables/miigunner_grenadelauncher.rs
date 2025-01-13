use super::*;
use utils::ext::*;
use super::super::energy::PaddedVec2;

#[skyline::hook(offset = 0x3453b10)]
unsafe extern "C" fn miigunner_grenadelauncher_init(vtable: u64, weapon: &mut smash::app::Weapon, init_struct: u64) {
    let module_accessor = weapon.battle_object.module_accessor;
    let speed = WorkModule::get_param_float(module_accessor, hash40("param_grenadelauncher"), hash40("speed"));
    let mul = *(init_struct as *const f32).add(0x88 / 0x4);
    let speed = speed * mul;
    let angle = WorkModule::get_param_float(module_accessor, hash40("param_grenadelauncher"), hash40("angle"));
    let lr = PostureModule::lr(module_accessor);
    let energy = KineticModule::get_energy(module_accessor, 0) as *mut super::super::energy::KineticEnergy;
    let gravity_acl_max = WorkModule::get_param_float(module_accessor, hash40("param_grenadelauncher"), hash40("gravity_acl_max"));
    let gravity_accel = WorkModule::get_param_float(module_accessor, hash40("param_grenadelauncher"), hash40("gravity_accel"));
    let cos = angle.to_radians().cos();
    let sin = angle.to_radians().sin();

    (*energy).speed = PaddedVec2::new(speed * lr * cos, speed * sin);
    (*energy).speed_limit = PaddedVec2::new(-1.0, gravity_acl_max);
    (*energy).speed_max = PaddedVec2::new(0.0, 0.0);
    let brake_x = WorkModule::get_param_float(module_accessor, hash40("param_grenadelauncher"), hash40("brake_x"));
    (*energy).speed_brake = PaddedVec2::new(brake_x, 0.0);
    (*energy).accel = PaddedVec2::new(0.0, -gravity_accel);
}

pub fn install() {
    skyline::install_hooks!(
        miigunner_grenadelauncher_init
    );
