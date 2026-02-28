use super::*;

mod acmd;
mod opff;
mod status;

pub fn install() {
    let agent = &mut Agent::new("pitb_bowarrow");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();
}