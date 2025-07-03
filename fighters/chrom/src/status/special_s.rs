use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    return false.into();
}

pub unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_spd_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("start_spd_x_mul"));
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * start_spd_x_mul;
    let reset_speed_2f = Vector2f { x: speed_x, y: 0.0 };
    let reset_speed_3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let mut motion_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut app::KineticEnergy;
        smash::app::lua_bind::KineticEnergy::reset_energy(motion_energy, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS_IGNORE_NORMAL, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
        smash::app::lua_bind::KineticEnergy::enable(motion_energy);
        smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    else {
        let air_spd_y = fighter.get_param_float("param_special_s", "air_spd_y");
        let reset_speed_gravity_2f = Vector2f { x: 0.0, y: air_spd_y };
        let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
        let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::KineticEnergy;
        smash::app::lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
        smash::app::lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &reset_speed_gravity_2f, &reset_speed_3f, fighter.module_accessor);
        smash::app::lua_bind::KineticEnergy::enable(stop_energy);
        smash::app::lua_bind::KineticEnergy::enable(gravity_energy);
        smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    return false.into();
}

pub unsafe extern "C" fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);

    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2]) && fighter.is_situation(*SITUATION_KIND_AIR) {
        let fighter_gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::FighterKineticEnergyGravity;
        lua_bind::FighterKineticEnergyGravity::set_accel(fighter_gravity, -0.072);
        lua_bind::FighterKineticEnergyGravity::set_stable_speed(fighter_gravity, -2.0);
    }

    ret
}

unsafe extern "C" fn special_s3_check_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    // force knockdown
    if fighter.is_motion_one_of(&[Hash40::new("special_s3_lw"), Hash40::new("special_air_s3_lw")])
    && (&param_3["object_category_"]).get_i32() == *BATTLE_OBJECT_CATEGORY_FIGHTER 
    && (&param_3["kind_"]).get_i32() == *COLLISION_KIND_HIT {
        let object_id = (&param_3["object_id_"]).get_u32();
        let opponent_boma = sv_battle_object::module_accessor(object_id);
        let opponent_object = utils::util::get_battle_object_from_accessor(opponent_boma);
        VarModule::on_flag(opponent_object, vars::common::instance::IS_KNOCKDOWN_THROW);
    }
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, special_s_pre);
    agent.status(Init, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, special_s_init);
    agent.status(Exec, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, special_s_exec);

    agent.status(Pre, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, special_s_pre);
    agent.status(Init, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, special_s_init);
    agent.status(Exec, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, special_s_exec);
    agent.status(CheckAttack, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, special_s3_check_attack);

    agent.status(Pre, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, special_s_pre);
    agent.status(Init, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, special_s_init);
    agent.status(Exec, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, special_s_exec);
}