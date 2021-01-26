use crate::common::db;

// Action template
macro_rules! build_action {
    ($func_name:ident ($src_prog: ident, $hooked_fn: ident, $arg_id:ident, $args:ident, $do_return:ident, $return_value:ident) $body:block) => {
        #[allow(unused_assignments)]
        #[allow(unused_mut)]
        pub fn $func_name ($src_prog: String, mut $hooked_fn: String, $arg_id: i64, mut $args: Vec<crate::common::db::ArgumentRow>, mut $do_return: bool, mut $return_value: isize) -> (String, Vec<crate::common::db::ArgumentRow>, bool, isize) {
            $body
            ($hooked_fn, $args, $do_return, $return_value)
        }
    };
}

// Load action modules
mod actions {
    automod::dir!(pub "src/library/common/action/actions");
}

pub fn process_action(src_prog: String, rule: db::RuleRow, hooked_fn: String, args: Vec<db::ArgumentRow>) -> (String, Vec<db::ArgumentRow>, bool, isize) {
    let action: &str = &rule.action;
    let arg_id: i64 = rule.arg;
    let do_return = false;
    let return_value = 0 as isize;
    // TODO: Use automod to generate this
    match action {
        "VerifyCanExecute" => actions::verify_can_execute(src_prog, hooked_fn, arg_id, args, do_return, return_value),
        "VerifyFileHash" => actions::verify_file_hash(src_prog, hooked_fn, arg_id, args, do_return, return_value),
        "FilterEnvironment" => actions::filter_environment(src_prog, hooked_fn, arg_id, args, do_return, return_value),
        "ConsumeVariadic" => actions::consume_variadic(src_prog, hooked_fn, arg_id, args, do_return, return_value),
        "OverrideFunction" => actions::override_function(src_prog, hooked_fn, arg_id, args, do_return, return_value),
        _ => panic!("WhiteBeam: Invalid action: {}", action)
    }
}
