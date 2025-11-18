use dioxus::prelude::*;
use dioxus_components::*;

#[component]
pub fn AvatarDemo() -> Element {
    rsx! {
        div {
            class: "space-y-8",

            h1 { class: "text-4xl font-bold tracking-tight", "Avatar" }
            p { class: "text-lg text-muted-foreground",
                "User avatars with image loading and fallback support."
            }

            // Basic Avatars
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Basic Avatars" }
                div {
                    class: "flex gap-4 p-6 border rounded-lg",
                    Avatar {
                        AvatarImage {
                            src: "https://github.com/shadcn.png",
                            alt: "User Avatar"
                        }
                        AvatarFallback { "CN" }
                    }
                    Avatar {
                        AvatarFallback { "AB" }
                    }
                    Avatar {
                        AvatarFallback { "XY" }
                    }
                }
            }

            // In Profile Card
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "In Profile Card" }
                div {
                    class: "p-6 border rounded-lg bg-card max-w-sm",
                    div {
                        class: "flex items-center gap-4",
                        Avatar {
                            class: "h-16 w-16",
                            AvatarImage {
                                src: "https://github.com/shadcn.png",
                                alt: "User"
                            }
                            AvatarFallback { "CN" }
                        }
                        div {
                            p { class: "font-semibold", "John Doe" }
                            p { class: "text-sm text-muted-foreground", "john@example.com" }
                        }
                    }
                }
            }
        }
    }
}
