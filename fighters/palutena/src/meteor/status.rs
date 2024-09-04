use super::*;

unsafe extern "C" fn move_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.init()
}

unsafe extern "C" fn move_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let angle: f32 = 75.0;
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let palutena = utils::util::get_battle_object_from_id(owner_id);
    let palutena_boma = &mut *(*palutena).module_accessor;
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_meteor"), hash40("life_s"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_meteor"), hash40("speed_s"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let owner_pos_x = PostureModule::pos_x(palutena_boma);
    let owner_pos_y = PostureModule::pos_y(palutena_boma);
    let owner_pos_z = PostureModule::pos_z(palutena_boma);
    let speed_y = angle.to_radians().cos()*speed_max;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    ModelModule::set_scale(weapon.module_accessor, 1.7);
    weapon.clear_lua_stack();
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, -speed_y/4.5);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr, -speed_y/4.5);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+5.0*lr, y: owner_pos_y+17.0, z: owner_pos_z});
    0.into()
}

unsafe extern "C" fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(move_main_loop as *const () as _))
}

unsafe extern "C" fn move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = weapon.global_table[PREV_SITUATION_KIND].get_i32();
    let pos = *PostureModule::pos(weapon.module_accessor);
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    || life <= 0
    || (situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR) {
        WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z+5.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
    }
    0.into()
}

unsafe extern "C" fn move_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn move_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, WEAPON_PALUTENA_METEOR_STATUS_KIND_MOVE, move_pre);
    agent.status(Init, WEAPON_PALUTENA_METEOR_STATUS_KIND_MOVE, move_init);
    agent.status(Main, WEAPON_PALUTENA_METEOR_STATUS_KIND_MOVE, move_main);
    agent.status(Exec, WEAPON_PALUTENA_METEOR_STATUS_KIND_MOVE, move_exec);
    agent.status(End, WEAPON_PALUTENA_METEOR_STATUS_KIND_MOVE, move_end);
}