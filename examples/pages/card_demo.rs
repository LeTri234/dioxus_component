use dioxus::prelude::*;
use dioxus_components::*;

#[component]
pub fn CardDemo() -> Element {
    rsx! {
        div {
            class: "space-y-8",

            h1 { class: "text-4xl font-bold tracking-tight", "Card" }
            p { class: "text-lg text-muted-foreground",
                "Flexible card container with header, content, and footer sections."
            }

            div {
                class: "grid gap-6 md:grid-cols-2",

                // Basic Card
                Card {
                    class: "w-full",
                    CardHeader {
                        CardTitle { "Notifications" }
                        CardDescription { "You have 3 unread messages." }
                    }
                    CardContent {
                        p { "Check your inbox for updates." }
                    }
                    CardFooter {
                        Button {
                            variant: ButtonVariant::Default,
                            class: "w-full",
                            "Mark all as read"
                        }
                    }
                }

                // Card with Action
                Card {
                    class: "w-full",
                    CardHeader {
                        CardTitle { "Team Members" }
                        CardDescription { "Invite and manage your team." }
                        CardAction {
                            Button {
                                variant: ButtonVariant::Ghost,
                                class: "h-8 w-8 p-0",
                                "+"
                            }
                        }
                    }
                    CardContent {
                        div {
                            class: "text-sm",
                            "3 members in your team"
                        }
                    }
                }

                // Stats Card
                Card {
                    class: "w-full",
                    CardHeader {
                        CardTitle { "Total Revenue" }
                        CardDescription { "January - December 2024" }
                    }
                    CardContent {
                        div {
                            class: "space-y-2",
                            p { class: "text-3xl font-bold", "$45,231.89" }
                            p { class: "text-sm text-muted-foreground",
                                "+20.1% from last year"
                            }
                        }
                    }
                }

                // Form Card
                Card {
                    class: "w-full",
                    CardHeader {
                        CardTitle { "Create Account" }
                        CardDescription { "Enter your details to get started." }
                    }
                    CardContent {
                        div {
                            class: "space-y-4",
                            div {
                                class: "space-y-2",
                                label { class: "text-sm font-medium", "Email" }
                                input {
                                    class: "flex h-10 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm",
                                    r#type: "email",
                                    placeholder: "m@example.com"
                                }
                            }
                        }
                    }
                    CardFooter {
                        class: "border-t",
                        Button {
                            variant: ButtonVariant::Default,
                            class: "w-full",
                            "Create Account"
                        }
                    }
                }
            }
        }
    }
}
