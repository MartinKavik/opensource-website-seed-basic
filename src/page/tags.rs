use seed::{prelude::*, *};
use crate::{Msg, Model, view_category, iter_projects_by_tag, Urls, Project};

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, tag: &str) -> Vec<Node<Msg>> {
    vec![
        view_section_back(&model.base_url),
        view_section_projects(tag, &model.data.projects, &model.base_url),
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
