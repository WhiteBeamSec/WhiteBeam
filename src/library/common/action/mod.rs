use crate::common::db;

pub struct ActionObject {
    pub alias: &'static str,
    pub function: fn(String, db::HookRow, i64, Vec<db::ArgumentRow>, bool, isize) -> (db::HookRow, Vec<crate::common::db::ArgumentRow>, bool, isize)
}

// Action template
macro_rules! build_action {
    ($alias:ident ($src_prog:ident, $hook:ident, $arg_id:ident, $args:ident, $do_return:ident, $return_value:ident) $body:block) => {
        #[allow(unused_imports)]
        use crate::common::event;
        #[cfg(target_os = "windows")]
        #[allow(unused_imports)]
        use crate::platforms::windows as platform;
        #[cfg(target_os = "linux")]
        #[allow(unused_imports)]
        use crate::platforms::linux as platform;
        #[cfg(target_os = "macos")]
        #[allow(unused_imports)]
        use crate::platforms::macos as platform;
        #[allow(non_snake_case)]
        #[allow(unused_assignments)]
        #[allow(unused_mut)]
        pub fn $alias ($src_prog: String, mut $hook: crate::common::db::HookRow, $arg_id: i64, mut $args: Vec<crate::common::db::ArgumentRow>, mut $do_return: bool, mut $return_value: isize) -> (crate::common::db::HookRow, Vec<crate::common::db::ArgumentRow>, bool, isize) {
            $body
            ($hook, $args, $do_return, $return_value)
        }
        #[linkme::distributed_slice(crate::common::action::ACTION_INDEX)]
        pub static ACTION: crate::common::action::ActionObject = crate::common::action::ActionObject { alias: stringify!($alias), function: $alias };
    };
}

// Load action modules
mod actions {
    automod::dir!(pub "src/library/common/action/actions");
}

// Collect actions in distributed slice
#[linkme::distributed_slice]
pub static ACTION_INDEX: [ActionObject] = [..];

pub fn process_action(src_prog: String, rule: db::RuleRow, hook: db::HookRow, args: Vec<db::ArgumentRow>) -> (db::HookRow, Vec<db::ArgumentRow>, bool, isize) {
    let action: &str = &rule.action;
    let arg_id: i64 = rule.arg;
    let do_return = false;
    let return_value = 0 as isize;
    match ACTION_INDEX.iter().find(|a| a.alias == action) {
        Some(action) => {(action.function)(src_prog, hook, arg_id, args, do_return, return_value)}
        None => panic!("WhiteBeam: Invalid action: {}", action)
    }
}
