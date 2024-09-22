use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("pzenigame_water");
    acmd::install(agent);
    agent.install();
}