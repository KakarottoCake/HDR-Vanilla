use super::*;

// This file contains code for item throw

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_ItemThrowCommon,
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_ItemThrowCommon)]
unsafe fn status_ItemThrowCommon(fighter: &mut L2CFighterCommon) {
    call_original!(fighter);

    if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_DITCIT) {
        let mut speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let min_speed = ParamModule::get_float(fighter.battle_object, ParamType::Common, "ditcit.min_speed");
        let max_speed = ParamModule::get_float(fighter.battle_object, ParamType::Common, "ditcit.max_speed");
        let ditcit_mul = ParamModule::get_float(fighter.object(), ParamType::Shared, "ditcit_mul");

        speed_x = speed_x * ditcit_mul;

        if min_speed > 0.0 {
            if speed_x == 0.0 {
                speed_x = min_speed * PostureModule::lr(fighter.module_accessor);
            }
            else {
                speed_x = speed_x.abs().max(min_speed) * speed_x.signum();
            }
        }

        if max_speed > 0.0 {
            speed_x = speed_x.abs().min(max_speed) * speed_x.signum();
        }

        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_x, speed_y);
    }
}
