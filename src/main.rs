#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {
        stringify!($func)
    };
}

#[macro_use]

pub mod ai_functions;
pub mod apis;
pub mod helpers;
pub mod models;

use helpers::command_lines::get_user_response;
fn main() {
    let usr_req = get_user_response("What webserver are we building today");

    dbg!(usr_req);
}
