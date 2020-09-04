use seed::{prelude::*, *};
use crate::{Msg, Model};

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, tag: &str) -> Vec<Node<Msg>> {
    vec![
        div!["Tags page"]
    ]
}
