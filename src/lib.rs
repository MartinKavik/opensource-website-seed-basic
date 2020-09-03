#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use serde::Deserialize;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::DataFetched(async {
        fetch("/public/data.json")
            .await?
            .check_status()?
            .json()
            .await
    }.await)});

    Model {
        data: Data {
            projects: Vec::new(),
        }
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    data: Data
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Data {
    projects: Vec<Project>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Project {
    name: String,
    emoji: String,
    tags: Vec<String>,
    description: String,
    #[serde(default)]
    featured: bool,
    extended_description: Option<String>,
    feature_image: Option<String>,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    DataFetched(fetch::Result<Data>),
    OpenSearch,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::DataFetched(Ok(data)) => {
            model.data = data;
            model.data.projects.sort_by_cached_key(|project| project.name.clone());
        },
        Msg::DataFetched(Err(error)) => error!(error),
        Msg::OpenSearch => log!("Msg::OpenSearch"),
    }
}

// ------ ------
// View Helpers
// ------ ------

fn repo_url(project_name: &str) -> String {
    format!("https://github.com/EmbarkStudios/{}", project_name)
}

fn star_button_src(project_name: &str) -> String {
    format!("https://ghbtns.com/github-btn.html?user=EmbarkStudios&repo={}&type=star&count=true&size=large", project_name)
}

fn view_category<'a>(tag: &str, projects: impl Iterator<Item = &'a Project>) -> Node<Msg> {
    section![C!["category"],
        h2![C!["category-title"],
            "Our ",
            span![C!["category-tag"], tag],
            " projects"
        ],
        div![id!(tag), C!["projects-container"],
            projects.map(view_project)
        ]
    ]
}

fn view_project(project: &Project) -> Node<Msg> {
    a![C!["project"],
        attrs!{At::Href => repo_url(&project.name)},
        div![
            h3![C!["title"],
                span![C!["emoji"],
                    &project.emoji
                ],
                " ",
                &project.name,
            ],
            p![
                raw![&project.description],
            ],
            view_tags(project.tags.iter())
        ],
        view_star_button(&project.name),
    ]
}

fn view_tags<'a>(tags: impl Iterator<Item = &'a String>) -> Node<Msg> {
    div![C!["tags"],
        tags.map(|tag| {
            div![C!["tag", format!("tag-{}", tag)],
                a![attrs!{At::Href => format!("/tags?tag={}", tag)},
                    tag
                ]
            ]
        })
    ]
}

fn view_star_button(project_name: &str) -> Node<Msg> {
    iframe![C!["star-button"],
        style!{St::Border => 0},
        attrs!{
            At::Src => star_button_src(project_name),
            At::Width => px(160),
            At::Height => px(30),
        }
    ]
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    let projects = &model.data.projects;

    vec![
        view_header(),
        view_search_overlay(),
        view_section_hero(),
        view_section_featured(projects.iter().filter(|project| project.featured)),
        view_section_blender(projects.iter().filter(|project| project.tags.iter().any(|tag| tag == "blender"))),
        view_section_rust(projects.iter().filter(|project| project.tags.iter().any(|tag| tag == "rust"))),
        view_section_projects(projects),
        view_section_sponsorship(),
        view_section_project_list(),
        view_section_newsletter(),
        view_section_contribute(),
    ]
}


fn view_header() -> Node<Msg> {
    // <header class="header">
    //     <a href="https://embark-studios.com"><img id="logo" src="./img/logo.png"/></a>
    //     <div>
    //       <a href="https://embark-studios.com" class="fa fa-globe"></a>
    //       <a href="https://twitter.com/EmbarkStudios" class="fa fa-twitter"></a>
    //       <a href="https://github.com/EmbarkStudios" class="fa fa-github"></a>
    //       <a href="#" class="fa fa-search search-icon" @click.prevent="openSearch"></a>
    //     </div>
    //   </header>
    header![C!["header"],
        a![attrs!{At::Href => "https://embark-studios.com"},
            img![id!("logo"), attrs!{At::Src => "/public/img/logo.png"}]
        ],
        div![
            a![C!["fa", "fa-globe"], attrs!{At::Href => "https://embark-studios.com"}],
            " ",
            a![C!["fa", "fa-twitter"], attrs!{At::Href => "https://twitter.com/EmbarkStudios"}],
            " ",
            a![C!["fa", "fa-github"], attrs!{At::Href => "https://github.com/EmbarkStudios"}],
            " ",
            a![C!["fa", "fa-search", "search-icon"], attrs!{At::Href => "#"}, 
                ev(Ev::Click, |event| { event.prevent_default(); Msg::OpenSearch })
            ],
        ]
    ]
}

