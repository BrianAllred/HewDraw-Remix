use super::*;

mod special_s;
mod special_hi;
mod special_lw;

pub fn install() {
    special_s::install();
    special_hi::install();
    special_lw::install();
}

pub fn add_statuses() {
    special_s::add_statuses();
}