use dioxus::prelude::*;

mod components;
mod utils;

use components::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionType, Avatar,
    AvatarFallback, AvatarImage, Badge, BadgeVariant, Button, ButtonVariant, Spinner, SpinnerSize,
    Tooltip, TooltipContent, TooltipProvider, TooltipTrigger,
};

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
                h2 { class: "text-2xl font-bold", "Tooltip Example:" }
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

            // Spinner Examples
            div {
                class: "text-center space-y-4",
                h2 { class: "text-2xl font-bold", "Spinner Examples:" }
                div {
                    class: "flex gap-4 items-center justify-center",

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner {
                            size: SpinnerSize::Small,
                        }
                        span { class: "text-sm", "Small" }
                    }

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner {
                            size: SpinnerSize::Medium,
                            color: "text-primary"
                        }
                        span { class: "text-sm", "Medium" }
                    }

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner {
                            size: SpinnerSize::Large,
                            color: "text-destructive"
                        }
                        span { class: "text-sm", "Large" }
                    }

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner {
                            size: SpinnerSize::XLarge,
                            color: "text-secondary-foreground"
                        }
                        span { class: "text-sm", "XLarge" }
                    }
                }

                // Spinner in button
                div {
                    class: "flex gap-4 justify-center mt-4",
                    Button {
                        variant: ButtonVariant::Default,
                        class: "gap-2",
                        Spinner {
                            size: SpinnerSize::Small,
                            color: "text-primary-foreground"
                        }
                        "Loading..."
                    }
                }
            }

            // Accordion Example
            div {
                class: "w-full max-w-2xl space-y-4",
                h2 { class: "text-2xl font-bold text-center", "Accordion Example:" }

                Accordion {
                    accordion_type: AccordionType::Single { collapsible: true },
                    class: "w-full",

                    AccordionItem {
                        value: "item-1",
                        AccordionTrigger { "Is it accessible?" }
                        AccordionContent {
                            "Yes. It adheres to the WAI-ARIA design pattern. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                        }
                    }

                    AccordionItem {
                        value: "item-2",
                        AccordionTrigger { "Is it styled?" }
                        AccordionContent {
                            "Yes. It comes with default styles that matches the other components aesthetic."
                        }
                    }

                    AccordionItem {
                        value: "item-3",
                        AccordionTrigger { "Is it animated?" }
                        AccordionContent {
                            "Yes. It's animated by default, but you can disable it if you prefer."
                        }
                    }
                }

                h3 { class: "text-xl font-bold text-center mt-8", "Multiple Accordion:" }

                Accordion {
                    accordion_type: AccordionType::Multiple,
                    class: "w-full",

                    AccordionItem {
                        value: "feature-1",
                        AccordionTrigger { "Feature 1" }
                        AccordionContent {
                            "This accordion allows multiple items to be open at the same time."
                        }
                    }

                    AccordionItem {
                        value: "feature-2",
                        AccordionTrigger { "Feature 2" }
                        AccordionContent {
                            "Try opening multiple sections at once!"
                        }
                    }

                    AccordionItem {
                        value: "feature-3",
                        AccordionTrigger { "Feature 3" }
                        AccordionContent {
                            "All sections can be open simultaneously or all closed."
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

            // Badge Example
            div {
                class: "flex gap-4 flex-wrap mt-8",
                h2 { class: "w-full text-2xl font-bold", "Badge Examples:" }
                Badge {
                    "Default"
                }
                Badge {
                    variant: BadgeVariant::Secondary,
                    "Secondary"
                }
                Badge {
                    variant: BadgeVariant::Destructive,
                    "Destructive"
                }
                Badge {
                    variant: BadgeVariant::Outline,
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
