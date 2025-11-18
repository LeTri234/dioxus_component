use dioxus::prelude::*;

#[component]
pub fn Landing() -> Element {
    rsx! {
        div {
            class: "space-y-12",

            // Hero Section
            div {
                class: "text-center space-y-4 py-12",
                h1 {
                    class: "text-4xl md:text-6xl font-bold tracking-tight",
                    "Dioxus Components"
                }
                p {
                    class: "text-xl text-muted-foreground max-w-2xl mx-auto",
                    "A comprehensive collection of 11 production-ready components for Dioxus 0.7"
                }
                div {
                    class: "flex gap-4 justify-center mt-8",
                    Link {
                        to: crate::Route::Installation {},
                        class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 bg-primary text-primary-foreground hover:bg-primary/90 h-11 px-8",
                        "Get Started"
                    }
                    a {
                        href: "https://github.com/LeTri234/dioxus_component",
                        target: "_blank",
                        class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 border border-input hover:bg-accent hover:text-accent-foreground h-11 px-8",
                        "View on GitHub"
                    }
                }
            }

            // Features
            div {
                class: "grid gap-6 md:grid-cols-3",

                div {
                    class: "p-6 border rounded-lg bg-card",
                    h3 { class: "text-lg font-semibold mb-2", "🎨 Tailwind CSS v4" }
                    p { class: "text-sm text-muted-foreground",
                        "Modern utility-first styling with full dark mode support"
                    }
                }

                div {
                    class: "p-6 border rounded-lg bg-card",
                    h3 { class: "text-lg font-semibold mb-2", "♿ Accessible" }
                    p { class: "text-sm text-muted-foreground",
                        "WAI-ARIA compliant with full keyboard navigation"
                    }
                }

                div {
                    class: "p-6 border rounded-lg bg-card",
                    h3 { class: "text-lg font-semibold mb-2", "🦀 Pure Rust" }
                    p { class: "text-sm text-muted-foreground",
                        "No JavaScript required - 100% Rust/WASM"
                    }
                }
            }

            // Components Overview
            div {
                class: "space-y-6",
                h2 {
                    class: "text-3xl font-bold tracking-tight",
                    "11 Production-Ready Components"
                }

                div {
                    class: "grid gap-4 md:grid-cols-2 lg:grid-cols-3",

                    ComponentCard {
                        name: "Accordion",
                        description: "Collapsible sections with single/multiple modes",
                        route: crate::Route::AccordionDemo {}
                    }
                    ComponentCard {
                        name: "Avatar",
                        description: "User avatars with image loading and fallback",
                        route: crate::Route::AvatarDemo {}
                    }
                    ComponentCard {
                        name: "Badge",
                        description: "Flexible badges with 4 style variants",
                        route: crate::Route::BadgeDemo {}
                    }
                    ComponentCard {
                        name: "Button",
                        description: "Versatile buttons with 6 variants and 6 sizes",
                        route: crate::Route::ButtonDemo {}
                    }
                    ComponentCard {
                        name: "Card",
                        description: "Flexible card container with header and footer",
                        route: crate::Route::CardDemo {}
                    }
                    ComponentCard {
                        name: "Checkbox",
                        description: "Three-state checkbox (checked/unchecked/indeterminate)",
                        route: crate::Route::CheckboxDemo {}
                    }
                    ComponentCard {
                        name: "Dialog",
                        description: "Accessible modal dialogs with overlay",
                        route: crate::Route::DialogDemo {}
                    }
                    ComponentCard {
                        name: "Empty",
                        description: "Empty state component for 'no content' scenarios",
                        route: crate::Route::EmptyDemo {}
                    }
                    ComponentCard {
                        name: "Portal",
                        description: "Render content outside parent DOM hierarchy",
                        route: crate::Route::PortalDemo {}
                    }
                    ComponentCard {
                        name: "Spinner",
                        description: "Loading indicators with multiple sizes",
                        route: crate::Route::SpinnerDemo {}
                    }
                    ComponentCard {
                        name: "Tooltip",
                        description: "Hover-triggered tooltips with flexible positioning",
                        route: crate::Route::TooltipDemo {}
                    }
                }
            }

            // Quick Start
            div {
                class: "p-8 border rounded-lg bg-muted/50 space-y-4",
                h2 {
                    class: "text-2xl font-bold",
                    "Quick Start"
                }
                p {
                    class: "text-muted-foreground",
                    "Install via Cargo and configure Tailwind CSS:"
                }
                pre {
                    class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                    code {
                        class: "text-sm font-mono",
                        "cargo add dioxus_components"
                    }
                }
                Link {
                    to: crate::Route::Installation {},
                    class: "inline-flex items-center text-sm font-medium text-primary hover:underline",
                    "View full installation guide →"
                }
            }
        }
    }
}

#[component]
fn ComponentCard(name: String, description: String, route: crate::Route) -> Element {
    rsx! {
        Link {
            to: route,
            class: "block p-6 border rounded-lg bg-card hover:bg-accent transition-colors",
            h3 {
                class: "text-lg font-semibold mb-2",
                "{name}"
            }
            p {
                class: "text-sm text-muted-foreground",
                "{description}"
            }
        }
    }
}