fn view_search_overlay() -> Node<Msg> {
    // <div class="search-overlay" v-show="showSearch" @keyup.esc="closeSearch" style="display: none;">
    //     <div class="search-overlay__content">
    //       <span class="fa fa-close search-overlay__close" @click="closeSearch"></span>
    //       <input id="search-input" type="text" placeholder="Start typing..." v-model="search" ref="search">
    //       <div class="search-overlay__results">
    //         <a v-bind:href="repoUrl(p)" class="project" v-for="p in searchedProjects">
    //           <div>
    //             <h3 class="title">
    //               <span class="emoji">{{ p.emoji }}</span>
    //               {{ p.name }}
    //             </h3>
    //             <p>{{ p.description }}</p>
    //             <tags v-bind:tags="p.tags"></tags>
    //             <iframe class="star-button" v-bind:src="starButton(p)" frameborder="0" scrolling="0" width="160px" height="30px"></iframe>
    //           </div>
    //         </a>
    //       </div>
    //     </div>
    //   </div>
    div![]
}

fn view_section_hero() -> Node<Msg> {
    //     <section class="full-width-section cover" id="hero">
    //     <div class="container">
    //       <div>
    //         <h1>
    //           Embark ❤️ Open Source
    //         </h1>
    //         <h2>
    //           Empowering everyone to create
    //         </h2>
    //       </div>
    //
    //       <div>
    //         <p>
    //           Embark Studios is a Stockholm-based games studio, on a mission to blur the line between playing and making.
    //         </p>
    //         <p>
    //           Technology is reshaping our industry. We want to be part of this change, by exploring and applying the latest technology, by being honest and transparent in our relationship with each other and our community, and by allowing our curiosity to lead us down unexpected paths.
    //         </p>
    //         <p>
    //           In our open source work, we're exploring and pushing the boundaries of new technologies, and sharing our learnings with the community.
    //         </p>
    //         <a href="#contribute" class="button-primary">
    //           Contribute
    //         </a>
    //       </div>
    //     </div>
    //   </section>
    section![id!("hero"), C!["full-width-section", "cover"],
        div![C!["container"],
            div![
                h1![
                    "Embark ❤️ Open Source"
                ],
                h2![
                    "Empowering everyone to create",
                ]
            ],
            div![
                p![
                    "Embark Studios is a Stockholm-based games studio, on a mission to blur the line between playing and making.",
                ],
                p![
                    "Technology is reshaping our industry. We want to be part of this change, by exploring and applying the latest technology, by being honest and transparent in our relationship with each other and our community, and by allowing our curiosity to lead us down unexpected paths.",
                ],
                p![
                    "In our open source work, we're exploring and pushing the boundaries of new technologies, and sharing our learnings with the community.",
                ],
                a![C!["button-primary"], attrs!{At::Href => "#contribute"},
                    "Contribute"
                ]
            ]
        ]
    ]
}

fn view_section_featured<'a>(featured_projects: impl Iterator<Item = &'a Project>) -> Node<Msg> {
    //     <section id="featured">
    //     <div class="container">
    //       <h2>
    //         Featured Open Source Projects
    //       </h2>
    //       <div class="projects-container">
    //         <a v-bind:style="{ 'background-image': 'url(' + p.featureImage + ')' }" v-bind:href="repoUrl(p)" class="project project-featured" v-for="p in featuredProjects">
    //           <h3 class="title">
    //             <span class="emoji">{{ p.emoji }}</span>
    //             {{ p.name }}
    //           </h3>
    //           <p>
    //             {{ p.extendedDescription }}
    //           </p>
    //           <tags v-bind:tags="p.tags"></tags>
    //         </a>
    //       </div>
    //     </div>
    //   </section>
    section![id!("featured"),
        div![C!["container"],
            h2![
                "Featured Open Source Projects"
            ],
            div![C!["projects-container"],
                featured_projects.map(|project| {
                    let feature_image = if let Some(feature_image) = &project.feature_image {
                        feature_image
                    } else {
                        error!("feature image is missing");
                        return empty![];
                    };

                    let extended_description = if let Some(extended_description) = &project.extended_description {
                        extended_description
                    } else {
                        error!("extended_description is missing");
                        return empty![];
                    };

                    a![C!["project", "project-featured"],
                        style!{St::BackgroundImage => format!("url({})", feature_image)},
                        attrs!{At::Href => repo_url(&project.name)},
                        h3![C!["title"],
                            span![C!["emoji"],
                                &project.emoji
                            ],
                            &project.name,
                        ],
                        p![
                            &extended_description
                        ],
                        view_tags(project.tags.iter())
                    ]
                })
            ]
        ]
    ]
}

