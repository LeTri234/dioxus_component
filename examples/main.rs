use dioxus::prelude::*;

mod pages;
use pages::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        #[route("/")]
        Landing {},
        #[route("/installation")]
        Installation {},
        #[route("/components/accordion")]
        AccordionDemo {},
        #[route("/components/avatar")]
        AvatarDemo {},
        #[route("/components/badge")]
        BadgeDemo {},
        #[route("/components/button")]
        ButtonDemo {},
        #[route("/components/card")]
        CardDemo {},
        #[route("/components/checkbox")]
        CheckboxDemo {},
        #[route("/components/dialog")]
        DialogDemo {},
        #[route("/components/empty")]
        EmptyDemo {},
        #[route("/components/portal")]
        PortalDemo {},
        #[route("/components/spinner")]
        SpinnerDemo {},
        #[route("/components/tooltip")]
        TooltipDemo {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const TAILWIND_CSS: &str = include_str!("../assets/tailwind.css");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        style { {TAILWIND_CSS} }
        div {
            class: "dark min-h-screen bg-background text-foreground antialiased",
            Router::<Route> {}
        }
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div {
            class: "min-h-screen flex flex-col",

            // Header
            header {
                class: "sticky top-0 z-50 border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60",
                div {
                    class: "container flex h-16 items-center px-4 mx-auto max-w-7xl",

                    // Logo
                    Link {
                        to: Route::Landing {},
                        class: "flex items-center gap-2 font-bold text-xl",
                        "🦀 Dioxus Components"
                    }

                    // Spacer
                    div { class: "flex-1" }

                    // Navigation
                    nav {
                        class: "flex items-center gap-6 text-sm",
                        Link {
                            to: Route::Installation {},
                            class: "text-muted-foreground hover:text-foreground transition-colors",
                            "Installation"
                        }
                        a {
                            href: "https://github.com/LeTri234/dioxus_component",
                            target: "_blank",
                            class: "text-muted-foreground hover:text-foreground transition-colors",
                            "GitHub"
                        }
                    }
                }
            }

            // Main Content
            div {
                class: "flex-1 flex",

                // Sidebar
                aside {
                    class: "hidden md:flex w-64 border-r",
                    div {
                        class: "sticky top-16 h-[calc(100vh-4rem)] overflow-auto py-6 px-4",
                        nav {
                            class: "space-y-6",

                            // Getting Started
                            div {
                                h4 { class: "font-semibold text-sm mb-2", "Getting Started" }
                                div {
                                    class: "space-y-1",
                                    Link {
                                        to: Route::Landing {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Overview"
                                    }
                                    Link {
                                        to: Route::Installation {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Installation"
                                    }
                                }
                            }

                            // Components
                            div {
                                h4 { class: "font-semibold text-sm mb-2", "Components" }
                                div {
                                    class: "space-y-1",
                                    Link {
                                        to: Route::AccordionDemo {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Accordion"
                                    }
                                    Link {
                                        to: Route::AvatarDemo {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Avatar"
                                    }
                                    Link {
                                        to: Route::BadgeDemo {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Badge"
                                    }
                                    Link {
                                        to: Route::ButtonDemo {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Button"
                                    }
                                    Link {
                                        to: Route::CardDemo {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Card"
                                    }
                                    Link {
                                        to: Route::CheckboxDemo {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Checkbox"
                                    }
                                    Link {
                                        to: Route::DialogDemo {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Dialog"
                                    }
                                    Link {
                                        to: Route::EmptyDemo {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Empty"
                                    }
                                    Link {
                                        to: Route::PortalDemo {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Portal"
                                    }
                                    Link {
                                        to: Route::SpinnerDemo {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Spinner"
                                    }
                                    Link {
                                        to: Route::TooltipDemo {},
                                        class: "block px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                                        "Tooltip"
                                    }
                                }
                            }
                        }
                    }
                }

                // Main Content Area
                main {
                    class: "flex-1 overflow-auto",
                    div {
                        class: "container mx-auto max-w-5xl p-6 md:p-8",
                        Outlet::<Route> {}
                    }
                }
            }

            // Footer
            footer {
                class: "border-t py-6 md:py-0",
                div {
                    class: "container flex flex-col items-center justify-between gap-4 md:h-16 md:flex-row px-4 mx-auto max-w-7xl",
                    p {
                        class: "text-center text-sm leading-loose text-muted-foreground md:text-left",
                        "Built with "
                        a {
                            href: "https://dioxuslabs.com",
                            target: "_blank",
                            class: "font-medium underline underline-offset-4",
                            "Dioxus"
                        }
                        ". Styled with "
                        a {
                            href: "https://tailwindcss.com",
                            target: "_blank",
                            class: "font-medium underline underline-offset-4",
                            "Tailwind CSS"
                        }
                        "."
                    }
                }
            }
        }
    }
}
