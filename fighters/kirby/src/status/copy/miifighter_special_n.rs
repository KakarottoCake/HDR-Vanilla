use super::*;

pub unsafe extern "C" fn miifighter_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        special_n_change_motion(fighter, Hash40::new("miifighter_special_n1"));
    }
    else if fighter.is_situation(*SITUATION_KIND_AIR) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        special_n_change_motion(fighter, Hash40::new("miifighter_special_air_n1"));
    }
    VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_ANGLE, 45.0);
    VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_SPEED, 2.25);

    fighter.main_shift(miifighter_special_n_main_loop)
}

unsafe extern "C" fn miifighter_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            special_n_change_motion(fighter, Hash40::new("miifighter_special_n1"));
        }
        else if fighter.is_situation(*SITUATION_KIND_AIR) {
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            special_n_change_motion(fighter, Hash40::new("miifighter_special_air_n1"));
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_CHECK_HOLD) {
        VarModule::off_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_CHECK_HOLD);
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            let throw_speed = if fighter.is_situation(*SITUATION_KIND_GROUND) { 2.75 } else { 2.25 };
            VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_ANGLE, 30.0);
            VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_SPEED, throw_speed);
            fighter.change_motion_inherit_frame_by_situation("miifighter_special_n1_bowl", "miifighter_special_air_n1_bowl", -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT } else { FIGHTER_STATUS_KIND_FALL };
        fighter.change_status(status.into(), false.into());
    }

    return 0.into();
}

unsafe fn special_n_change_motion(fighter: &mut L2CFighterCommon, motion: Hash40) {
    if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_IRONBALL_FLAG_FIRST) {
        let _motion = if fighter.is_motion(Hash40::new("miifighter_special_n1_bowl")) {
            VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_SPEED, 2.25);
            Hash40::new("miifighter_special_air_n1_bowl")
        }
        else if fighter.is_motion(Hash40::new("miifighter_special_air_n1_bowl")) {
            VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_SPEED, 2.75);
            Hash40::new("miifighter_special_n1_bowl")
        }
        else { motion };
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, _motion, -1.0, 1.0, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_IRONBALL_FLAG_FIRST);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_MIIFIGHTER_SPECIAL_N, miifighter_special_n_main);
}