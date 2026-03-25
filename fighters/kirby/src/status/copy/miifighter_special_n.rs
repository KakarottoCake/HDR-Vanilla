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
    let charge = VarModule::get_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_CHARGE) as f32;
    let angle = 45.0 - charge * 0.75;
    let guide_pos = arrow_guide_pos(fighter, angle);
    if VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_START_HOLD) {
        let mut eff_handle = VarModule::get_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
            eff_handle = EffectModule::req(fighter.module_accessor, Hash40::new("sys_direction2"), &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0) as u32;
            let team_color = FighterUtil::get_team_color(fighter.module_accessor);
            let mut effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
            EffectModule::set_rgb(fighter.module_accessor, eff_handle, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
            VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE, eff_handle as i32);
        }

        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::inc_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_CHARGE);
            if charge == 1.0 {
                MotionModule::set_rate(fighter.module_accessor, 0.5);
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 2, 12, -3, 0, 0, 0, 0.3, false, *EF_FLIP_AXIS_YZ);
            }
            if charge == 10.0 {
                fighter.change_motion_inherit_frame_keep_rate_by_situation("special_n1_bowl", "special_air_n1_bowl", -1.0, 1.0, 0.0);
            }
        }
        else {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            VarModule::on_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_END_HOLD);
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_END_HOLD) {
        VarModule::off_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_START_HOLD);
        VarModule::off_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_END_HOLD);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        let throw_speed = if fighter.is_situation(*SITUATION_KIND_GROUND) { 2.75 } else { 2.75 - (charge * 0.025) };
        VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_ANGLE, angle);
        VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_SPEED, throw_speed);
    }
    let eff_handle = VarModule::get_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE) as u32;
    if EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
        EffectModule::set_pos(fighter.module_accessor, eff_handle, &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0});
        if fighter.lr() >= 0.0 {
            EffectModule::set_rot(fighter.module_accessor, eff_handle, &Vector3f{x: 0.0, y: 0.0, z: angle - 90.0});
        }
        else {
            EffectModule::set_rot(fighter.module_accessor, eff_handle, &Vector3f{x: 0.0, y: 0.0, z: -270.0 - angle});
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

unsafe fn arrow_guide_pos(fighter: &mut L2CFighterCommon, angle: f32) -> Vector2f {
    let pos = PostureModule::pos(fighter.module_accessor);
    let angle_rad = angle.to_radians();
    let scale = PostureModule::scale(fighter.module_accessor);
    let dist = 9.0;
    let dist_scaled = dist * scale;
    let x_pos = angle_rad.cos() * dist_scaled * fighter.lr() + (*pos).x;
    let y_pos = angle_rad.sin() * dist_scaled + (*pos).y;
    let y_offset = 6.0;
    let y_pos = y_offset * scale + y_pos;
    Vector2f{x: x_pos, y: y_pos}
}

pub unsafe extern "C" fn miifighter_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MIIFIGHTER_GENERATE_ARTICLE_IRONBALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    let eff_handle = VarModule::get_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE) as u32;
    if EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
        EffectModule::kill(fighter.module_accessor, eff_handle, true, true);
        VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_N1_EFFECT_HANDLE, -1);
    }
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_MIIFIGHTER_SPECIAL_N, miifighter_special_n_main);
    agent.status(End, *FIGHTER_KIRBY_STATUS_KIND_MIIFIGHTER_SPECIAL_N, miifighter_special_n_end);
}