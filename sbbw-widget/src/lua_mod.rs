extern crate hlua;

use hlua::Lua;

use std::env;

// pub fn generate_sbbw_lib(lua: &mut L) where L: hlua::AsMutLua<'lua>{
//     lua.set("sbbw", [
//         ("SO", std::env::consts::OS),
//         ("version", env!("CARGO_PKG_VERSION")),
//     ]);
// }