fn view_section_blender<'a>(blender_projects: impl Iterator<Item = &'a Project>) -> Node<Msg> {
    //   <section id="blender" class="full-width-section background-blue">
    //     <div class="container">
    //       <h1>
    //         <svg class="feature-logo" role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><title>Blender icon</title><path d="M12.51 13.214c.046-.8.438-1.506 1.03-2.006a3.424 3.424 0 0 1 2.212-.79c.85 0 1.631.3 2.211.79.592.5.983 1.206 1.028 2.005.045.823-.285 1.586-.865 2.153a3.389 3.389 0 0 1-2.374.938 3.393 3.393 0 0 1-2.376-.938c-.58-.567-.91-1.33-.865-2.152M7.35 14.831c.006.314.106.922.256 1.398a7.372 7.372 0 0 0 1.593 2.757 8.227 8.227 0 0 0 2.787 2.001 8.947 8.947 0 0 0 3.66.76 8.964 8.964 0 0 0 3.657-.772 8.285 8.285 0 0 0 2.785-2.01 7.428 7.428 0 0 0 1.592-2.762 6.964 6.964 0 0 0 .25-3.074 7.123 7.123 0 0 0-1.016-2.779 7.764 7.764 0 0 0-1.852-2.043h.002L13.566 2.55l-.02-.015c-.492-.378-1.319-.376-1.86.002-.547.382-.609 1.015-.123 1.415l-.001.001 3.126 2.543-9.53.01h-.013c-.788.001-1.545.518-1.695 1.172-.154.665.38 1.217 1.2 1.22V8.9l4.83-.01-8.62 6.617-.034.025c-.813.622-1.075 1.658-.563 2.313.52.667 1.625.668 2.447.004L7.414 14s-.069.52-.063.831zm12.09 1.741c-.97.988-2.326 1.548-3.795 1.55-1.47.004-2.827-.552-3.797-1.538a4.51 4.51 0 0 1-1.036-1.622 4.282 4.282 0 0 1 .282-3.519 4.702 4.702 0 0 1 1.153-1.371c.942-.768 2.141-1.183 3.396-1.185 1.256-.002 2.455.41 3.398 1.175.48.391.87.854 1.152 1.367a4.28 4.28 0 0 1 .522 1.706 4.236 4.236 0 0 1-.239 1.811 4.54 4.54 0 0 1-1.035 1.626"/></svg>
    //         Embark + Blender
    //       </h1>
    //       <p>
    //         We <a href="https://medium.com/embarkstudios/a-love-letter-to-blender-e54167c22193">recently announced</a> that Embark has become a corporate sponsor of Blender, a free and open source 3D creation software.
    //       </p>
    //       <p> 
    //         We think Blender is a great example of what happens when powerful software is made available for everyone to use, and communities start to work together to change the status quo.
    //       </p>
    //       <p> 
    //         We have also released an open source add-on featuring some of our day-to-day studio tools.
    //       </p>
    //       <a v-bind:href="repoUrl(p)" class="project" v-for="p in projectsWithTag('blender')">
    //         <div>
    //           <h3 class="title">
    //             <span class="emoji">{{ p.emoji }}</span>
    //             {{ p.name }}
    //           </h3>
    //           <p v-html="p.description"></p>
    //           <tags v-bind:tags="p.tags"></tags>
    //         </div>
    //         <iframe class="star-button" v-bind:src="starButton(p)" frameborder="0" scrolling="0" width="160px" height="30px"></iframe>
    //       </a>
    //       <a href="https://medium.com/embarkstudios/a-love-letter-to-blender-e54167c22193" class="button-primary background-grey">Learn More</a>
    //     </div>
    //   </section>
    section![id!("blender"), C!["full-width-section", "background-blue"],
        div![C!["container"],
            h1![
                svg![C!["feature-logo"], attrs!{At::from("role") => "img", At::ViewBox => "0 0 24 24"},
                    title!["Blender icon"],
                    path![attrs!{At::D => "M12.51 13.214c.046-.8.438-1.506 1.03-2.006a3.424 3.424 0 0 1 2.212-.79c.85 0 1.631.3 2.211.79.592.5.983 1.206 1.028 2.005.045.823-.285 1.586-.865 2.153a3.389 3.389 0 0 1-2.374.938 3.393 3.393 0 0 1-2.376-.938c-.58-.567-.91-1.33-.865-2.152M7.35 14.831c.006.314.106.922.256 1.398a7.372 7.372 0 0 0 1.593 2.757 8.227 8.227 0 0 0 2.787 2.001 8.947 8.947 0 0 0 3.66.76 8.964 8.964 0 0 0 3.657-.772 8.285 8.285 0 0 0 2.785-2.01 7.428 7.428 0 0 0 1.592-2.762 6.964 6.964 0 0 0 .25-3.074 7.123 7.123 0 0 0-1.016-2.779 7.764 7.764 0 0 0-1.852-2.043h.002L13.566 2.55l-.02-.015c-.492-.378-1.319-.376-1.86.002-.547.382-.609 1.015-.123 1.415l-.001.001 3.126 2.543-9.53.01h-.013c-.788.001-1.545.518-1.695 1.172-.154.665.38 1.217 1.2 1.22V8.9l4.83-.01-8.62 6.617-.034.025c-.813.622-1.075 1.658-.563 2.313.52.667 1.625.668 2.447.004L7.414 14s-.069.52-.063.831zm12.09 1.741c-.97.988-2.326 1.548-3.795 1.55-1.47.004-2.827-.552-3.797-1.538a4.51 4.51 0 0 1-1.036-1.622 4.282 4.282 0 0 1 .282-3.519 4.702 4.702 0 0 1 1.153-1.371c.942-.768 2.141-1.183 3.396-1.185 1.256-.002 2.455.41 3.398 1.175.48.391.87.854 1.152 1.367a4.28 4.28 0 0 1 .522 1.706 4.236 4.236 0 0 1-.239 1.811 4.54 4.54 0 0 1-1.035 1.626"}]
                ],
                " ",
                "Embark + Blender",
            ],
            p![
                "We ", a![attrs!{At::Href => "https://medium.com/embarkstudios/a-love-letter-to-blender-e54167c22193"}, "recently announced"], " that Embark has become a corporate sponsor of Blender, a free and open source 3D creation software.",
            ],
            p![
                "We have also released an open source add-on featuring some of our day-to-day studio tools.",
            ],
            blender_projects.map(view_project),
            a![C!["button-primary", "background-grey"], attrs!{At::Href => "https://medium.com/embarkstudios/a-love-letter-to-blender-e54167c22193"},
                "Learn More"
            ],
        ]
    ]
}

