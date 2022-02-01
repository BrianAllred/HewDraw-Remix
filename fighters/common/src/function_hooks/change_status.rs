use super::*;
use globals::*;


#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;

    if boma.is_fighter() {
        if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE].contains(&StatusModule::status_kind(boma)) && !CancelModule::is_enable_cancel(boma) {
            if [*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_TURN].contains(&next_status) {
                return 0;
            }
        }
        if boma.kind() == *FIGHTER_KIND_TRAIL {
            if StatusModule::status_kind(boma) == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH {
                if next_status == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_TURN {
                    if !VarModule::is_flag(boma.object(), vars::trail::IS_SIDE_SPECIAL_INPUT)
                    && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
                        if VarModule::is_flag(boma.object(), vars::trail::SIDE_SPECIAL_HIT)
                        && !VarModule::is_flag(boma.object(), vars::trail::UP_SPECIAL_TO_SIDE_SPECIAL)
                        && !VarModule::is_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL) {
                            VarModule::on_flag(boma.object(), vars::common::SIDE_SPECIAL_CANCEL);
                            next_status = *FIGHTER_STATUS_KIND_FALL;
                        }
                        else {
                            next_status = *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END;
                        }
                    }
                }
            }
        }
    }
    //println!("next status: {}", next_status);
    original!()(boma, next_status, arg3)
}

pub fn install() {
    skyline::install_hooks!(
        change_status_request_from_script_hook
    );
}