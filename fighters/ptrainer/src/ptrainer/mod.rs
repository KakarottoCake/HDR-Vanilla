use super::*;

mod opff;

pub fn install() {
    let agent = &mut Agent::new("ptrainer_ptrainer");
    opff::install(agent);
    agent.install();
}