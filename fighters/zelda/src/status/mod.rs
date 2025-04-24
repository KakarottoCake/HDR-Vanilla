use super::*;
use globals::*;
// status script import

mod special_hi;
mod special_lw;
mod special_s;
mod special_n;

/// Can't use side b if 3 are out (shouldn't happen)
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dein = VarModule::get_int(fighter.battle_object, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID);
    let dein2 = VarModule::get_int(fighter.battle_object, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID_2);
    let dein3 = VarModule::get_int(fighter.battle_object, vars::zelda::instance::SPECIAL_S_CURRENT_DEIN_MOVE_OBJECT_ID);
    if dein != 0 && sv_battle_object::is_active(dein as u32) 
    && dein2 != 0 && sv_battle_object::is_active(dein2 as u32) 
    && dein3 != 0 && sv_battle_object::is_active(dein3 as u32) 
    {//if all 3 active
        return false.into()
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::zelda::instance::SPECIAL_LW_COOLDOWN_EFFECT_HANDLE, -1); //phantom
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_hi::install(agent);
    special_lw::install(agent);
    special_s::install(agent);
    special_n::install(agent);
}
