use dioxus::prelude::*;
use dioxus_components::*;

#[component]
pub fn ButtonDemo() -> Element {
    rsx! {
        div {
            class: "space-y-8",

            h1 { class: "text-4xl font-bold tracking-tight", "Button" }
            p { class: "text-lg text-muted-foreground",
                "Versatile buttons with 6 variants and 6 sizes."
            }

            // Variants
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Variants" }
                div {
                    class: "flex flex-wrap gap-4 p-6 border rounded-lg",
                    Button { variant: ButtonVariant::Default, "Default" }
                    Button { variant: ButtonVariant::Destructive, "Destructive" }
                    Button { variant: ButtonVariant::Outline, "Outline" }
                    Button { variant: ButtonVariant::Secondary, "Secondary" }
                    Button { variant: ButtonVariant::Ghost, "Ghost" }
                    Button { variant: ButtonVariant::Link, "Link" }
                }
            }

            // Sizes
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Sizes" }
                div {
                    class: "flex flex-wrap gap-4 items-center p-6 border rounded-lg",
                    Button { size: ButtonSize::Sm, "Small" }
                    Button { size: ButtonSize::Default, "Default" }
                    Button { size: ButtonSize::Lg, "Large" }
                }
            }

            // Icon Buttons
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Icon Buttons" }
                div {
                    class: "flex gap-4 items-center p-6 border rounded-lg",
                    Button {
                        size: ButtonSize::IconSm,
                        "+"
                    }
                    Button {
                        size: ButtonSize::Icon,
                        "×"
                    }
                    Button {
                        size: ButtonSize::IconLg,
                        "✓"
                    }
                }
            }
        }
    }
}
