use seed::{prelude::*, *};
use crate::{Msg, Project, iter_projects_by_tag};
use super::partial::{repo_url, view_category, view_tags, view_project};

// ------ ------
//     View
// ------ ------

pub fn view(projects: &[Project], base_url: &Url) -> Vec<Node<Msg>> {
    let featured_projects = projects.iter().filter(|project| project.featured);

    vec![
        view_section_hero(),
        view_section_featured(featured_projects, base_url),
        view_section_blender(iter_projects_by_tag(projects, "blender"), base_url),
        view_section_rust(iter_projects_by_tag(projects, "rust"), base_url),
        view_section_projects(projects, base_url),
        view_section_sponsorship(),
        view_section_project_list(projects.iter(), base_url),
        view_section_newsletter(),
        view_section_contribute(),
    ]
}

fn view_section_hero() -> Node<Msg> {
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

fn view_section_featured<'a>(featured_projects: impl Iterator<Item = &'a Project>, base_url: &Url) -> Node<Msg> {
    section![
        id!("featured"),
        div![
            C!["container"],
            h2!["Featured Open Source Projects"],
            div![
                C!["projects-container"],
                featured_projects.map(|project| {
                    let feature_image = if let Some(feature_image) = &project.feature_image {
                        feature_image
                    } else {
                        error!("feature image is missing");
                        return empty![];
                    };

                    let extended_description =
                        if let Some(extended_description) = &project.extended_description {
                            extended_description
                        } else {
                            error!("extended_description is missing");
                            return empty![];
                        };

                    a![
                        C!["project", "project-featured"],
                        style! {St::BackgroundImage => format!("url({})", feature_image)},
                        attrs! {At::Href => repo_url(&project.name)},
                        h3![
                            C!["title"],
                            span![C!["emoji"], &project.emoji],
                            &project.name,
                        ],
                        p![&extended_description],
                        view_tags(project.tags.iter(), base_url)
                    ]
                })
            ]
        ]
    ]
}

fn view_section_blender<'a>(blender_projects: impl Iterator<Item = &'a Project>, base_url: &Url) -> Node<Msg> {
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
            blender_projects.map(|project| view_project(project, base_url)),
            a![C!["button-primary", "background-grey"], attrs!{At::Href => "https://medium.com/embarkstudios/a-love-letter-to-blender-e54167c22193"},
                "Learn More"
            ],
        ]
    ]
}

fn view_section_rust<'a>(rust_projects: impl Iterator<Item = &'a Project>, base_url: &Url) -> Node<Msg> {
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
            view_category("rust", rust_projects, base_url)
        ]
    ]
}

fn view_section_projects(projects: &[Project], base_url: &Url) -> Node<Msg> {
    section![div![
        C!["container"],
        view_category("go", iter_projects_by_tag(projects, "go"), base_url),
        view_category("web", iter_projects_by_tag(projects, "web"), base_url),
    ]]
}

fn view_section_sponsorship() -> Node<Msg> {
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
                ]
            ]
        ]
    ]
}

fn view_section_project_list<'a>(projects: impl Iterator<Item = &'a Project>, base_url: &Url) -> Node<Msg> {
    section![div![
        C!["container"],
        h3!["Projects A-Z"],
        ul![
            C!["projects-list"],
            projects.map(|project| {
                a![
                    attrs! {At::Href => repo_url(&project.name)},
                    li![
                        span![&project.emoji, " ", &project.name,],
                        view_tags(project.tags.iter(), base_url),
                    ]
                ]
            })
        ]
    ]]
}

fn view_section_newsletter() -> Node<Msg> {
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
