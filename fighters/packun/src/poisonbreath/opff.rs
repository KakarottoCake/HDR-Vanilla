// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub extern "C" fn poisonbreath_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        let owner_module_accessor = weapon.get_owner_boma();
        if owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN {
            let pos_x = PostureModule::pos_x(boma);
            let pos_y = PostureModule::pos_y(boma);
            let packun_pos_x = PostureModule::pos_x(owner_module_accessor);
            let packun_pos_y = PostureModule::pos_y(owner_module_accessor);
            // let fire_pos_x = VarModule::get_float(owner_object, vars::packun::instance::FIRE_POS_X);
            // let fire_pos_y = VarModule::get_float(owner_object, vars::packun::instance::FIRE_POS_Y);
            let scale = PostureModule::scale(boma);
            if ((pos_x - packun_pos_x).abs() < 12.0 * scale)
            && ((pos_y - packun_pos_y).abs() < 12.0 * scale)
            && pos_y != 0.0 {
                if owner_module_accessor.is_status(*FIGHTER_STATUS_KIND_APPEAL) {
                    VarModule::on_flag(owner_module_accessor.object(), vars::packun::status::APPEAL_CLOUD_COVER);
                }
            }
            // if ((pos_x - fire_pos_x).abs() < 12.0 * scale) &&
            //     ((pos_y - fire_pos_y).abs() < 12.0 * scale) &&
            //     pos_y != 0.0 && fire_pos_y != 0.0 {
            //     if motion_kind != hash40("explode") {
            //         MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, false, 0.0, false, false);
            //     }
            // }
		}
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, poisonbreath_frame);
}