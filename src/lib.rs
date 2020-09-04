#![allow(
    clippy::wildcard_imports,
    clippy::non_ascii_literal,
    clippy::must_use_candidate
)]

use seed::{prelude::*, *};
use serde::Deserialize;

mod page;

// -- Url parts --
const TAGS: &str = "tags";
const TAGS_TAG_PARAMETER: &str = "tag";

type Tag = String;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged).perform_cmd(async {
        Msg::DataFetched(
            async {
                fetch("/public/data.json")
                    .await?
                    .check_status()?
                    .json()
                    .await
            }
            .await,
        )
    });

    Model {
        base_url: url.to_base_url(),
        data: Data {
            projects: Vec::new(),
        },
        show_search: false,
        search_query: String::new(),
        search_input_element: ElRef::default(),
        page: Page::init(url),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    data: Data,
    show_search: bool,
    search_query: String,
    search_input_element: ElRef<web_sys::HtmlInputElement>,
    page: Page,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    projects: Vec<Project>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    name: String,
    emoji: String,
    tags: Vec<String>,
    description: String,
    #[serde(default)]
    featured: bool,
    extended_description: Option<String>,
    feature_image: Option<String>,
}

// ------ Page ------

enum Page {
    Home,
    Tags(Tag),
}

impl Page {
    fn init(mut url: Url) -> Self {
        let selected_tag = url
            .search_mut()
            .remove(TAGS_TAG_PARAMETER)
            .and_then(|mut values| values.pop());

        match (url.remaining_path_parts().as_slice(), selected_tag) {
            ([TAGS], Some(tag)) => Self::Tags(tag),
            _ => Self::Home,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url().set_search(UrlSearch::default())
    }
    pub fn tags(self, tag: &str) -> Url {
        self.base_url()
            .add_path_part(TAGS)
            .set_search(UrlSearch::new(vec![(TAGS_TAG_PARAMETER, vec![tag])]))
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    DataFetched(fetch::Result<Data>),
    ToggleSearch,
    CloseSearch,
    SearchQueryChanged(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);

            window().scroll_to_with_scroll_to_options(web_sys::ScrollToOptions::new().top(0.));
        }
        Msg::DataFetched(Ok(data)) => {
            model.data = data;
            model
                .data
                .projects
                .sort_by_cached_key(|project| project.name.clone());
        }
        Msg::DataFetched(Err(error)) => error!(error),
        Msg::ToggleSearch => {
            if model.show_search {
                model.show_search = false;
            } else {
                model.show_search = true;

                let search_input_element = model.search_input_element.clone();
                orders.after_next_render(move |_| {
                    let input_element = search_input_element.get().expect("input_element");
                    input_element.focus().expect("focus input_element");
                    input_element.select();
                });
            }
        }
        Msg::CloseSearch => model.show_search = false,
        Msg::SearchQueryChanged(query) => model.search_query = query,
    }
}

// ------ ------
// View Helpers
// ------ ------

pub fn iter_projects_by_tag<'a>(
    projects: &'a [Project],
    tag: &'a str,
) -> impl Iterator<Item = &'a Project> {
    projects.iter().filter(move |project| {
        project
            .tags
            .iter()
            .any(|project_tag| project_tag.as_str() == tag)
    })
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    let projects = &model.data.projects;
    let base_url = &model.base_url;

    let search_results = projects.iter().filter(|project| {
        project
            .name
            .to_lowercase()
            .contains(&model.search_query.to_lowercase())
    });

    nodes![
        view_header(),
        view_search_overlay(
            model.show_search,
            &model.search_query,
            search_results,
            &model.search_input_element,
            base_url,
        ),
        match &model.page {
            Page::Home => page::home::view(projects, base_url),
            Page::Tags(tag) => page::tags::view(tag, projects, base_url),
        }
    ]
}

fn view_header() -> Node<Msg> {
    header![
        C!["header"],
        a![
            attrs! {At::Href => "https://embark-studios.com"},
            img![id!("logo"), attrs! {At::Src => "/public/img/logo.png"}]
        ],
        div![
            a![
                C!["fa", "fa-globe"],
                attrs! {At::Href => "https://embark-studios.com"}
            ],
            " ",
            a![
                C!["fa", "fa-twitter"],
                attrs! {At::Href => "https://twitter.com/EmbarkStudios"}
            ],
            " ",
            a![
                C!["fa", "fa-github"],
                attrs! {At::Href => "https://github.com/EmbarkStudios"}
            ],
            " ",
            a![
                C!["fa", "fa-search", "search-icon"],
                attrs! {At::Href => "#"},
                ev(Ev::Click, |event| {
                    event.prevent_default();
                    Msg::ToggleSearch
                })
            ],
        ]
    ]
}

fn view_search_overlay<'a>(
    show_search: bool,
    search_query: &str,
    search_results: impl Iterator<Item = &'a Project>,
    search_input_element: &ElRef<web_sys::HtmlInputElement>,
    base_url: &Url,
) -> Node<Msg> {
    div![
        C!["search-overlay"],
        // @TODO remove style! below ; custom style to see at least something
        style! {
            St::Position => "absolute",
            St::Background => "white",
            St::MaxHeight => vh(85),
            St::OverflowY => "auto",
        },
        style! {
            St::Display => if show_search { "block" } else { "none" },
        },
        keyboard_ev(
            Ev::KeyUp,
            |event| IF!(event.key() == "Escape" => Msg::CloseSearch)
        ),
        div![
            C!["search-overlay__content"],
            span![
                C!["fa", "fa-close", "search-overlay__close"],
                ev(Ev::Click, |_| Msg::CloseSearch)
            ],
            input![
                id!("search-input"),
                el_ref(search_input_element),
                attrs! {
                    At::Type => "text",
                    At::Placeholder => "Start typing...",
                    At::Value => search_query,
                },
                input_ev(Ev::Input, Msg::SearchQueryChanged),
            ],
            div![
                C!["search-overlay__results"],
                search_results.map(|project| page::partial::view_project(project, base_url))
            ]
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
