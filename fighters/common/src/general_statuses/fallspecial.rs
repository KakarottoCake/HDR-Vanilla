use super::*;
use globals::*;


pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_fall_special
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_fall_special)]
pub unsafe fn status_fall_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = call_original!(fighter);

    let front_cliff_hang_data = fighter.get_front_cliff_hangdata();
    let p1_x = front_cliff_hang_data.x;

    // Reduce forward ledgegrab range while in special fall
    fighter.set_front_cliff_hangdata(p1_x - 4.0, front_cliff_hang_data.y);

    ret
}