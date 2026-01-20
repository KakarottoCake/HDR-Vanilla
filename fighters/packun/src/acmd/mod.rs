use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod other;

#[repr(C)]
pub struct StanceInfo {
    label: i32,
    damage_bite: f32,
    damage_head: f32,
    damage_other: f32
}

impl From<i32> for StanceInfo {
    fn from(other: i32) -> Self {
        match other {
            0 => StanceInfo { // Regular
                label: 0,
                damage_bite: 1.0,
                damage_head: 1.0,
                damage_other: 1.0
            },
            1 => StanceInfo { // Putrid
                label: 1,
                damage_bite: 0.75,
                damage_head: 0.75,
                damage_other: 0.75
            },
            2 => StanceInfo { // Prickly
                label: 2,
                damage_bite: 1.0,
                damage_head: 1.3,
                damage_other: 0.9
            },
            _ => StanceInfo { // same as regular
                label: 3,
                damage_bite: 1.0,
                damage_head: 1.0,
                damage_other: 1.0
            },
        }
    }
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