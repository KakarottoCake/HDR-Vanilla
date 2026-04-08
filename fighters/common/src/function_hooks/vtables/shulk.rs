use super::*;

extern "C" {
    #[link_name = "shulk_check_valid_arts_statuses_inner"]
    fn shulk_check_valid_arts_statuses_inner(fighter: &mut Fighter) -> bool;
}

// disables art wheel during hitstun, and enables it during jab, tilts and aerials

#[skyline::hook(offset = 0x116a3d0)]
pub unsafe extern "C" fn shulk_check_valid_arts_statuses(fighter: &mut Fighter) -> bool {
    shulk_check_valid_arts_statuses_inner(fighter)
}

pub fn install() {
    skyline::install_hooks!(
        shulk_check_valid_arts_statuses,
    );
}
