use dioxus::prelude::*;
use dioxus_components::*;

#[component]
pub fn PortalDemo() -> Element {
    let mut show_portal = use_signal(|| false);

    rsx! {
        div {
            class: "space-y-8",

            h1 { class: "text-4xl font-bold tracking-tight", "Portal" }
            p { class: "text-lg text-muted-foreground",
                "Render content outside the parent DOM hierarchy."
            }

            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Basic Portal" }
                p { class: "text-sm text-muted-foreground",
                    "Portal renders content with fixed positioning, appearing above other content."
                }

                div {
                    class: "p-6 border rounded-lg",
                    Button {
                        variant: ButtonVariant::Default,
                        onclick: move |_| show_portal.set(!show_portal()),
                        if show_portal() {
                            "Hide Portal"
                        } else {
                            "Show Portal"
                        }
                    }

                    if show_portal() {
                        Portal {
                            container: "body",
                            div {
                                class: "fixed inset-0 bg-black/50 flex items-center justify-center z-50",
                                onclick: move |_| show_portal.set(false),
                                div {
                                    class: "bg-card p-8 rounded-lg shadow-lg max-w-md border",
                                    onclick: move |e| e.stop_propagation(),
                                    h3 { class: "text-xl font-bold mb-4", "Portal Content" }
                                    p { class: "mb-4 text-muted-foreground",
                                        "This content is rendered using a Portal component. It appears with fixed positioning above other content."
                                    }
                                    Button {
                                        variant: ButtonVariant::Destructive,
                                        onclick: move |_| show_portal.set(false),
                                        "Close"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Code Example
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Code Example" }
                pre {
                    class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                    code {
                        class: "text-sm font-mono",
                        "Portal {{\n"
                        "    container: \"body\",\n"
                        "    div {{\n"
                        "        class: \"fixed inset-0 bg-black/50\",\n"
                        "        // Your content here\n"
                        "    }}\n"
                        "}}"
                    }
                }
            }
        }
    }
}
