// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
//     if !fighter.is_in_hitlag()
//     && !StatusModule::is_changing(fighter.module_accessor)
//     && fighter.is_status_one_of(&[
//         *FIGHTER_STATUS_KIND_SPECIAL_LW,
//         *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A,
//         ]) 
//     && fighter.is_situation(*SITUATION_KIND_AIR) {
//         fighter.sub_air_check_dive();
//         if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
//             if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
//                 fighter.clear_lua_stack();
//                 lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
//                 let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

//                 fighter.clear_lua_stack();
//                 lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
//                 app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
//                 fighter.clear_lua_stack();
//                 lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
//                 app::sv_kinetic_energy::enable(fighter.lua_state_agent);

//                 KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
//             }
//         }
//     }
// }
 
pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
    //fastfall_specials(fighter);
    
    // disables bomb jump
    WorkModule::off_flag(fighter.module_accessor, 0x200000E2); // FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_IS_CHANGE_STATUS_BOMBJUMP
}

pub extern "C" fn samusd_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        samusd_frame(fighter);
    }
}

pub unsafe fn samusd_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, samusd_frame_wrapper);
}