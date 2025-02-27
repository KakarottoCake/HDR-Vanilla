use super::*;

#[skyline::hook(offset = 0x34ceab0)]
pub unsafe extern "C" fn request_change_pokemon(weapon: &mut smash::app::Weapon) -> u64 {
    let boma = weapon.battle_object.module_accessor;
    let object = utils::util::get_battle_object_from_accessor(boma);
    if VarModule::is_flag(object, vars::ptrainer::instance::DISABLE_SPECIAL_LW) {
        return 0;
    }

    original!()(weapon)
}

pub fn install() {
    skyline::install_hooks!(
        request_change_pokemon
    );
}