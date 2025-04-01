use super::*;
use globals::*;

mod special_s;
mod special_hi;
mod special_lw;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reset shine stall flag on landing or ledgegrab
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&fighter.global_table[SITUATION_KIND].get_i32())
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        VarModule::off_flag(fighter.battle_object, vars::falco::instance::SPECIAL_LW_DISABLE_STALL);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
}

unsafe extern "C" fn attack_lw4_check_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    if (&param_3["object_category_"]).get_i32() == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if (&param_3["kind_"]).get_i32() == *COLLISION_KIND_HIT {
            if VarModule::is_flag(fighter.battle_object, vars::falco::instance::HANDS_OFF_MY) {
                let object_id = (&param_3["object_id_"]).get_u32();
                let opponent_boma = sv_battle_object::module_accessor(object_id);
                StatusModule::change_status_force(opponent_boma, *FIGHTER_STATUS_KIND_DEAD, false);
            }
        }
        
    }
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    
    special_s::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);

    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_check_attack);
}