fn view_section_rust<'a>(rust_projects: impl Iterator<Item = &'a Project>) -> Node<Msg> {
    //     <section id="rust" class="full-width-section background-grey">
    //     <div class="container">
    //       <h1>
    //         🦀 Rust at Embark
    //       </h1>
    //       <p>
    //         When we started Embark, we chose Rust as our primary language for the long term future we are building. We love the safety and robustness of the language, the ability to write high performance, safe, and (mostly) bug free code and then fearlessly refactor and change it without common lifetime/ownership, memory safety or race condition problems.
    //       </p>
    //       <p>
    //         Possibly even more important is the openness and collaborative nature of the quickly growing ecosystem and community around Rust. With tens of thousands of open source crates on crates.io and a best-in-class package system, cargo, we truly believe Rust is a language for the future.
    //       </p>
    //       <a href="https://embark.rs" class="button-primary background-red">Learn More</a>
    //       <project-category
    //         tag="rust"
    //         v-bind:projects="projectsWithTag('rust')"
    //       ></project-category>
    //     </div>
    //   </section>
    section![id!("rust"), C!["full-width-section", "background-grey"],
        div![C!["container"],
            h1![
                "🦀 Rust at Embark"
            ],
            p![
                "When we started Embark, we chose Rust as our primary language for the long term future we are building. We love the safety and robustness of the language, the ability to write high performance, safe, and (mostly) bug free code and then fearlessly refactor and change it without common lifetime/ownership, memory safety or race condition problems.",
            ],
            p![
                "Possibly even more important is the openness and collaborative nature of the quickly growing ecosystem and community around Rust. With tens of thousands of open source crates on crates.io and a best-in-class package system, cargo, we truly believe Rust is a language for the future.",
            ],
            a![C!["button-primary", "background-red"], attrs!{At::Href => "https://embark.rs"},
                "Learn More",
            ],
            view_category("rust", rust_projects)
        ]
    ]
}

