use dioxus::prelude::*;
use dioxus_components::*;

#[component]
pub fn TooltipDemo() -> Element {
    rsx! {
        div {
            class: "space-y-8",

            h1 { class: "text-4xl font-bold tracking-tight", "Tooltip" }
            p { class: "text-lg text-muted-foreground",
                "Displays additional information when hovering over or focusing on an element."
            }

            // Basic Tooltip
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Basic Tooltip" }
                div {
                    class: "p-6 border rounded-lg flex justify-center",
                    TooltipProvider {
                        Tooltip {
                            TooltipTrigger {
                                Button {
                                    variant: ButtonVariant::Outline,
                                    "Hover me"
                                }
                            }
                            TooltipContent {
                                side: TooltipSide::Top,
                                "This is a tooltip!"
                            }
                        }
                    }
                }
            }

            // Different Positions
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Different Positions" }
                div {
                    class: "p-12 border rounded-lg flex gap-8 justify-center items-center",

                    TooltipProvider {
                        Tooltip {
                            TooltipTrigger {
                                Button {
                                    variant: ButtonVariant::Outline,
                                    "Top"
                                }
                            }
                            TooltipContent {
                                side: TooltipSide::Top,
                                "Tooltip on top"
                            }
                        }
                    }

                    TooltipProvider {
                        Tooltip {
                            TooltipTrigger {
                                Button {
                                    variant: ButtonVariant::Outline,
                                    "Right"
                                }
                            }
                            TooltipContent {
                                side: TooltipSide::Right,
                                "Tooltip on right"
                            }
                        }
                    }

                    TooltipProvider {
                        Tooltip {
                            TooltipTrigger {
                                Button {
                                    variant: ButtonVariant::Outline,
                                    "Bottom"
                                }
                            }
                            TooltipContent {
                                side: TooltipSide::Bottom,
                                "Tooltip on bottom"
                            }
                        }
                    }

                    TooltipProvider {
                        Tooltip {
                            TooltipTrigger {
                                Button {
                                    variant: ButtonVariant::Outline,
                                    "Left"
                                }
                            }
                            TooltipContent {
                                side: TooltipSide::Left,
                                "Tooltip on left"
                            }
                        }
                    }
                }
            }
        }
    }
}
