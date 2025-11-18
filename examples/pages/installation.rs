use dioxus::prelude::*;

#[component]
pub fn Installation() -> Element {
    rsx! {
        div {
            class: "space-y-8 max-w-4xl",

            h1 {
                class: "text-4xl font-bold tracking-tight",
                "Installation Guide"
            }

            p {
                class: "text-lg text-muted-foreground",
                "Follow these steps to install and configure Dioxus Components in your project."
            }

            // Step 1: Add Dependency
            div {
                class: "space-y-4",
                h2 {
                    class: "text-2xl font-semibold",
                    "1. Add Rust Dependency"
                }
                p {
                    class: "text-muted-foreground",
                    "Add ", code { class: "bg-muted px-1 py-0.5 rounded text-sm", "dioxus_components" }, " to your ", code { class: "bg-muted px-1 py-0.5 rounded text-sm", "Cargo.toml" }, ":"
                }
                pre {
                    class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                    code {
                        class: "text-sm font-mono",
                        "[dependencies]\n"
                        "dioxus_components = \"0.1.1\"\n"
                        "dioxus = {{ version = \"0.7.1\", features = [\"web\"] }}"
                    }
                }
                p {
                    class: "text-sm text-muted-foreground",
                    "Or add with cargo:"
                }
                pre {
                    class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                    code {
                        class: "text-sm font-mono",
                        "cargo add dioxus_components"
                    }
                }
            }

            // Step 2: Configure Tailwind
            div {
                class: "space-y-4",
                h2 {
                    class: "text-2xl font-semibold",
                    "2. Configure Tailwind CSS"
                }
                div {
                    class: "bg-yellow-50 dark:bg-yellow-950 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4",
                    p {
                        class: "text-sm font-semibold mb-2",
                        "⚠️ Important"
                    }
                    p {
                        class: "text-sm",
                        "Tailwind must be configured to scan the library source code. Choose one of the methods below:"
                    }
                }

                // Option A: Safelist
                div {
                    class: "space-y-3",
                    h3 {
                        class: "text-lg font-semibold",
                        "Option A: Use Safelist (Recommended)"
                    }
                    p {
                        class: "text-sm text-muted-foreground",
                        "Download the safelist file:"
                    }
                    pre {
                        class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                        code {
                        class: "text-sm font-mono",
                            "curl -o safelist.json https://raw.githubusercontent.com/LeTri234/dioxus_component/main/safelist.json"
                        }
                    }
                    p {
                        class: "text-sm text-muted-foreground",
                        "Update your ", code { class: "bg-muted px-1 py-0.5 rounded text-sm", "tailwind.config.js" }, ":"
                    }
                    pre {
                        class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                        code {
                        class: "text-sm font-mono",
                            "module.exports = {{\n"
                            "  content: [\"./src/**/*.{{rs,html}}\"],\n"
                            "  safelist: require('./safelist.json'),\n"
                            "  theme: {{ extend: {{}} }},\n"
                            "  plugins: [],\n"
                            "}}"
                        }
                    }
                }

                // Option B: Tailwind v4
                div {
                    class: "space-y-3",
                    h3 {
                        class: "text-lg font-semibold",
                        "Option B: Tailwind v4 with @source"
                    }
                    p {
                        class: "text-sm text-muted-foreground",
                        "Update your ", code { class: "bg-muted px-1 py-0.5 rounded text-sm", "tailwind.css" }, ":"
                    }
                    pre {
                        class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                        code {
                        class: "text-sm font-mono",
                            "@import \"tailwindcss\";\n"
                            "@source \"../src\";\n"
                            "@source \"../../.cargo/registry/src/*/dioxus_components-*/src\";"
                        }
                    }
                }
            }

            // Step 3: Import Styles
            div {
                class: "space-y-4",
                h2 {
                    class: "text-2xl font-semibold",
                    "3. Import Component Styles"
                }
                p {
                    class: "text-muted-foreground",
                    "Download the animations CSS file:"
                }
                pre {
                    class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                    code {
                        class: "text-sm font-mono",
                        "curl -o src/components.css https://raw.githubusercontent.com/LeTri234/dioxus_component/main/src/components.css"
                    }
                }
                p {
                    class: "text-sm text-muted-foreground",
                    "Import in your ", code { class: "bg-muted px-1 py-0.5 rounded text-sm", "tailwind.css" }, ":"
                }
                pre {
                    class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                    code {
                        class: "text-sm font-mono",
                        "@import \"./components.css\";"
                    }
                }
            }

            // Step 4: Build CSS
            div {
                class: "space-y-4",
                h2 {
                    class: "text-2xl font-semibold",
                    "4. Build CSS"
                }
                pre {
                    class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                    code {
                        class: "text-sm font-mono",
                        "npx @tailwindcss/cli -i ./src/tailwind.css -o ./assets/tailwind.css"
                    }
                }
            }

            // Step 5: Use Components
            div {
                class: "space-y-4",
                h2 {
                    class: "text-2xl font-semibold",
                    "5. Use Components"
                }
                p {
                    class: "text-muted-foreground",
                    "Import and use components in your Dioxus app:"
                }
                pre {
                    class: "bg-muted/50 p-4 rounded-lg border overflow-x-auto",
                    code {
                        class: "text-sm font-mono",
                        "use dioxus::prelude::*;\n"
                        "use dioxus_components::*;\n\n"
                        "#[component]\n"
                        "fn App() -> Element {{\n"
                        "    rsx! {{\n"
                        "        Button {{\n"
                        "            variant: ButtonVariant::Default,\n"
                        "            \"Click me\"\n"
                        "        }}\n"
                        "    }}\n"
                        "}}"
                    }
                }
            }

            // Resources
            div {
                class: "p-6 border rounded-lg bg-muted/50 space-y-4",
                h2 {
                    class: "text-xl font-semibold",
                    "Additional Resources"
                }
                ul {
                    class: "space-y-2",
                    li {
                        a {
                            href: "https://github.com/LeTri234/dioxus_component/blob/main/QUICKSTART.md",
                            target: "_blank",
                            class: "text-sm text-primary hover:underline",
                            "⚡ Quick Start Guide - Fix missing styles in 2 minutes"
                        }
                    }
                    li {
                        a {
                            href: "https://github.com/LeTri234/dioxus_component/blob/main/TROUBLESHOOTING.md",
                            target: "_blank",
                            class: "text-sm text-primary hover:underline",
                            "🔧 Troubleshooting Guide - Common issues and solutions"
                        }
                    }
                    li {
                        a {
                            href: "https://github.com/LeTri234/dioxus_component/blob/main/EXAMPLE_PROJECT_SETUP.md",
                            target: "_blank",
                            class: "text-sm text-primary hover:underline",
                            "📦 Example Project Setup - Step-by-step new project guide"
                        }
                    }
                    li {
                        a {
                            href: "https://github.com/LeTri234/dioxus_component/blob/main/COMPONENTS.md",
                            target: "_blank",
                            class: "text-sm text-primary hover:underline",
                            "📚 Component Documentation - Complete API reference"
                        }
                    }
                }
            }
        }
    }
}
