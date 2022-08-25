use sbbw_exec::Params;

use super::{MethodActions, SbbwResponse};

pub fn register(action: &mut MethodActions) {
    action.insert("", Box::new());
}
