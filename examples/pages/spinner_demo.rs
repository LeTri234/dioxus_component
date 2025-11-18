use dioxus::prelude::*;
use dioxus_components::*;

#[component]
pub fn SpinnerDemo() -> Element {
    rsx! {
        div {
            class: "space-y-8",

            h1 {
                class: "text-4xl font-bold tracking-tight",
                "Spinner"
            }

            p {
                class: "text-lg text-muted-foreground",
                "Loading indicators with multiple sizes and colors."
            }

            // Sizes
            div {
                class: "space-y-4",
                h2 {
                    class: "text-2xl font-semibold",
                    "Sizes"
                }
                div {
                    class: "flex gap-8 items-end p-6 border rounded-lg",

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner { size: SpinnerSize::Small }
                        span { class: "text-sm", "Small" }
                    }

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner { size: SpinnerSize::Medium }
                        span { class: "text-sm", "Medium" }
                    }

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner { size: SpinnerSize::Large }
                        span { class: "text-sm", "Large" }
                    }

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner { size: SpinnerSize::XLarge }
                        span { class: "text-sm", "XLarge" }
                    }
                }
            }

            // Colors
            div {
                class: "space-y-4",
                h2 {
                    class: "text-2xl font-semibold",
                    "Colors"
                }
                div {
                    class: "flex gap-8 items-center p-6 border rounded-lg",

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner {
                            size: SpinnerSize::Large,
                            color: "text-primary"
                        }
                        span { class: "text-sm", "Primary" }
                    }

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner {
                            size: SpinnerSize::Large,
                            color: "text-destructive"
                        }
                        span { class: "text-sm", "Destructive" }
                    }

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner {
                            size: SpinnerSize::Large,
                            color: "text-secondary-foreground"
                        }
                        span { class: "text-sm", "Secondary" }
                    }
                }
            }

            // In Buttons
            div {
                class: "space-y-4",
                h2 {
                    class: "text-2xl font-semibold",
                    "In Buttons"
                }
                div {
                    class: "flex gap-4 p-6 border rounded-lg",

                    Button {
                        variant: ButtonVariant::Default,
                        class: "gap-2",
                        Spinner {
                            size: SpinnerSize::Small,
                            color: "text-primary-foreground"
                        }
                        "Loading..."
                    }

                    Button {
                        variant: ButtonVariant::Outline,
                        class: "gap-2",
                        Spinner {
                            size: SpinnerSize::Small,
                        }
                        "Processing"
                    }
                }
            }

            // Code Example
            div {
                class: "space-y-4",
                h2 {
                    class: "text-2xl font-semibold",
                    "Code Example"
                }
                pre {
                    class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                    code {
                        class: "text-sm font-mono",
                        "use dioxus::prelude::*;\n"
                        "use dioxus_components::*;\n\n"
                        "rsx! {{\n"
                        "    Spinner {{\n"
                        "        size: SpinnerSize::Large,\n"
                        "        color: \"text-primary\"\n"
                        "    }}\n"
                        "}}"
                    }
                }
            }
        }
    }
}