fn view_section_projects(projects: &[Project]) -> Node<Msg> {
    //     <section>
    //     <div class="container">
    //       <project-category
    //         tag="go"
    //         v-bind:projects="projectsWithTag('go')"
    //       ></project-category>

    //       <project-category
    //         tag="web"
    //         v-bind:projects="projectsWithTag('web')"
    //       ></project-category>
    //     </div>
    //   </section>

    let go_projects = projects.iter().filter(|project| project.tags.iter().any(|tag| tag == "go"));
    let web_projects = projects.iter().filter(|project| project.tags.iter().any(|tag| tag == "web"));

    section![
        div![C!["container"],
            view_category("go", go_projects),
            view_category("web", web_projects),
        ]
    ]
}

fn view_section_sponsorship() -> Node<Msg> {
    //     <section class="full-width-section background-grey" id="sponsorship">
    //     <div class="container">
    //       <h1>
    //         Sponsorship
    //       </h1>
    //       <p>
    //         We believe that open source creators are integral to the success of the developer ecosystem. We offer monetary sponsorship to several individuals and projects via Patreon, GitHub and OpenCollective. You can see who we're currently supporting below:
    //       </p>
    //       <div class="logo-container">
    //         <a href="https://opencollective.com/embarkstudios">
    //           <svg class="logo-image" role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><title>Open Collective icon</title><path d="M21.86 5.17a11.94 11.94 0 0 1 0 13.66l-3.1-3.1a7.68 7.68 0 0 0 0-7.46l3.1-3.1zm-3.03-3.03l-3.1 3.1a7.71 7.71 0 1 0 0 13.51l3.1 3.11a12 12 0 1 1 0-19.73 M21.86 5.17a11.94 11.94 0 0 1 0 13.66l-3.1-3.1a7.68 7.68 0 0 0 0-7.46l3.1-3.1z"/></svg>
    //         </a>
    //         <a href="https://www.patreon.com/embarkstudios/creators">
    //           <svg class="logo-image" role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><title>Patreon icon</title><path d="M15.386.524c-4.764 0-8.64 3.876-8.64 8.64 0 4.75 3.876 8.613 8.64 8.613 4.75 0 8.614-3.864 8.614-8.613C24 4.4 20.136.524 15.386.524M.003 23.537h4.22V.524H.003"/></svg>
    //         </a>
    //         <a href="https://github.com/embark-studios">
    //           <svg class="logo-image" role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><title>GitHub icon</title><path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/></svg>
    //         </a>
    //         <a href="https://fund.blender.org/">
    //           <svg class="logo-image" role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><title>Blender icon</title><path d="M12.51 13.214c.046-.8.438-1.506 1.03-2.006a3.424 3.424 0 0 1 2.212-.79c.85 0 1.631.3 2.211.79.592.5.983 1.206 1.028 2.005.045.823-.285 1.586-.865 2.153a3.389 3.389 0 0 1-2.374.938 3.393 3.393 0 0 1-2.376-.938c-.58-.567-.91-1.33-.865-2.152M7.35 14.831c.006.314.106.922.256 1.398a7.372 7.372 0 0 0 1.593 2.757 8.227 8.227 0 0 0 2.787 2.001 8.947 8.947 0 0 0 3.66.76 8.964 8.964 0 0 0 3.657-.772 8.285 8.285 0 0 0 2.785-2.01 7.428 7.428 0 0 0 1.592-2.762 6.964 6.964 0 0 0 .25-3.074 7.123 7.123 0 0 0-1.016-2.779 7.764 7.764 0 0 0-1.852-2.043h.002L13.566 2.55l-.02-.015c-.492-.378-1.319-.376-1.86.002-.547.382-.609 1.015-.123 1.415l-.001.001 3.126 2.543-9.53.01h-.013c-.788.001-1.545.518-1.695 1.172-.154.665.38 1.217 1.2 1.22V8.9l4.83-.01-8.62 6.617-.034.025c-.813.622-1.075 1.658-.563 2.313.52.667 1.625.668 2.447.004L7.414 14s-.069.52-.063.831zm12.09 1.741c-.97.988-2.326 1.548-3.795 1.55-1.47.004-2.827-.552-3.797-1.538a4.51 4.51 0 0 1-1.036-1.622 4.282 4.282 0 0 1 .282-3.519 4.702 4.702 0 0 1 1.153-1.371c.942-.768 2.141-1.183 3.396-1.185 1.256-.002 2.455.41 3.398 1.175.48.391.87.854 1.152 1.367a4.28 4.28 0 0 1 .522 1.706 4.236 4.236 0 0 1-.239 1.811 4.54 4.54 0 0 1-1.035 1.626"/></svg>
    //         </a>
    //       </div>
    //     </div>
    //   </section>
    section![id!("sponsorship"), C!["full-width-section", "background-grey"],
        div![C!["container"],
            h1!["Sponsorship"],
            p![
                "We believe that open source creators are integral to the success of the developer ecosystem. We offer monetary sponsorship to several individuals and projects via Patreon, GitHub and OpenCollective. You can see who we're currently supporting below:"
            ],
            div![C!["logo-container"],
                a![attrs!{At::Href => "https://opencollective.com/embarkstudios"},
                    svg![C!["logo-image"], attrs!{At::from("role") => "img", At::ViewBox => "0 0 24 24"},
                        title!["Open Collective icon"],
                        path![attrs!{At::D => "M21.86 5.17a11.94 11.94 0 0 1 0 13.66l-3.1-3.1a7.68 7.68 0 0 0 0-7.46l3.1-3.1zm-3.03-3.03l-3.1 3.1a7.71 7.71 0 1 0 0 13.51l3.1 3.11a12 12 0 1 1 0-19.73 M21.86 5.17a11.94 11.94 0 0 1 0 13.66l-3.1-3.1a7.68 7.68 0 0 0 0-7.46l3.1-3.1z"}],
                    ]
                ],
                a![attrs!{At::Href => "https://www.patreon.com/embarkstudios/creators"},
                    svg![C!["logo-image"], attrs!{At::from("role") => "img", At::ViewBox => "0 0 24 24"},
                        title!["Patreon icon"],
                        path![attrs!{At::D => "M15.386.524c-4.764 0-8.64 3.876-8.64 8.64 0 4.75 3.876 8.613 8.64 8.613 4.75 0 8.614-3.864 8.614-8.613C24 4.4 20.136.524 15.386.524M.003 23.537h4.22V.524H.003"}],
                    ]
                ],
                a![attrs!{At::Href => "https://github.com/embark-studios"},
                    svg![C!["logo-image"], attrs!{At::from("role") => "img", At::ViewBox => "0 0 24 24"},
                        title!["GitHub icon"],
                        path![attrs!{At::D => "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"}],
                    ]
                ],
                a![attrs!{At::Href => "https://fund.blender.org/"},
                    svg![C!["logo-image"], attrs!{At::from("role") => "img", At::ViewBox => "0 0 24 24"},
                        title!["Blender icon"],
                        path![attrs!{At::D => "M12.51 13.214c.046-.8.438-1.506 1.03-2.006a3.424 3.424 0 0 1 2.212-.79c.85 0 1.631.3 2.211.79.592.5.983 1.206 1.028 2.005.045.823-.285 1.586-.865 2.153a3.389 3.389 0 0 1-2.374.938 3.393 3.393 0 0 1-2.376-.938c-.58-.567-.91-1.33-.865-2.152M7.35 14.831c.006.314.106.922.256 1.398a7.372 7.372 0 0 0 1.593 2.757 8.227 8.227 0 0 0 2.787 2.001 8.947 8.947 0 0 0 3.66.76 8.964 8.964 0 0 0 3.657-.772 8.285 8.285 0 0 0 2.785-2.01 7.428 7.428 0 0 0 1.592-2.762 6.964 6.964 0 0 0 .25-3.074 7.123 7.123 0 0 0-1.016-2.779 7.764 7.764 0 0 0-1.852-2.043h.002L13.566 2.55l-.02-.015c-.492-.378-1.319-.376-1.86.002-.547.382-.609 1.015-.123 1.415l-.001.001 3.126 2.543-9.53.01h-.013c-.788.001-1.545.518-1.695 1.172-.154.665.38 1.217 1.2 1.22V8.9l4.83-.01-8.62 6.617-.034.025c-.813.622-1.075 1.658-.563 2.313.52.667 1.625.668 2.447.004L7.414 14s-.069.52-.063.831zm12.09 1.741c-.97.988-2.326 1.548-3.795 1.55-1.47.004-2.827-.552-3.797-1.538a4.51 4.51 0 0 1-1.036-1.622 4.282 4.282 0 0 1 .282-3.519 4.702 4.702 0 0 1 1.153-1.371c.942-.768 2.141-1.183 3.396-1.185 1.256-.002 2.455.41 3.398 1.175.48.391.87.854 1.152 1.367a4.28 4.28 0 0 1 .522 1.706 4.236 4.236 0 0 1-.239 1.811 4.54 4.54 0 0 1-1.035 1.626"}],
                    ]
                ],
            ]
        ]    
    ]
}

