// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn stance_head(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    // Enable meshes for stances
    // HeadA is the normal head
	// HeadB is the poison head
	// HeadS is the spike head
    if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 0 {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), false);
    }
    else if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 1  {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), false);
    }
    else if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2  {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), false);
    }
}

/// handle speed application
unsafe fn check_apply_speeds(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
    // handle speed application once
    if VarModule::is_flag(fighter.battle_object, vars::packun::instance::STANCE_ENABLE_CHANGE_SPEED) {
        if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 0 {
            apply_status_speed_mul(fighter, 1.0);
        } else if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_ESCAPE_F,
            *FIGHTER_STATUS_KIND_ESCAPE_B,
            *FIGHTER_STATUS_KIND_SLIP_STAND_F,
            *FIGHTER_STATUS_KIND_SLIP_STAND_B,
            *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
            *FIGHTER_STATUS_KIND_PASSIVE_FB]) {
                apply_status_speed_mul(fighter, 1.0);
        } else if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 1 {
            apply_status_speed_mul(fighter, 0.86);
        } else if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 {
            apply_status_speed_mul(fighter, 0.84);
        }
        VarModule::off_flag(fighter.battle_object, vars::packun::instance::STANCE_ENABLE_CHANGE_SPEED);
    }

    if fighter.status() != VarModule::get_int(fighter.battle_object, vars::packun::instance::STANCE_STATUS) {
        //println!("Status is changing!");
        VarModule::on_flag(fighter.battle_object, vars::packun::instance::STANCE_ENABLE_CHANGE_SPEED);
        VarModule::set_int(fighter.battle_object, vars::packun::instance::STANCE_STATUS, fighter.status());
        //println!("new stance status: {}", VarModule::get_int(fighter.battle_object, vars::packun::instance::STANCE_STATUS));
    }

    // dash & momentum transfer speeds
    if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 1 {
        VarModule::set_float(fighter.battle_object, vars::common::instance::JUMP_SPEED_MAX_MUL, 1.0);

        // if you are initial dash, slow them down slightly
        if fighter.is_status(*FIGHTER_STATUS_KIND_DASH) {
            let motion_vec = Vector3f {
                x: -0.15 * PostureModule::lr(fighter.boma()) * (1.0 - (MotionModule::frame(fighter.boma()) / MotionModule::end_frame(fighter.boma()))),
                y: 0.0, 
                z: 0.0
            };
            //KineticModule::add_speed_outside(fighter.boma(), *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }

    else if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 {
        VarModule::set_float(fighter.battle_object, vars::common::instance::JUMP_SPEED_MAX_MUL, 0.88);

        // if you are initial dash, slow them down slightly
        if fighter.is_status(*FIGHTER_STATUS_KIND_DASH) {
            let motion_vec = Vector3f {
                x: -0.15 * PostureModule::lr(fighter.boma()) * (1.0 - (MotionModule::frame(fighter.boma()) / MotionModule::end_frame(fighter.boma()))),
                y: 0.0, 
                z: 0.0
            };
            //KineticModule::add_speed_outside(fighter.boma(), *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }
}

/// applies the given multiplier to various speed stats of the given fighter. 
/// This should only be called once per status, or you will get some multiplicative effects
unsafe fn apply_status_speed_mul(fighter: &mut smash::lua2cpp::L2CFighterCommon, mul: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
    let og_speed_mul = app::sv_kinetic_energy::get_speed_mul(fighter.lua_state_agent);

    // set the X motion speed multiplier (where movement is baked into an anim)
    lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter.get_motion_energy(), og_speed_mul * mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_mul( fighter.get_controller_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_add( fighter.get_controller_energy(), mul);

    // set the X speed max multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_speed_max(fighter.get_controller_energy(), mul);
}

unsafe fn game_start_switch(fighter: &mut L2CFighterCommon) {
    if fighter.is_prev_status(*FIGHTER_STATUS_KIND_ENTRY) {
        if StatusModule::is_changing(fighter.module_accessor) {
            VarModule::on_flag(fighter.battle_object, vars::packun::status::STANCE_INIT);
        }
        if VarModule::is_flag(fighter.battle_object, vars::packun::status::STANCE_INIT) {
            if fighter.is_button_on(Buttons::AppealSL) {
                EFFECT(fighter, Hash40::new("sys_level_up"), Hash40::new("top"), -2, 10, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
                PLAY_SE(fighter, Hash40::new("se_packun_special_s02"));
                EFFECT_FOLLOW(fighter, Hash40::new("sys_grass_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, false);
                VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, 0);
                VarModule::off_flag(fighter.battle_object, vars::packun::status::STANCE_INIT);
            }
            else if fighter.is_button_on(Buttons::AppealSR) {
                EFFECT(fighter, Hash40::new("sys_level_up"), Hash40::new("top"), -2, 10, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
                PLAY_SE(fighter, Hash40::new("se_packun_special_s02"));
                EFFECT_FOLLOW(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, false);
                VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, 2);
                VarModule::off_flag(fighter.battle_object, vars::packun::status::STANCE_INIT);
            }
            else if fighter.is_button_on(Buttons::AppealLw) {
                EFFECT(fighter, Hash40::new("sys_level_up"), Hash40::new("top"), -2, 10, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
                PLAY_SE(fighter, Hash40::new("se_packun_special_s02"));
                EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("top"), 0, 15.5, 0, 0, 0, 0, 1.2, false);
                VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, 1);
                VarModule::off_flag(fighter.battle_object, vars::packun::status::STANCE_INIT);
            }
        }
        if fighter.status_frame() > 94 {
            VarModule::off_flag(fighter.battle_object, vars::packun::status::STANCE_INIT);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_FAILURE,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_HIT_END,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_FALL_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    stance_head(fighter);
    check_apply_speeds(fighter);
    game_start_switch(fighter);
    fastfall_specials(fighter);
}

unsafe extern "C" fn plant_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if !sv_information::is_ready_go() && fighter.status_frame() < 1 {
            return;
        }

        utils::ui::UiManager::set_plant_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_plant_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE)
        );
    }
}

pub extern "C" fn packun_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		packun_frame(fighter);
    }
}

pub unsafe fn packun_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, packun_frame_wrapper);
    agent.on_line(Main, plant_meter);
}