use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("pitb_bowarrow");
    acmd::install(agent);
    agent.install();
}