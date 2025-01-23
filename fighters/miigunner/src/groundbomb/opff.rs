utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;

pub extern "C" fn groundbomb_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        if weapon.is_flag(*WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_FLAG_DAMAGE_REFLECT) {
            // prevents the bomb from snapping back to the ground when hit while resting
            GroundModule::set_attach_ground(weapon.module_accessor, false);
            let rand = sv_math::rand(hash40("weapon"), 90);
            weapon.set_int(rand, *WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_INT_RANDOM_ROT_Z);
            PostureModule::set_rot(weapon.module_accessor, &Vector3f::new(rand as f32, 0.0, 0.0), 0);
        }
        if weapon.is_status(*WEAPON_MIIGUNNER_GROUNDBOMB_STATUS_KIND_FLY) {
            if StatusModule::is_changing(weapon.module_accessor) {
                TeamModule::set_hit_team(weapon.module_accessor, -1);
            }
            if StopModule::is_hit(weapon.module_accessor) {
                let life = weapon.get_param_int("param_groundbomb", "life");
                let explosion_frame = weapon.get_param_int("param_groundbomb", "damage_reflect_explosion_frame");
                weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                weapon.set_int(explosion_frame, *WEAPON_MIIGUNNER_GROUNDBOMB_INSTANCE_WORK_ID_INT_DAMAGE_REFLECT_AFTER_COUNT);
            }
            // something bypasses this
            // if StopModule::is_hit(weapon.module_accessor) { 
            //     for i in 0..Fighter::get_fighter_entry_count() {
            //         let opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i));
            //         if AttackModule::is_infliction(opponent_boma, *COLLISION_KIND_MASK_HIT) {
            //             let opponent_team_no = TeamModule::hit_team_no(opponent_boma) as i32;
            //             let owner_id = (&mut *(opponent_boma)).battle_object_id;
            //             TeamModule::set_team_owner_id(weapon.module_accessor, owner_id);
            //             TeamModule::set_hit_team(weapon.module_accessor, opponent_team_no);
            //         }
            //     }
            // }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, groundbomb_frame);
}