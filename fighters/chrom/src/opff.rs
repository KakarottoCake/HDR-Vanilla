use super::*;

utils::import_noreturn!(common::opff::fighter_common_opff);

// Side Special Cancels
unsafe fn side_special_cancels(fighter: &mut L2CFighterCommon) {
    // skip SSpecial1, because new SSpecial is just 3 hits
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) {
        fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_S2.into(), false.into());
    }
    // New SSpecial1 cancels into aerials on hit
    if fighter.is_status(*FIGHTER_ROY_STATUS_KIND_SPECIAL_S2)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY)
    && !fighter.is_in_hitlag()
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR 
    && fighter.get_aerial() != None {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), false.into());
    }
    // Disallow SpecialAirS4Lw because we dont have an animation for it lol
    if fighter.is_status(*FIGHTER_ROY_STATUS_KIND_SPECIAL_S4)
    && fighter.is_motion(Hash40::new("special_air_s4_lw")) {
        fighter.set_int64(hash40("special_s4_s") as i64, *FIGHTER_ROY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
        fighter.set_int64(hash40("special_air_s4_s") as i64, *FIGHTER_ROY_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
        fighter.off_flag(*FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_HI);
        fighter.off_flag(*FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_LW);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s4_s"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT
        ])
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

unsafe fn sspecial_ledgegrab_fix(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4,
    ])
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_transition_group_check_air_cliff();
    }
}

unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    let long_sword_scale = Vector3f{x: 1.015, y: 1.065, z: 1.045};
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}

pub unsafe extern "C" fn chrom_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    side_special_cancels(fighter);
    fastfall_specials(fighter);
    sspecial_ledgegrab_fix(fighter);
    sword_length(&mut *(fighter.module_accessor));
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, chrom_frame_wrapper); 
}