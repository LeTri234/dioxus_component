use dioxus::prelude::*;
use dioxus_components::*;

#[component]
pub fn DialogDemo() -> Element {
    rsx! {
        div {
            class: "space-y-8",

            h1 { class: "text-4xl font-bold tracking-tight", "Dialog" }
            p { class: "text-lg text-muted-foreground",
                "Accessible modal dialogs with overlay, focus trap, and keyboard handling."
            }

            div {
                class: "space-y-6",

                // Basic Dialog
                div {
                    class: "space-y-4",
                    h2 { class: "text-2xl font-semibold", "Basic Dialog" }
                    Dialog {
                        default_open: false,
                        modal: true,
                        DialogTrigger {
                            class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 bg-primary text-primary-foreground hover:bg-primary/90 h-10 py-2 px-4",
                            "Open Dialog"
                        }
                        DialogOverlay {
                            class: "fixed inset-0 bg-black/50"
                        }
                        DialogContent {
                            class: "fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 bg-background p-6 shadow-lg rounded-lg border max-w-lg w-full",
                            DialogTitle {
                                class: "text-lg font-semibold mb-2",
                                "Dialog Title"
                            }
                            DialogDescription {
                                class: "text-sm text-muted-foreground mb-4",
                                "This is a dialog description providing context about the dialog content."
                            }
                            div {
                                class: "flex justify-end gap-2 mt-6",
                                DialogClose {
                                    class: "inline-flex items-center justify-center rounded-md text-sm font-medium border h-10 py-2 px-4",
                                    "Cancel"
                                }
                                DialogClose {
                                    class: "inline-flex items-center justify-center rounded-md text-sm font-medium bg-primary text-primary-foreground h-10 py-2 px-4",
                                    "Confirm"
                                }
                            }
                        }
                    }
                }

                // Alert Dialog
                div {
                    class: "space-y-4",
                    h2 { class: "text-2xl font-semibold", "Alert Dialog" }
                    Dialog {
                        default_open: false,
                        modal: true,
                        DialogTrigger {
                            class: "inline-flex items-center justify-center rounded-md text-sm font-medium bg-destructive text-destructive-foreground h-10 py-2 px-4",
                            "Delete Account"
                        }
                        DialogOverlay {
                            class: "fixed inset-0 bg-black/50"
                        }
                        DialogContent {
                            class: "fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 bg-background p-6 shadow-lg rounded-lg border max-w-md w-full",
                            DialogTitle {
                                class: "text-lg font-semibold mb-2 text-destructive",
                                "⚠️ Are you absolutely sure?"
                            }
                            DialogDescription {
                                class: "text-sm text-muted-foreground mb-4",
                                "This action cannot be undone. This will permanently delete your account."
                            }
                            div {
                                class: "flex justify-end gap-2 mt-6",
                                DialogClose {
                                    class: "inline-flex items-center justify-center rounded-md text-sm font-medium border h-10 py-2 px-4",
                                    "Cancel"
                                }
                                DialogClose {
                                    class: "inline-flex items-center justify-center rounded-md text-sm font-medium bg-destructive text-destructive-foreground h-10 py-2 px-4",
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