fn view_section_project_list() -> Node<Msg> {
//     <section>
//     <div class="container">
//       <h3>
//         Projects A-Z
//       </h3>
//       <ul class="projects-list">
//         <a v-for="p in alphabetisedProjects" v-bind:href="repoUrl(p)"><li><span>{{ p.emoji }} {{ p.name }}</span><tags v-bind:tags="p.tags"></tags></li></a>
//       </ul>
//     </div>
//   </section>
    section![
        div![C!["container"],
            h3![
                "Projects A-Z"
            ],
            ul![C!["projects-list"],
                li![
                    "@TODO projects list"
                ]
            ]
        ]
    ]
}

fn view_section_newsletter() -> Node<Msg> {
    //     <section class="full-width-section background-grey" id="newsletter">
    //     <div class="container">
    //       <h1>
    //         Stay in the loop
    //       </h1>
    //       <p>Stay up to date with new open source projects and developer events from Embark with our developer newsletter.</p>
    //       <p>Recent Editions:</p>
    //       <ul id="archive-list">
    //         <div class="display_archive">
    //           <li class="campaign">02/03/2020 - <a href="http://eepurl.com/gTOOB5" title="Watch 2 New Talks from Embark! - Embark Dev Newsletter 004" target="_blank">Watch 2 New Talks from Embark! - Embark Dev Newsletter 004</a></li>
    //           <li class="campaign">23/01/2020 - <a href="http://eepurl.com/gPFCp1" title="A sneak peek at our experiments and upcoming events | Embark Dev Newsletter 003" target="_blank">A sneak peek at our experiments and upcoming events | Embark Dev Newsletter 003</a></li>
    //           <li class="campaign">12/06/2019 - <a href="http://eepurl.com/gJh88j" title="New Blender add-on, and a peek inside Rust development at Embark: Embark Dev Newsletter 002" target="_blank">New Blender add-on, and a peek inside Rust development at Embark: Embark Dev Newsletter 002</a></li>
    //           <li class="campaign">11/08/2019 - <a href="http://eepurl.com/gI3v89" title="Rust, Blender, Hacktoberfest, and more: Newsletter 001 from Embark" target="_blank">Rust, Blender, Hacktoberfest, and more: Newsletter 001 from Embark</a></li>
    //         </div>
    //       </ul>
    //       <a class="button-primary" href="https://mailchi.mp/3608b3a1adca/embarkdev">Sign up</a>
    //     </div>
    //   </section>
    section![id!("newsletter"), C!["full-width-section", "background-grey"],
        div![C!["container"],
            h1![
                "Stay in the loop"
            ],
            p![
                "Stay up to date with new open source projects and developer events from Embark with our developer newsletter.",
            ],
            p!["Recent Editions:"],
            ul![id!("archive-list"),
                div![C!["display_archive"],
                    li![C!["campaign"],
                        "02/03/2020 - ", 
                        a![attrs!{At::Href => "http://eepurl.com/gTOOB5", At::Title => "Watch 2 New Talks from Embark! - Embark Dev Newsletter 004", At::Target => "_blank"},
                            "Watch 2 New Talks from Embark! - Embark Dev Newsletter 004"
                        ],
                    ],
                    li![C!["campaign"],
                        "23/01/2020 - ", 
                        a![attrs!{At::Href => "http://eepurl.com/gPFCp1", At::Title => "A sneak peek at our experiments and upcoming events | Embark Dev Newsletter 003", At::Target => "_blank"},
                            "A sneak peek at our experiments and upcoming events | Embark Dev Newsletter 003"
                        ],
                    ],
                    li![C!["campaign"],
                        "12/06/2019 - ", 
                        a![attrs!{At::Href => "http://eepurl.com/gJh88j", At::Title => "New Blender add-on, and a peek inside Rust development at Embark: Embark Dev Newsletter 002", At::Target => "_blank"},
                            "New Blender add-on, and a peek inside Rust development at Embark: Embark Dev Newsletter 002"
                        ],
                    ],
                    li![C!["campaign"],
                        "11/08/2019 - ", 
                        a![attrs!{At::Href => "http://eepurl.com/gI3v89", At::Title => "Rust, Blender, Hacktoberfest, and more: Newsletter 001 from Embark", At::Target => "_blank"},
                            "Rust, Blender, Hacktoberfest, and more: Newsletter 001 from Embark"
                        ],
                    ],
                ]
            ],
            a![C!["button-primary"], attrs!{At::Href => "https://mailchi.mp/3608b3a1adca/embarkdev"},
                "Sign up"
            ]
        ]
    ]
}

