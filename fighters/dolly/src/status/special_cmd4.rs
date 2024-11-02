use super::*;

// statuses::ryu::ATTACK_COMMAND_4

pub unsafe extern "C" fn attack_command_4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_COMMAND2 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn attack_command_4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_command4"), 0.0, 1.0, false, 0.0, false, false);
    // fighter.clear_lua_stack();
    // fighter.push_lua_stack(&mut L2CValue::I32(*FIGHTER_KINETIC_ENERGY_ID_MOTION));
    // let mut lr = PostureModule::lr(fighter.module_accessor);
    // fighter.push_lua_stack(&mut L2CValue::F32(lr));
    // app::sv_kinetic_energy::set_chara_dir(fighter.lua_state_agent);
    VarModule::off_flag(fighter.battle_object, vars::dolly::status::ATTACK_COMMAND_4_RELEASE_BUTTON);
    fighter.set_int(*FIGHTER_DOLLY_STRENGTH_S, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_command_4_main_loop as *const () as _))
}

pub unsafe extern "C" fn attack_command_4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }
    let motion_frame = fighter.motion_frame();
    let frame = fighter.status_frame();
    if !VarModule::is_flag(fighter.battle_object, vars::dolly::status::ATTACK_COMMAND_4_RELEASE_BUTTON) && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::status::ATTACK_COMMAND_4_RELEASE_BUTTON);
        let button_strength = if motion_frame <= 4.0 {
            *FIGHTER_DOLLY_STRENGTH_W
        } else {
            *FIGHTER_DOLLY_STRENGTH_S
        };
        fighter.set_int(button_strength, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::dolly::ATTACK_COMMAND_4, attack_command_4_pre);
    agent.status(Main, statuses::dolly::ATTACK_COMMAND_4, attack_command_4_main);
}
