use smash::app::lua_bind::*;
use smash::app::*;
use smash::phx::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash_script::macros::ToF32;
use utils::consts::*;
use utils::ext::*;
use utils::*;
use utils::game_modes::CustomMode;
use smashline::*;

use globals::*;

#[skyline::hook(offset = 0xf13ddc, inline)]
unsafe fn steve_parry_stuff_fix(ctx: &mut skyline::hooks::InlineCtx) {
    if ctx.registers[0].x() == 0x1D {
        *((ctx as *mut _ as *mut u8).add(0x300).add(0x98) as *mut u32) = 0x1;
    }
}

// #[skyline::hook(offset = 0x641814, inline)]
// unsafe fn shield_damage_analog(ctx: &skyline::hooks::InlineCtx) {
//     let boma =
//         *(ctx.registers[0].x() as *const u64).add(1) as *mut BattleObjectModuleAccessor;
//     let current_shield = WorkModule::get_float(boma, 6);
//     let attack_power = *(ctx.registers[19].x() as *const f32).add(0xf730 / 4);
//     let analog = InputModule::get_analog_for_guard((*boma).object());
//     let damage_mul = WorkModule::get_param_float(
//         boma,
//         smash::hash40("common"),
//         smash::hash40("shield_damage_mul"),
//     );
//     let damage_mul = if analog > 0.0 && analog < 1.0 {
//         damage_mul + 0.2 * (1.0 - analog)
//     } else {
//         damage_mul
//     };
//     WorkModule::set_float(boma, current_shield - attack_power * damage_mul, 6);
// }

// #[skyline::hook(offset = 0x6285f0, inline)]
// unsafe fn shield_pushback_analog(ctx: &skyline::hooks::InlineCtx) {
//     let fighter = ctx.registers[19].x();
//     let boma = *(fighter as *const u64).add(4);
//     let attack_module: u64 = *(boma as *const u64).add(0xa0 / 8);
//     let transactor_count: u64 = *(attack_module as *const u64).add(0x20 / 8);
//     let transactors: u64 = *(attack_module as *const u64).add(0x28 / 8);

//     let mul = WorkModule::get_param_float(
//         boma as _,
//         smash::hash40("common"),
//         smash::hash40("shield_rebound_speed_mul"),
//     );

//     for x in 0..transactor_count {
//         let transactor = transactors + 720 * x;
//         let p_list = *(transactor as *const u64).add(608 / 8);
//         if p_list == 0 {
//             continue;
//         }

//         let mut current = *(p_list as *const u64);
//         while current != p_list && current != 0 {
//             if *(current as *const u8).add(47) == 2 {
//                 let battle_object_id = *(current as *const u32).add(36 / 4);
//                 let object = utils::util::get_battle_object_from_id(battle_object_id);
//                 let analog = InputModule::get_analog_for_guard(object);
//                 let mul = if analog > 0.0 && analog < 1.0 {
//                     mul * analog * 0.1
//                 } else {
//                     mul
//                 };
//                 ctx.registers_f[0].set_s(mul);
//                 return;
//             }

//             current = *(current as *const u64);
//         }
//     }
//     ctx.registers_f[0].set_s(mul);
// }

pub fn install() {
    smashline::Agent::new("fighter")
        .on_start(fighter_reset)
        .on_line(Main, turbo_mode)
        .on_line(Main, hitfall_mode)
        .on_line(Main, airdash_mode)
        .on_line(Main, magicseries_mode)
        .on_line(Main, rivals_mode)
        .install();
    // skyline::patching::Patch::in_text(0x6417f4).nop();
    // skyline::patching::Patch::in_text(0x6285d0).nop();
    skyline::install_hooks!(
        steve_parry_stuff_fix,
        //set_hit_team_hook,
        //set_hit_team_second_hook,
        //set_team_second_hook,
        set_team_hook,
        //set_team_owner_id_hook,
        // shield_damage_analog,
        // shield_pushback_analog
    );
}

// #[skyline::hook(replace=TeamModule::set_hit_team)]
// unsafe fn set_hit_team_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
//     original!()(boma, arg2);
//     if (boma.kind() == *ITEM_KIND_BARREL) {
//         //println!("set hit team called for barrel: {:x}", arg2);
//         //println!("set hit team called");
//         //println!("barrel status: {:x}", boma.status());
//         let current_team = TeamModule::hit_team_no(boma);
//         //println!("setting hit team from {} to {}", current_team, arg2);
//         //println!();
//         //return;
//     }
// }

// #[skyline::hook(replace=TeamModule::set_hit_team_second)]
// unsafe fn set_hit_team_second_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
//     original!()(boma, arg2);
//     if (boma.is_item()
//     && boma.kind() == *ITEM_KIND_BARREL) {
//         //println!("set hit team second called for barrel: {:x}", arg2);
//         //println!("set team second called");
//         //println!("barrel status: {:x}", boma.status());
//         let current_team = TeamModule::hit_team_second_no(boma);
//         //println!("setting hit team second from {} to {}", current_team, arg2);
//         //println!();
//         //return;
//     }
// }