fn view_section_contribute() -> Node<Msg> {
    // <section class="full-width-section" id="contribute">
    //     <div class="container">
    //       <h1>
    //         Want to get involved?
    //       </h1>
    //       <p>All projects created at Embark Studios are open for contribution. We welcome contributions from people of all backgrounds who are interested in making great software with us. We need <b>you</b> to help us achieve our goal of empowering everyone to create.</p>
    //       <a href="https://github.com/EmbarkStudios/opensource-website/blob/main/CODE_OF_CONDUCT.md"><h3>Read our Code of Conduct →</h3></a>
    //       <a href="https://github.com/search?q=user:EmbarkStudios+state:open"><h3>Check out open issues →</h3></a>
    //       <a href="https://github.com/EmbarkStudios/opensource-website/blob/main/CONTRIBUTING.md" class="button-primary">Get Started!</a>
    //       <p>If you want to collaborate with Embark, you can reach out to us at <a href="mailto:opensource@embark-studios.com">opensource@embark-studios.com</a>.</p>
    //       <p>We're also looking for passionate people to <a href="https://embark-studios.com/jobs">join our team</a> in Stockholm!</p>
    //     </div>
    //   </section>
    section![id!("contribute"), C!["full-width-section"],
        div![C!["container"],
            h1![
                "Want to get involved?"
            ],
            p![
                "All projects created at Embark Studios are open for contribution. We welcome contributions from people of all backgrounds who are interested in making great software with us. We need ", b!["you"], " to help us achieve our goal of empowering everyone to create."
            ],
            a![attrs!{At::Href => "https://github.com/EmbarkStudios/opensource-website/blob/main/CODE_OF_CONDUCT.md"},
                h3!["Read our Code of Conduct →"],
            ],
            a![attrs!{At::Href => "https://github.com/search?q=user:EmbarkStudios+state:open"},
                h3!["Check out open issues →"],
            ],
            a![C!["button-primary"], attrs!{At::Href => "https://github.com/EmbarkStudios/opensource-website/blob/main/CONTRIBUTING.md"},
                "Get Started!",
            ],
            p![
                "If you want to collaborate with Embark, you can reach out to us at ",
                a![attrs!{At::Href => "mailto:opensource@embark-studios.com"}, "opensource@embark-studios.com"],
                ".",
            ],
            p![
                "We're also looking for passionate people to ",
                a![attrs!{At::Href => "https://embark-studios.com/jobs"}, "join our team"],
                " in Stockholm!",
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
