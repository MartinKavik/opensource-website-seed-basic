use seed::{prelude::*, *};
use crate::{Msg, iter_projects_by_tag, Urls, Project};
use super::partial::view_category;

// ------ ------
//     View
// ------ ------

pub fn view(tag: &str, projects: &[Project], base_url: &Url) -> Vec<Node<Msg>> {
    vec![
        view_section_back(base_url),
        view_section_projects(tag, projects, base_url),
    ]
}

fn view_section_back(base_url: &Url) -> Node<Msg> {
    section![C!["full-width-section cover"],
        div![C!["container"],
            a![attrs!{At::Href => Urls::new(base_url).home()},
                h1![
                    i![C!["fa", "fa-long-arrow-left"]],
                    "Back",
                ]
            ]
        ]
    ]
}

fn view_section_projects(tag: &str, projects: &[Project], base_url: &Url) -> Node<Msg> {
    section![id!("projects"),
        div![C!["container"],
            view_category(tag, iter_projects_by_tag(projects, tag), base_url)
        ]
    ]
}
