use dioxus::prelude::*;
use dioxus_components::*;

#[component]
pub fn BadgeDemo() -> Element {
    rsx! {
        div {
            class: "space-y-8",

            h1 { class: "text-4xl font-bold tracking-tight", "Badge" }
            p { class: "text-lg text-muted-foreground",
                "Flexible badges with 4 style variants."
            }

            // Variants
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Variants" }
                div {
                    class: "flex flex-wrap gap-4 p-6 border rounded-lg items-center",
                    Badge { "Default" }
                    Badge { variant: BadgeVariant::Secondary, "Secondary" }
                    Badge { variant: BadgeVariant::Destructive, "Destructive" }
                    Badge { variant: BadgeVariant::Outline, "Outline" }
                }
            }

            // In Context
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "In Context" }
                div {
                    class: "space-y-4 p-6 border rounded-lg",
                    div {
                        class: "flex items-center gap-2",
                        span { "Status:" }
                        Badge { variant: BadgeVariant::Default, "Active" }
                    }
                    div {
                        class: "flex items-center gap-2",
                        span { "Priority:" }
                        Badge { variant: BadgeVariant::Destructive, "High" }
                    }
                    div {
                        class: "flex items-center gap-2",
                        span { "Version:" }
                        Badge { variant: BadgeVariant::Outline, "v0.1.1" }
                    }
                }
            }
        }
    }
}