/// This resolves an issue where when someone moves into a barrel
/// after throwing it upwards, they are able to be hit by their
/// own barrel for 1 frame. This can also happen when throwing the
/// barrel forward and then moving into it while it is traveling along
/// the ground. This is here because editing item statuses is not possible
#[skyline::hook(replace=TeamModule::set_team)]
unsafe fn set_team_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32, arg3: bool) {
    if (boma.is_item()
      && boma.kind() == *ITEM_KIND_BARREL) {
        //println!("set team ignored for barrel: {:x}", arg2);
        //println!("set team called");
        //println!("barrel status: {:x}", boma.status());
        let current_team = TeamModule::team_no(boma);
        //println!("setting team from {} to {}", current_team, arg2);
        if arg2 != -1 {
            original!()(boma, arg2, arg3);
        }
    } else {
        original!()(boma, arg2, arg3);
    }
}

// #[skyline::hook(replace=TeamModule::set_team_second)]
// unsafe fn set_team_second_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
//     original!()(boma, arg2);
//     // if (boma.is_item()
//     // && boma.kind() == *ITEM_KIND_BARREL) {
//     //     //println!("set team second called for barrel: {:x}", arg2);
//     //     return;
//     // }
// }

// #[skyline::hook(replace=TeamModule::set_team_owner_id)]
// unsafe fn set_team_owner_id_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
//     original!()(boma, arg2);
//     if (boma.is_item()
//     && boma.kind() == *ITEM_KIND_BARREL) {
//         //println!("set team owner id called for barrel: {:x}", arg2);
//         println!("set team owner id called");
//         //println!("barrel status: {:x}", boma.status());
//         //let current_team = TeamModule::team_owner_id(boma);
//         //println!("setting team owner id from {} to {}", current_team, arg2);
//         //println!();
//         //return;
//     }
// }

pub extern "C" fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let ratio =
            (WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x_max"), 0)
                / WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0));
        VarModule::set_float(
            fighter.battle_object,
            vars::common::instance::JUMP_SPEED_RATIO,
            ratio,
        );
        if fighter.kind() == *FIGHTER_KIND_KEN
            || fighter.kind() == *FIGHTER_KIND_RYU
            || fighter.kind() == *FIGHTER_KIND_DOLLY
        {
            MeterModule::reset(fighter.battle_object);
        }
    }
}

pub unsafe extern "C" fn turbo_mode(fighter: &mut L2CFighterCommon) {
    if utils::game_modes::check_custom_mode(CustomMode::TurboMode) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            // enable turbo behavior
            CancelModule::enable_cancel(fighter.boma());
            //println!("enabled cancelling!");

            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.sub_wait_ground_check_common(false.into());
            } else {
                fighter.sub_air_check_fall_common();
            }
        }
        
    }
}

pub unsafe extern "C" fn hitfall_mode(fighter: &mut L2CFighterCommon) {
    match utils::game_modes::get_custom_mode() {
        Some(modes) => {
            if modes.contains(&CustomMode::HitfallMode)
            || modes.contains(&CustomMode::RivalsOfAetherMode) {
                fighter.check_hitfall();
            }
        },
        _ => {}
    }
}

pub unsafe extern "C" fn airdash_mode(fighter: &mut L2CFighterCommon) {
    if utils::game_modes::check_custom_mode(CustomMode::AirdashMode) {
        fighter.check_airdash();
    }
}

pub unsafe extern "C" fn magicseries_mode(fighter: &mut L2CFighterCommon) {
    if utils::game_modes::check_custom_mode(CustomMode::MagicSeriesMode) {
        fighter.check_magicseries();
    }
}

pub unsafe extern "C" fn rivals_mode(fighter: &mut L2CFighterCommon) {
    if utils::game_modes::check_custom_mode(CustomMode::RivalsOfAetherMode) {
        rivals_drift_di(fighter);
        rivals_waveland(fighter);
        rivals_jab_tilt(fighter);
        rivals_landing_lag_jc(fighter);
        rivals_parry_stun(fighter);
    }
}

