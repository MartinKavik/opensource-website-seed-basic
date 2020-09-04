use seed::{prelude::*, *};
use crate::{Msg, Project, Urls};

pub fn repo_url(project_name: &str) -> String {
    format!("https://github.com/EmbarkStudios/{}", project_name)
}

pub fn star_button_src(project_name: &str) -> String {
    format!("https://ghbtns.com/github-btn.html?user=EmbarkStudios&repo={}&type=star&count=true&size=large", project_name)
}

pub fn view_category<'a>(tag: &str, projects: impl Iterator<Item = &'a Project>, base_url: &Url) -> Node<Msg> {
    section![
        C!["category"],
        h2![
            C!["category-title"],
            "Our ",
            span![C!["category-tag"], tag],
            " projects"
        ],
        div![
            id!(tag),
            C!["projects-container"],
            projects.map(|project| view_project(project, base_url))
        ]
    ]
}

pub fn view_project(project: &Project, base_url: &Url) -> Node<Msg> {
    a![
        C!["project"],
        attrs! {At::Href => repo_url(&project.name)},
        div![
            h3![
                C!["title"],
                span![C!["emoji"], &project.emoji],
                " ",
                &project.name,
            ],
            p![raw![&project.description],],
            view_tags(project.tags.iter(), base_url)
        ],
        view_star_button(&project.name),
    ]
}

pub fn view_tags<'a>(tags: impl Iterator<Item = &'a String>, base_url: &Url) -> Node<Msg> {
    div![
        C!["tags"],
        tags.map(|tag| {
            div![
                C!["tag", format!("tag-{}", tag)],
                a![attrs! {At::Href => Urls::new(base_url).tags(tag)}, tag]
            ]
        })
    ]
}

pub fn view_star_button(project_name: &str) -> Node<Msg> {
    iframe![
        C!["star-button"],
        style! {St::Border => 0},
        attrs! {
            At::Src => star_button_src(project_name),
            At::Width => px(160),
            At::Height => px(30),
        }
    ]
}
