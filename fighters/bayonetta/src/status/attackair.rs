use super::*;

// FIGHTER_STATUS_KIND_ATTACK_AIR

unsafe extern "C" fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    if fighter.global_table[CURRENT_FRAME].get_i32() as f32 <= fighter.get_param_float("param_special_hi", "jump_count_reset_frame") {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("bayonetta_feather_twinkle"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.8, true, 0, 0, 0, 0, 0, false, false);
    }
    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter)
}

// FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F

unsafe extern "C" fn attack_air_f_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_BAYONETTA_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_F_FLAG,
        *FIGHTER_BAYONETTA_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_F_INT,
        *FIGHTER_BAYONETTA_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_F_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn attack_air_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    fair_motion(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_attack_air_f_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_attack_air_f_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO) 
    && (ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F || fighter.is_cat_flag(Cat1::Catch))
    && !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION) {
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F.into(), false.into());
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) 
    && !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION)
    && !fighter.is_motion(Hash40::new("attack_air_f3")) {
        let control_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) as *mut smash::app::KineticEnergy;
        // set speed muls
        let mut x_speed = 0.6;
        let mut x_center = 0.0;
        let mut y_speed = 0.0;
        if fighter.is_motion(Hash40::new("attack_air_f")) {
            x_center = 8.0; // 8.0 farthest from center
            y_speed = fighter.get_param_float("param_private", "attack_air_f_hit_speed_y");
        } else {
            x_center = 12.5; // 7.5 farthest from center
            y_speed = fighter.get_param_float("param_private", "attack_air_f2_hit_speed_y");
        }
        // calc pos dependent speed
        let hit_pos = VarModule::get_vec3(fighter.battle_object, vars::common::instance::LAST_ATTACK_HIT_LOCATION);
        let center_pos = AttackModule::center_pos(fighter.module_accessor, 3, false);
        let x_add = (hit_pos.x - (PostureModule::pos_x(fighter.module_accessor) + x_center)) / 60.0 * fighter.lr();
        let y_add = (hit_pos.y - (PostureModule::pos_y(fighter.module_accessor) + 12.0)) / 19.0;
        // cut x speed and set y speed
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.055);
        smash::app::lua_bind::KineticEnergy::mul_speed(control_energy, &Vector3f::new(x_speed + x_add, 1.0, 1.0)); 
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, y_speed + y_add);
    }
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            fighter.sub_attack_air_uniq_process_exec_fix_pos();
        }
        return 0.into()
    }
    1.into()
}

unsafe extern "C" fn fair_motion(fighter: &mut L2CFighterCommon) -> L2CValue {
    // doesn't autocancel during startup if fair was combo'd
    if fighter.is_prev_status(*FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F) {
        fighter.on_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    let fair = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::ATTACK_AIR_F_COUNT);
    if fair == 1 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f2"), 0.0, 1.0, false, 0.0, false, false);
        //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F2); makes each fair stale separately
    } else if fair == 2 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f3"), 0.0, 1.0, false, 0.0, false, false);
        //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F3);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f"), 0.0, 1.0, false, 0.0, false, false);
        //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F);
    }
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("gun_hand") as i64, hash40("gun_hand_show_all") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    false.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_pre);
    
    agent.status(Pre, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F, attack_air_f_pre);
    agent.status(Main,*FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F,attack_air_f_main);
}
