use super::*;
use utils::ext::*;

#[skyline::hook(offset = 0xd59650)]
pub unsafe extern "C" fn miifighter_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = *(log as *const u64).add(0x10/0x8) as *const CollisionLog;
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_N_NO) == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3
    && (&mut *(boma)).is_status(*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_CATCH) {
        let opponent_id = (*collision_log).opponent_battle_object_id;
        if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            check_grabbed_article(boma, opponent_id);
        }
    }
    println!();
    
    original!()(vtable, fighter, log)
}

unsafe fn check_grabbed_article(boma: *mut BattleObjectModuleAccessor, opponent_id: u32) {
    let counter_throw_boma = sv_battle_object::module_accessor(opponent_id as u32);
    if sv_battle_object::category(opponent_id) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        LinkModule::remove_model_constraint(counter_throw_boma, true);
        if LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
            LinkModule::unlink(counter_throw_boma, *LINK_NO_ARTICLE);
        }
        if !LinkModule::is_link(counter_throw_boma, *LINK_NO_ARTICLE) {
            VisibilityModule::set_whole(counter_throw_boma, true);
            LinkModule::link(counter_throw_boma, *LINK_NO_ARTICLE, (*boma).battle_object_id);
            if (&mut *(counter_throw_boma)).kind() == *WEAPON_KIND_DOLLY_WAVE {
                ArticleModule::remove_exist(boma, *FIGHTER_DOLLY_GENERATE_ARTICLE_WAVE, ArticleOperationTarget(0));
                return;
            }
            LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *LINK_NO_ARTICLE, Hash40::new("rot"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
            LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
        }
    }
    else if sv_battle_object::category(opponent_id) == *BATTLE_OBJECT_CATEGORY_ITEM {
        LinkModule::remove_model_constraint(counter_throw_boma, true);
        if LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(counter_throw_boma, *ITEM_LINK_NO_HAVE);
        }
        if !LinkModule::is_link(counter_throw_boma, *ITEM_LINK_NO_HAVE) {
            VisibilityModule::set_whole(counter_throw_boma, true);
            LinkModule::link(counter_throw_boma, *ITEM_LINK_NO_HAVE, (*boma).battle_object_id);
            LinkModule::set_model_constraint_pos_ort(counter_throw_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
            LinkModule::set_constraint_translate_offset(counter_throw_boma, &Vector3f::zero());
        }
    }
    else {
        return;
    }
    
    GroundModule::set_ignore_boss(counter_throw_boma, true);
    GroundModule::set_passable_check(counter_throw_boma, false);
    GroundModule::set_collidable(counter_throw_boma, false);
    JostleModule::set_status(counter_throw_boma, false);
    WorkModule::on_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_THROW_AFTER_LANDING);
    VarModule::set_int((&mut *(boma)).object(), vars::miifighter::instance::SPECIAL_N3_GRABBED_OBJECT_ID, opponent_id as i32);
    VarModule::on_flag((&mut *(boma)).object(), vars::miifighter::instance::SPECIAL_N3_IS_LINK);
    StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, false);
}

pub fn install() {
    skyline::install_hooks!(
        miifighter_on_search
    );
}