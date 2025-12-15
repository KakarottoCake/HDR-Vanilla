use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod other;

pub unsafe fn shulk_get_trail(agent: &mut L2CAgentBase) {
    let original_trail = "tex_shulk_sword_hdr";
    let boma = agent.boma();
        if !WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
        return;
    }
    let active_art = WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
    let trail = match active_art {
        0 => format!("{}{}", original_trail, "_jump"),
        1 => format!("{}{}", original_trail, "_speed"),
        2 => format!("{}{}", original_trail, "_shield"),
        3 => format!("{}{}", original_trail, "_buster"),
        4 => format!("{}{}", original_trail, "_smash"),
        _ => original_trail.to_string()
    };
    AFTER_IMAGE4_ON_arg29(agent, trail.to_hash(), Hash40::new("tex_shulk_sword2"), 8, Hash40::new("haver"), 0, 3, 0.9, Hash40::new("haver"), 0, 19, 1.1, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
}

pub fn install(agent: &mut Agent) {
    ground::install(agent);
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
    other::install(agent);
}