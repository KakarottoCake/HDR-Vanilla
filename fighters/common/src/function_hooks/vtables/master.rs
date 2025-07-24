use super::*;
use utils::ext::*;

#[skyline::hook(offset = 0xceb020)]
pub unsafe extern "C" fn master_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash_rs::app::LinkEvent) -> bool {
    if event.link_event_kind.0 == 0x1e4c0767e5 {
        return false;
    }
    original!()(vtable, fighter, event)
}

pub fn install() {
    skyline::install_hooks!(
        master_link_event
    );
}