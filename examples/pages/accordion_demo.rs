use dioxus::prelude::*;
use dioxus_components::*;

#[component]
pub fn AccordionDemo() -> Element {
    rsx! {
        div {
            class: "space-y-8",

            h1 {
                class: "text-4xl font-bold tracking-tight",
                "Accordion"
            }

            p {
                class: "text-lg text-muted-foreground",
                "A vertically stacked set of interactive headings that each reveal an associated section of content."
            }

            // Single Accordion
            div {
                class: "space-y-4",
                h2 {
                    class: "text-2xl font-semibold",
                    "Single Accordion (Collapsible)"
                }
                p {
                    class: "text-sm text-muted-foreground",
                    "Only one item can be open at a time. Clicking an open item will close it."
                }

                Accordion {
                    accordion_type: AccordionType::Single { collapsible: true },
                    class: "w-full max-w-2xl",

                    AccordionItem {
                        value: "item-1",
                        AccordionTrigger { "Is it accessible?" }
                        AccordionContent {
                            "Yes. It adheres to the WAI-ARIA design pattern. Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                        }
                    }

                    AccordionItem {
                        value: "item-2",
                        AccordionTrigger { "Is it styled?" }
                        AccordionContent {
                            "Yes. It comes with default styles that match the other components aesthetic."
                        }
                    }

                    AccordionItem {
                        value: "item-3",
                        AccordionTrigger { "Is it animated?" }
                        AccordionContent {
                            "Yes. It's animated by default with smooth slide transitions."
                        }
                    }
                }
            }

            // Multiple Accordion
            div {
                class: "space-y-4",
                h2 {
                    class: "text-2xl font-semibold",
                    "Multiple Accordion"
                }
                p {
                    class: "text-sm text-muted-foreground",
                    "Multiple items can be open at the same time."
                }

                Accordion {
                    accordion_type: AccordionType::Multiple,
                    class: "w-full max-w-2xl",

                    AccordionItem {
                        value: "feature-1",
                        AccordionTrigger { "Feature 1: Multiple Open Items" }
                        AccordionContent {
                            "This accordion allows multiple items to be open at the same time."
                        }
                    }

                    AccordionItem {
                        value: "feature-2",
                        AccordionTrigger { "Feature 2: Smooth Animations" }
                        AccordionContent {
                            "Try opening multiple sections at once! Each section animates independently."
                        }
                    }

                    AccordionItem {
                        value: "feature-3",
                        AccordionTrigger { "Feature 3: Flexible Content" }
                        AccordionContent {
                            "All sections can be open simultaneously or all closed. The choice is yours!"
                        }
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
                        "    Accordion {{\n"
                        "        accordion_type: AccordionType::Single {{ collapsible: true }},\n"
                        "        AccordionItem {{\n"
                        "            value: \"item-1\",\n"
                        "            AccordionTrigger {{ \"Is it accessible?\" }}\n"
                        "            AccordionContent {{\n"
                        "                \"Yes. It adheres to the WAI-ARIA design pattern.\"\n"
                        "            }}\n"
                        "        }}\n"
                        "    }}\n"
                        "}}"
                    }
                }
            }
        }
    }
}
