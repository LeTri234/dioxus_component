use dioxus::prelude::*;

mod components;
mod utils;

use components::{Avatar, AvatarFallback, AvatarImage, Button, ButtonVariant};

use crate::components::{Tooltip, TooltipContent, TooltipProvider, TooltipTrigger};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }

        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center gap-8 p-8",

            div {
                class: "text-center space-y-4",
                TooltipProvider {
                    Tooltip {
                        TooltipTrigger {
                            Button {
                                variant: ButtonVariant::Outline,
                                "Hover me for tooltip"
                            }
                        }
                        TooltipContent {
                            side: components::TooltipSide::Bottom,
                            "This is a tooltip!"
                        }
                    }
                }
            }

            // Avatar Examples
            div {
                class: "flex gap-4 items-center",
                h2 { class: "text-2xl font-bold", "Avatar Examples:" }
            }

            // Avatar with working image
            div {
                class: "flex gap-4",
                Avatar {
                    AvatarImage {
                        src: "https://github.com/shadcn.png",
                        alt: "User Avatar"
                    }
                    AvatarFallback {
                        "CN"
                    }
                }

                // Another avatar
                Avatar {
                    AvatarFallback {
                        "AB"
                    }
                }
            }

            // Button Example
            div {
                class: "flex gap-4 mt-8",
                Button {
                    variant: ButtonVariant::Default,
                    "Click me"
                }
                Button {
                    variant: ButtonVariant::Destructive,
                    "Delete"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    "Outline"
                }
            }
        }

        Hero {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