unsafe fn rivals_drift_di(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && !StopModule::is_stop(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
    ]) {
        let damage_speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let damage_speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    
        let mut initial_speed_x = VarModule::get_float(
            fighter.object(),
            vars::common::status::INITIAL_KNOCKBACK_VEL_X,
        );
        let mut initial_speed_y = VarModule::get_float(
            fighter.object(),
            vars::common::status::INITIAL_KNOCKBACK_VEL_Y,
        );
    
        // if these floats are both exactly zero, its because
        // status change reset them to zero. Thus, we should set them.
        if initial_speed_x == 0.0 && initial_speed_y == 0.0 {
            VarModule::set_float(
                fighter.object(),
                vars::common::status::INITIAL_KNOCKBACK_VEL_X,
                damage_speed_x,
            );
            VarModule::set_float(
                fighter.object(),
                vars::common::status::INITIAL_KNOCKBACK_VEL_Y,
                damage_speed_y,
            );
    
            initial_speed_x = VarModule::get_float(
                fighter.object(),
                vars::common::status::INITIAL_KNOCKBACK_VEL_X,
            );
            initial_speed_y = VarModule::get_float(
                fighter.object(),
                vars::common::status::INITIAL_KNOCKBACK_VEL_Y,
            );
        }
    
        let mut speed_mul = 0.005;
        let speed_mul_add_max = 0.0025;
        let speed_lerp_max = 3.0;
    
        let ratio = 1.0 - (initial_speed_x.abs() / speed_lerp_max).clamp(0.0, 1.0);
        speed_mul = (speed_mul + speed_mul_add_max) * ratio;
    
        let drift_value = fighter.left_stick_x() * speed_mul;
    
        fighter.set_speed(
            Vector2f::new(damage_speed_x + drift_value, damage_speed_y),
            *FIGHTER_KINETIC_ENERGY_ID_DAMAGE,
        );
    }
}

// immediately actionable waveland in rivals mode
unsafe fn rivals_waveland(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_LANDING)
    && fighter.is_prev_status_one_of(&[
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
    ]) {
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide_max"));
        if fighter.status_frame().to_f32() < landing_frame {
            let terms_to_enable = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            ];
            fighter.enable_transition_term_many(&terms_to_enable);
            if fighter.sub_transition_group_check_ground_item().get_bool()
            || fighter.sub_transition_group_check_ground_special().get_bool()
            || fighter.sub_transition_group_check_ground_attack().get_bool()
            || fighter.sub_transition_group_check_ground_jump().get_bool() {
                return;
            }
        }
    }
}

// cancel jabs directly into tilts
unsafe fn rivals_jab_tilt(fighter: &mut L2CFighterCommon) {
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    && !fighter.is_in_hitlag() 
    && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK 
    && fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
        if fighter.is_cat_flag(Cat1::AttackS3) && !fighter.is_cat_flag(Cat1::AttackS4) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
        }
        if fighter.is_cat_flag(Cat1::AttackHi3) && !fighter.is_cat_flag(Cat1::AttackHi4) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
        }
        if fighter.is_cat_flag(Cat1::AttackLw3) && !fighter.is_cat_flag(Cat1::AttackLw4) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
        }
    }
}

// cancel last few frames of landing animation into jumpsquat
unsafe fn rivals_landing_lag_jc(fighter: &mut L2CFighterCommon) {
    let prev_inflict_status = VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
    if fighter.is_status(*FIGHTER_STATUS_KIND_LANDING)
    || (fighter.is_status(*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR) && prev_inflict_status & *COLLISION_KIND_MASK_HIT != 0) {
        let landing_lag = VarModule::get_float(fighter.battle_object, vars::common::instance::LANDING_LAG_FOR_RIVALS_MODE);
        let jump_squat = fighter.get_param_int("jump_squat_frame", "") as f32;
        let status_frame = fighter.status_frame() as f32;
        if status_frame + jump_squat > landing_lag {
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
            fighter.check_jump_cancel(false, false, false);
        }
    }
}

// get stunned when parried
unsafe fn rivals_parry_stun(fighter: &mut L2CFighterCommon) {

    if fighter.is_status_one_of(&[
        // parry stun statuses
        *FIGHTER_STATUS_KIND_FURAFURA_END,
        *FIGHTER_STATUS_KIND_FALL_SPECIAL,

        // jab
        *FIGHTER_STATUS_KIND_ATTACK,

        // damage statuses
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_SLIP_DAMAGE,
        *FIGHTER_STATUS_KIND_STABBED_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_THROWN,
        *FIGHTER_STATUS_KIND_MEWTWO_THROWN,
        *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY,
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_THROWN,
        *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN,
        *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN,
        *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN,
        *FIGHTER_STATUS_KIND_CATCHED_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_REFLET,
        *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
        *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
        *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD,
        *FIGHTER_STATUS_KIND_DOWN
    ]) {
        return;
    }

    let prev_inflict_status = VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
    let was_previously_parried = StatusModule::is_changing(fighter.module_accessor) && prev_inflict_status & *COLLISION_KIND_MASK_PARRY != 0;
    let is_currently_parried = CancelModule::is_enable_cancel(fighter.module_accessor) && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY);

    if was_previously_parried || is_currently_parried {
        WorkModule::set_flag(
            fighter.module_accessor,
            MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("down_spot_u")),
            *FIGHTER_STATUS_DOWN_FLAG_UP
        );
        fighter.set_float(30.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_FURAFURA_END, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        return;
    }

}
