// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff});
use super::*;
use globals::*;

unsafe fn korean_back_dash(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_DEMON_STATUS_KIND_DASH_BACK)
    && boma.left_stick_y() < WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y")) {
        boma.change_status_req(*FIGHTER_STATUS_KIND_SQUAT, false);
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
    ])
    && boma.is_cat_flag(Cat1::TurnDash) && boma.left_stick_y() > WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y")) {
        boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_DASH_BACK, false);
    }
}

unsafe fn enable_both_recovery_specials(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_LW_FALL]) && boma.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
    }
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_AIR_END]) && boma.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) && !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
    }
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) && !VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_N_AIR_SHOOT,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_AIR_END,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_FALL,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

unsafe fn up_special_freefall(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(fighter.module_accessor)
    && (fighter.is_situation(*SITUATION_KIND_GROUND)
        || fighter.is_situation(*SITUATION_KIND_CLIFF)
        || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]))
    {
        VarModule::off_flag(fighter.battle_object, vars::demon::instance::SPECIAL_HI_ENABLE_FREEFALL);
    }

    if fighter.is_prev_status(*FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE) {
        if StatusModule::is_changing(fighter.module_accessor) {
            VarModule::on_flag(fighter.battle_object, vars::demon::instance::SPECIAL_HI_ENABLE_FREEFALL);
        }
    }

    if fighter.is_status(*FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE) {
        if fighter.is_situation(*SITUATION_KIND_AIR)
        && VarModule::is_flag(fighter.battle_object, vars::demon::instance::SPECIAL_HI_ENABLE_FREEFALL) {
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                let speed_x_max_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_max_speed_x_mul"));
                WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
                fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
                *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
            }
        }
    }
}

unsafe fn camera_lockout(fighter: &mut L2CFighterCommon) {
    let lockout = VarModule::get_int(fighter.battle_object, vars::demon::instance::CAMERA_LOCKOUT_TIMER);
    VarModule::set_int(fighter.battle_object, vars::demon::instance::CAMERA_LOCKOUT_TIMER, (lockout - 1).max(0));
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    korean_back_dash(boma);
    enable_both_recovery_specials(boma);
    fastfall_specials(fighter);
    up_special_freefall(fighter, boma);
    camera_lockout(fighter);
}

pub extern "C" fn demon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		demon_frame(fighter)
    }
}

pub unsafe fn demon_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, demon_frame_wrapper);
}
