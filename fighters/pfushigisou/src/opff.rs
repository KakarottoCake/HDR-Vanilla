// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn special_lw_track(boma: &mut BattleObjectModuleAccessor) {
    let parent_id = LinkModule::get_parent_id(boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
    let object = utils::util::get_battle_object_from_id(parent_id);
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && !boma.is_button_on(Buttons::SpecialAll) {
        VarModule::off_flag(object, vars::ptrainer::instance::SPECIAL_LW_BACKWARDS_SWITCH);
    }
    if is_training_mode() && !sv_information::is_ready_go() {
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, *PLEDGE_STATE_NONE);
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 0);
        VarModule::off_flag(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER);
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER, 0);
    }
    let pledge = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE) as i32;
    if pledge == *PLEDGE_STATE_NONE 
    && (sv_information::is_ready_go() || boma.status_frame() >= 1)
    && !boma.is_status_one_of(&[
        *FIGHTER_POKEMON_STATUS_KIND_SPECIAL_LW_OUT,
        *FIGHTER_POKEMON_STATUS_KIND_SPECIAL_LW_STANDBY
    ]) {
        let entry_id = boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        utils::ui::UiManager::set_ptrainer_meter_enable(entry_id, false);
    }
    dbg!(boma.status());
}

unsafe fn up_special_freefall(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && !StatusModule::is_changing(fighter.module_accessor)
    && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && CancelModule::is_enable_cancel(fighter.module_accessor) {
        let accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_accel_x_mul");
        let speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_speed_x_max_mul");
        WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
        WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);

        let landing_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.landing_frame");
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);

        let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
        *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
    }

    if fighter.is_prev_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && StatusModule::is_changing(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PFUSHIGISOU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_HOP);
    }

    if fighter.is_status(*FIGHTER_STATUS_KIND_AIR_LASSO_FAILURE)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && !StatusModule::is_changing(fighter.module_accessor)
    && MotionModule::is_end(fighter.module_accessor) {
        let accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_accel_x_mul");
        let speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_speed_x_max_mul");
        WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
        WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);

        let landing_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.landing_frame");
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    up_special_freefall(fighter);
    special_lw_track(boma);
    fastfall_specials(fighter);
}

pub extern "C" fn pfushigisou_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pfushigisou_frame(fighter)
    }
}

pub unsafe fn pfushigisou_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pfushigisou_frame_wrapper);
}
