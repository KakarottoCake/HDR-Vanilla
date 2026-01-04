use super::*;
use crate::consts::*;
use crate::consts::globals::*;
use std::ffi::{c_void, c_float};

#[skyline::hook(offset = 0x6d2498, inline)]
unsafe fn hitstun_gravity_1(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[19].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
    let hitstun_gravity_min = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_min");
    let hitstun_gravity_max = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_max");

    let hitstun_gravity = air_accel_y.clamp(hitstun_gravity_min, hitstun_gravity_max);

    ctx.registers_f[0].set_s(hitstun_gravity)
}

#[skyline::hook(offset = 0x6d24c0, inline)]
unsafe fn hitstun_fall_speed_1(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[19].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_speed_y_stable = WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0);

    ctx.registers_f[0].set_s(air_speed_y_stable)
}

#[skyline::hook(offset = 0x6c399c, inline)]
unsafe fn hitstun_gravity_2(ctx: &mut skyline::hooks::InlineCtx) {
    println!("DEBUG >>>>>>> hitstun_gravity_2");
    let work_module = ctx.registers[23].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
    let hitstun_gravity_min = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_min");
    let hitstun_gravity_max = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_max");

    let hitstun_gravity = air_accel_y.clamp(hitstun_gravity_min, hitstun_gravity_max);

    ctx.registers_f[0].set_s(hitstun_gravity);

    // DEBUG
    let knockback_info = ctx.registers[22].x() as *const f32;
    print_knockback_info(knockback_info);
}

#[skyline::hook(offset = 0x6c39c4, inline)]
unsafe fn hitstun_fall_speed_2(ctx: &mut skyline::hooks::InlineCtx) {
    println!("DEBUG >>>>>>> hitstun_fall_speed_2");
    let work_module = ctx.registers[23].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_speed_y_stable = WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0);

    ctx.registers_f[0].set_s(air_speed_y_stable);

    // DEBUG
    let knockback_info = ctx.registers[22].x() as *const f32;
    print_knockback_info(knockback_info);
}

#[skyline::hook(offset = 0x6d5920, inline)]
unsafe fn hitstun_gravity_3(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[20].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
    let hitstun_gravity_min = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_min");
    let hitstun_gravity_max = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_max");

    let hitstun_gravity = air_accel_y.clamp(hitstun_gravity_min, hitstun_gravity_max);

    ctx.registers_f[0].set_s(hitstun_gravity)
}

#[skyline::hook(offset = 0x6d5948, inline)]
unsafe fn hitstun_fall_speed_3(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[20].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_speed_y_stable = WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0);
    
    ctx.registers_f[0].set_s(air_speed_y_stable)
}

#[skyline::hook(offset = 0x13e90a0)]
unsafe fn trajectory_manager_hook(
    main_obj: *mut c_void,
    unused_arg: u64,
    boma_ptr: *mut c_void,
    knockback_info: *mut f32, 
) {
    let boma = boma_ptr as *mut BattleObjectModuleAccessor;
    
    let di_angle = WorkModule::get_param_float(boma, hash40("common"), hash40("damage_fly_correction_max"));
    let launch_speed = Vector2f::new(*knockback_info.add(4), *knockback_info.add(5));
    let total_speed = (launch_speed.x.powi(2) + launch_speed.y.powi(2)).sqrt();
    let kb_angle = launch_speed.y.atan2(launch_speed.x).to_degrees();
    let min_di = kb_angle - di_angle;
    let max_di = kb_angle + di_angle;
    let min_launch_speed_x = total_speed * min_di.to_radians().cos();
    let min_launch_speed_y = total_speed * min_di.to_radians().sin();
    let max_launch_speed_x = total_speed * max_di.to_radians().cos();
    let max_launch_speed_y = total_speed * max_di.to_radians().sin();

    let red_line = 40;
    let green_line = 40 + (1 * 8);
    let blue_line = 40 + (2 * 8);

    *knockback_info.add(red_line) = launch_speed.x;
    *knockback_info.add(red_line + 1) = launch_speed.y;

    *knockback_info.add(green_line) = min_launch_speed_x;
    *knockback_info.add(green_line + 1) = min_launch_speed_y;
    
    *knockback_info.add(blue_line) = max_launch_speed_x;
    *knockback_info.add(blue_line + 1) = max_launch_speed_y;

    call_original!(main_obj, unused_arg, boma_ptr, knockback_info);
}

// DEBUG
unsafe fn print_knockback_info(knockback_info: *const f32) {
    let knockback = *knockback_info;
    let hitstun = *knockback_info.add(0x48 / 4);
    let damage = *knockback_info.add(22);
    let sdi_mul = *knockback_info.add(24);
    let launch_radians = *knockback_info.add(0x10);
    let launch_speed = Vector2f::new(*knockback_info.add(4), *knockback_info.add(5));
    let is_tumble = *(knockback_info.add(1) as *const u32) >= 3;

    println!("DEBUG >>>>>>> knockback: {}", knockback);
    println!("DEBUG >>>>>>> hitstun: {}", hitstun);
    println!("DEBUG >>>>>>> damage: {}", damage);
    println!("DEBUG >>>>>>> sdi_mul: {}", sdi_mul);
    println!("DEBUG >>>>>>> launch_radians: {}", launch_radians);
    println!("DEBUG >>>>>>> launch_speed.x: {}", launch_speed.x);
    println!("DEBUG >>>>>>> launch_speed.y: {}", launch_speed.y);
    println!("DEBUG >>>>>>> is_tumble: {}", is_tumble);
    println!("----------")
}

pub fn install() {
    unsafe {
        // Stubs damage_fly_top_air_accel_y and
        // damage_fly_top_speed_y_stable param pulls
        skyline::patching::Patch::in_text(0x6d2498).nop();
        skyline::patching::Patch::in_text(0x6d24c0).nop();
        skyline::patching::Patch::in_text(0x6c399c).nop();
        skyline::patching::Patch::in_text(0x6c39c4).nop();
        skyline::patching::Patch::in_text(0x6d5920).nop();
        skyline::patching::Patch::in_text(0x6d5948).nop();

        // Allows custom hitstun gravity to apply to all knockback angles
        // rather than just vertical angles
        skyline::patching::Patch::in_text(0x6d1f20).nop();
        skyline::patching::Patch::in_text(0x6d1f24).nop();
        skyline::patching::Patch::in_text(0x6d1f48).nop();
        skyline::patching::Patch::in_text(0x6d1f4c).nop();
        skyline::patching::Patch::in_text(0x6cf574).data(0x320003F8);
        skyline::patching::Patch::in_text(0x6d1624).data(0x320003F8);
        skyline::patching::Patch::in_text(0x6c3568).data(0x320003E8);
        skyline::patching::Patch::in_text(0x6c3764).data(0x320003E8);
    }
    skyline::install_hooks!(
        hitstun_gravity_1,
        hitstun_fall_speed_1,
        hitstun_gravity_2,
        hitstun_fall_speed_2,
        hitstun_gravity_3,
        hitstun_fall_speed_3,
        trajectory_manager_hook
    );
}