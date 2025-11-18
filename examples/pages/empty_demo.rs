use dioxus::prelude::*;
use dioxus_components::*;

#[component]
pub fn EmptyDemo() -> Element {
    rsx! {
        div {
            class: "space-y-8",

            h1 { class: "text-4xl font-bold tracking-tight", "Empty" }
            p { class: "text-lg text-muted-foreground",
                "Empty state component for 'no content' scenarios."
            }

            div {
                class: "grid gap-6 md:grid-cols-2",

                // No Files
                Empty {
                    class: "min-h-[300px] border",
                    EmptyHeader {
                        EmptyMedia {
                            variant: EmptyMediaVariant::Icon,
                            svg {
                                class: "size-6",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M3.75 9.776c.112-.017.227-.026.344-.026h15.812c.117 0 .232.009.344.026m-16.5 0a2.25 2.25 0 0 0-1.883 2.542l.857 6a2.25 2.25 0 0 0 2.227 1.932H19.05a2.25 2.25 0 0 0 2.227-1.932l.857-6a2.25 2.25 0 0 0-1.883-2.542m-16.5 0V6A2.25 2.25 0 0 1 6 3.75h3.879a1.5 1.5 0 0 1 1.06.44l2.122 2.12a1.5 1.5 0 0 0 1.06.44H18A2.25 2.25 0 0 1 20.25 9v.776"
                                }
                            }
                        }
                        EmptyTitle { "No files found" }
                        EmptyDescription { "You haven't uploaded any files yet." }
                    }
                    EmptyContent {
                        Button { variant: ButtonVariant::Default, "Upload file" }
                    }
                }

                // No Search Results
                Empty {
                    class: "min-h-[300px] border",
                    EmptyHeader {
                        EmptyMedia {
                            variant: EmptyMediaVariant::Icon,
                            svg {
                                class: "size-6",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"
                                }
                            }
                        }
                        EmptyTitle { "No results found" }
                        EmptyDescription {
                            "We couldn't find what you're looking for. Try adjusting your search."
                        }
                    }
                    EmptyContent {
                        Button { variant: ButtonVariant::Outline, "Clear search" }
                    }
                }

                // Inbox Zero
                Empty {
                    class: "min-h-[300px] border",
                    EmptyHeader {
                        EmptyMedia {
                            variant: EmptyMediaVariant::Icon,
                            svg {
                                class: "size-6",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75"
                                }
                            }
                        }
                        EmptyTitle { "Inbox zero!" }
                        EmptyDescription { "You're all caught up. Great work!" }
                    }
                }

                // No Data
                Empty {
                    class: "min-h-[300px] border",
                    EmptyHeader {
                        EmptyMedia {
                            variant: EmptyMediaVariant::Icon,
                            svg {
                                class: "size-6",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 0 1 3 19.875v-6.75ZM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 0 1-1.125-1.125V8.625ZM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 0 1-1.125-1.125V4.125Z"
                                }
                            }
                        }
                        EmptyTitle { "No data available" }
                        EmptyDescription {
                            "Start collecting data to see insights and analytics here."
                        }
                    }
                    EmptyContent {
                        div {
                            class: "flex gap-2",
                            Button { variant: ButtonVariant::Default, "Get started" }
                            Button { variant: ButtonVariant::Outline, "Learn more" }
                        }
                    }
                }
            }
        }
    }
}
