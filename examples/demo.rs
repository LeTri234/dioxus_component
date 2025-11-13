use dioxus::prelude::*;
use dioxus_components::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionType, Avatar,
    AvatarFallback, AvatarImage, Badge, BadgeVariant, Button, ButtonVariant, Card, CardAction,
    CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Checkbox, CheckboxIndicator,
    CheckboxLabel, CheckboxProvider, CheckboxTrigger, CheckedState, Dialog, DialogClose,
    DialogContent, DialogDescription, DialogOverlay, DialogTitle, DialogTrigger, Portal, Spinner,
    SpinnerSize, Tooltip, TooltipContent, TooltipProvider, TooltipSide, TooltipTrigger,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Track if the app is hydrated/loaded
    let mut is_loaded = use_signal(|| false);

    // Set loaded after first render
    use_effect(move || {
        is_loaded.set(true);
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // Loading overlay
        if !is_loaded() {
            div {
                class: "fixed inset-0 bg-background flex items-center justify-center z-50",
                style: "min-height: 100vh;",
                div {
                    class: "text-center space-y-4",
                    div {
                        class: "inline-block h-12 w-12 animate-spin rounded-full border-4 border-solid border-primary border-r-transparent"
                    }
                    p { class: "text-lg text-muted-foreground", "Loading components..." }
                }
            }
        }

        // Main content with opacity transition
        div {
            class: if is_loaded() { "opacity-100 transition-opacity duration-300" } else { "opacity-0" },
            Router::<Route> {}
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center gap-8 p-8",

            div {
                class: "text-center space-y-4",
                h2 { class: "text-2xl font-bold", "Tooltip Example:" }
                TooltipProvider {
                    Tooltip {
                        TooltipTrigger {
                            Button {
                                variant: ButtonVariant::Outline,
                                "Hover me for tooltip"
                            }
                        }
                        TooltipContent {
                            side: TooltipSide::Bottom,
                            "This is a tooltip!"
                        }
                    }
                }
            }

            // Spinner Examples
            div {
                class: "text-center space-y-4",
                h2 { class: "text-2xl font-bold", "Spinner Examples:" }
                div {
                    class: "flex gap-4 items-center justify-center",

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner {
                            size: SpinnerSize::Small,
                        }
                        span { class: "text-sm", "Small" }
                    }

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner {
                            size: SpinnerSize::Medium,
                            color: "text-primary"
                        }
                        span { class: "text-sm", "Medium" }
                    }

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner {
                            size: SpinnerSize::Large,
                            color: "text-destructive"
                        }
                        span { class: "text-sm", "Large" }
                    }

                    div {
                        class: "flex flex-col gap-2 items-center",
                        Spinner {
                            size: SpinnerSize::XLarge,
                            color: "text-secondary-foreground"
                        }
                        span { class: "text-sm", "XLarge" }
                    }
                }

                // Spinner in button
                div {
                    class: "flex gap-4 justify-center mt-4",
                    Button {
                        variant: ButtonVariant::Default,
                        class: "gap-2",
                        Spinner {
                            size: SpinnerSize::Small,
                            color: "text-primary-foreground"
                        }
                        "Loading..."
                    }
                }
            }

            // Portal Examples
            div {
                class: "w-full max-w-2xl space-y-4",
                h2 { class: "text-2xl font-bold text-center", "Portal Examples:" }

                div {
                    class: "space-y-4 p-6 border rounded-lg bg-card",
                    p { class: "text-sm text-muted-foreground mb-4",
                        "Portal renders content with fixed positioning, appearing above other content."
                    }

                    PortalExample {}
                }
            }

            // Dialog Examples
            div {
                class: "w-full max-w-2xl space-y-4",
                h2 { class: "text-2xl font-bold text-center", "Dialog Examples:" }

                div {
                    class: "space-y-4 p-6 border rounded-lg bg-card",
                    p { class: "text-sm text-muted-foreground mb-4",
                        "Dialog (Modal) - Accessible dialog components with overlay, focus trap, and keyboard handling."
                    }

                    DialogExample {}
                }
            }

            // Card Examples
            div {
                class: "w-full max-w-2xl space-y-4",
                h2 { class: "text-2xl font-bold text-center", "Card Examples:" }

                div {
                    class: "space-y-6 p-6 border rounded-lg bg-card",
                    p { class: "text-sm text-muted-foreground mb-4",
                        "Card - Flexible card container with header, content, and footer sections."
                    }

                    CardExample {}
                }
            }

            // Checkbox Examples
            div {
                class: "w-full max-w-2xl space-y-4",
                h2 { class: "text-2xl font-bold text-center", "Checkbox Examples:" }

                div {
                    class: "space-y-4 p-6 border rounded-lg bg-card",

                    // Provider pattern (Radix UI style)
                    div {
                        class: "flex items-center gap-2",
                        h3 { class: "text-sm font-semibold mb-2 w-full", "Provider Pattern:" }
                    }
                    div {
                        class: "flex items-center gap-2",
                        CheckboxProvider {
                            default_checked: CheckedState::Unchecked,
                            name: Some("provider_example".to_string()),
                            CheckboxTrigger {
                                id: Some("provider".to_string()),
                                CheckboxIndicator {}
                            }
                        }
                        CheckboxLabel {
                            for_id: Some("provider".to_string()),
                            "Provider + Trigger + Indicator"
                        }
                    }

                    // Convenience component (backward compatible)
                    div {
                        class: "flex items-center gap-2 mt-4",
                        h3 { class: "text-sm font-semibold mb-2 w-full", "Convenience Component:" }
                    }
                    div {
                        class: "flex items-center gap-2",
                        Checkbox {
                            default_checked: CheckedState::Checked,
                            id: Some("convenience".to_string()),
                            CheckboxIndicator {}
                        }
                        CheckboxLabel {
                            for_id: Some("convenience".to_string()),
                            "Pre-checked checkbox"
                        }
                    }

                    // Indeterminate state
                    div {
                        class: "flex items-center gap-2 mt-4",
                        h3 { class: "text-sm font-semibold mb-2 w-full", "Indeterminate State:" }
                    }
                    div {
                        class: "flex items-center gap-2",
                        Checkbox {
                            default_checked: CheckedState::Indeterminate,
                            id: Some("indeterminate".to_string()),
                            CheckboxIndicator {}
                        }
                        CheckboxLabel {
                            for_id: Some("indeterminate".to_string()),
                            "Mixed selection state"
                        }
                    }

                    // Disabled states
                    div {
                        class: "flex items-center gap-2 mt-4",
                        h3 { class: "text-sm font-semibold mb-2 w-full", "Disabled:" }
                    }
                    div {
                        class: "flex gap-4",
                        div {
                            class: "flex items-center gap-2",
                            Checkbox {
                                default_checked: CheckedState::Checked,
                                disabled: true,
                                id: Some("disabled_checked".to_string()),
                                CheckboxIndicator {}
                            }
                            CheckboxLabel {
                                for_id: Some("disabled_checked".to_string()),
                                class: Some("opacity-50".to_string()),
                                "Disabled checked"
                            }
                        }
                        div {
                            class: "flex items-center gap-2",
                            Checkbox {
                                default_checked: CheckedState::Unchecked,
                                disabled: true,
                                id: Some("disabled_unchecked".to_string()),
                                CheckboxIndicator {}
                            }
                            CheckboxLabel {
                                for_id: Some("disabled_unchecked".to_string()),
                                class: Some("opacity-50".to_string()),
                                "Disabled unchecked"
                            }
                        }
                    }

                    // Form integration
                    div {
                        class: "flex items-center gap-2 mt-4",
                        h3 { class: "text-sm font-semibold mb-2 w-full", "Form Integration:" }
                    }
                    form {
                        id: "demo_form",
                        class: "space-y-2",
                        onsubmit: move |evt| {
                            evt.prevent_default();
                        },

                        div {
                            class: "flex items-center gap-2",
                            Checkbox {
                                name: Some("terms".to_string()),
                                form: Some("demo_form".to_string()),
                                required: true,
                                id: Some("terms".to_string()),
                                CheckboxIndicator {}
                            }
                            CheckboxLabel {
                                for_id: Some("terms".to_string()),
                                "I accept the terms *"
                            }
                        }
                        div {
                            class: "flex items-center gap-2",
                            Checkbox {
                                name: Some("newsletter".to_string()),
                                form: Some("demo_form".to_string()),
                                value: "yes".to_string(),
                                id: Some("newsletter".to_string()),
                                CheckboxIndicator {}
                            }
                            CheckboxLabel {
                                for_id: Some("newsletter".to_string()),
                                "Subscribe to newsletter"
                            }
                        }

                        button {
                            r#type: "submit",
                            class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2 mt-2",
                            "Submit Form"
                        }
                    }
                }
            }

            // Accordion Example
            div {
                class: "w-full max-w-2xl space-y-4",
                h2 { class: "text-2xl font-bold text-center", "Accordion Example:" }

                Accordion {
                    accordion_type: AccordionType::Single { collapsible: true },
                    class: "w-full",

                    AccordionItem {
                        value: "item-1",
                        AccordionTrigger { "Is it accessible?" }
                        AccordionContent {
                            "Yes. It adheres to the WAI-ARIA design pattern. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                        }
                    }

                    AccordionItem {
                        value: "item-2",
                        AccordionTrigger { "Is it styled?" }
                        AccordionContent {
                            "Yes. It comes with default styles that matches the other components aesthetic."
                        }
                    }

                    AccordionItem {
                        value: "item-3",
                        AccordionTrigger { "Is it animated?" }
                        AccordionContent {
                            "Yes. It's animated by default, but you can disable it if you prefer."
                        }
                    }
                }

                h3 { class: "text-xl font-bold text-center mt-8", "Multiple Accordion:" }

                Accordion {
                    accordion_type: AccordionType::Multiple,
                    class: "w-full",

                    AccordionItem {
                        value: "feature-1",
                        AccordionTrigger { "Feature 1" }
                        AccordionContent {
                            "This accordion allows multiple items to be open at the same time."
                        }
                    }

                    AccordionItem {
                        value: "feature-2",
                        AccordionTrigger { "Feature 2" }
                        AccordionContent {
                            "Try opening multiple sections at once!"
                        }
                    }

                    AccordionItem {
                        value: "feature-3",
                        AccordionTrigger { "Feature 3" }
                        AccordionContent {
                            "All sections can be open simultaneously or all closed."
                        }
                    }
                }
            }

            // Avatar Examples
            div {
                class: "flex gap-4 items-center",
                h2 { class: "text-2xl font-bold", "Avatar Examples:" }
            }

            // Avatar with working image
            div {
                class: "flex gap-4",
                Avatar {
                    AvatarImage {
                        src: "https://github.com/shadcn.png",
                        alt: "User Avatar"
                    }
                    AvatarFallback {
                        "CN"
                    }
                }

                // Another avatar
                Avatar {
                    AvatarFallback {
                        "AB"
                    }
                }
            }

            // Button Example
            div {
                class: "flex gap-4 mt-8",
                Button {
                    variant: ButtonVariant::Default,
                    "Click me"
                }
                Button {
                    variant: ButtonVariant::Destructive,
                    "Delete"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    "Outline"
                }
            }

            // Badge Example
            div {
                class: "flex gap-4 flex-wrap mt-8",
                h2 { class: "w-full text-2xl font-bold", "Badge Examples:" }
                Badge {
                    "Default"
                }
                Badge {
                    variant: BadgeVariant::Secondary,
                    "Secondary"
                }
                Badge {
                    variant: BadgeVariant::Destructive,
                    "Destructive"
                }
                Badge {
                    variant: BadgeVariant::Outline,
                    "Outline"
                }
            }
        }

    }
}

/// Portal Example Component
#[component]
fn PortalExample() -> Element {
    let mut show_portal = use_signal(|| false);

    rsx! {
        div {
            class: "space-y-4",

            Button {
                variant: ButtonVariant::Default,
                onclick: move |_| show_portal.set(!show_portal()),
                if show_portal() {
                    "Hide Portal"
                } else {
                    "Show Portal"
                }
            }

            // Portal content
            if show_portal() {
                Portal {
                    container: "main",
                    class: "portal-overlay",
                    div {
                        class: "fixed inset-0 bg-black/50 flex items-center justify-center",
                        onclick: move |_| show_portal.set(false),
                        div {
                            class: "bg-card p-8 rounded-lg shadow-lg max-w-md",
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
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}

/// Example showcasing Dialog component
#[component]
fn DialogExample() -> Element {
    rsx! {
        div {
            class: "space-y-4",

            // Basic Dialog Example
            div {
                h3 { class: "text-sm font-semibold mb-2", "Basic Dialog:" }
                Dialog {
                    default_open: false,
                    modal: true,
                    DialogTrigger {
                        class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background bg-primary text-primary-foreground hover:bg-primary/90 h-10 py-2 px-4",
                        "Open Dialog"
                    }
                    DialogOverlay {
                        class: "dialog-overlay"
                    }
                    DialogContent {
                        class: "fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 bg-background p-6 shadow-lg rounded-lg border max-w-lg w-full",
                        container: "body".to_string(),
                        close_on_outside_click: true,
                        close_on_escape: true,
                        DialogTitle {
                            class: "text-lg font-semibold mb-2",
                            "Dialog Title"
                        }
                        DialogDescription {
                            class: "text-sm text-muted-foreground mb-4",
                            "This is a dialog description. It provides additional context about the dialog content."
                        }
                        div {
                            class: "space-y-4",
                            p {
                                class: "text-sm",
                                "This dialog demonstrates the Dialog component with:"
                            }
                            ul {
                                class: "list-disc list-inside text-sm space-y-1 ml-2",
                                li { "Modal overlay that blocks interaction" }
                                li { "Click outside to close" }
                                li { "Press Escape to close" }
                                li { "Accessible with ARIA attributes" }
                                li { "Uses Portal for rendering" }
                            }
                        }
                        div {
                            class: "flex justify-end gap-2 mt-6",
                            DialogClose {
                                class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background border border-input hover:bg-accent hover:text-accent-foreground h-10 py-2 px-4",
                                "Cancel"
                            }
                            DialogClose {
                                class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background bg-primary text-primary-foreground hover:bg-primary/90 h-10 py-2 px-4",
                                "Confirm"
                            }
                        }
                    }
                }
            }

            // Alert Dialog Example
            div {
                h3 { class: "text-sm font-semibold mb-2", "Alert Dialog:" }
                Dialog {
                    default_open: false,
                    modal: true,
                    DialogTrigger {
                        class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background border border-input bg-destructive text-destructive-foreground hover:bg-destructive/90 h-10 py-2 px-4",
                        "Delete Account"
                    }
                    DialogOverlay {}
                    DialogContent {
                        class: "fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 bg-background p-6 shadow-lg rounded-lg border max-w-md w-full",
                        DialogTitle {
                            class: "text-lg font-semibold mb-2 text-destructive",
                            "⚠️ Are you absolutely sure?"
                        }
                        DialogDescription {
                            class: "text-sm text-muted-foreground mb-4",
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                        div {
                            class: "flex justify-end gap-2 mt-6",
                            DialogClose {
                                class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background border border-input hover:bg-accent hover:text-accent-foreground h-10 py-2 px-4",
                                "Cancel"
                            }
                            DialogClose {
                                class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background bg-destructive text-destructive-foreground hover:bg-destructive/90 h-10 py-2 px-4",
                                "Delete Account"
                            }
                        }
                    }
                }
            }

            // Form Dialog Example
            div {
                h3 { class: "text-sm font-semibold mb-2", "Form Dialog:" }
                Dialog {
                    default_open: false,
                    modal: true,
                    DialogTrigger {
                        class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background bg-primary text-primary-foreground hover:bg-primary/90 h-10 py-2 px-4",
                        "Edit Profile"
                    }
                    DialogOverlay {}
                    DialogContent {
                        class: "fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 bg-background p-6 shadow-lg rounded-lg border max-w-md w-full",
                        DialogTitle {
                            class: "text-lg font-semibold mb-2",
                            "Edit Profile"
                        }
                        DialogDescription {
                            class: "text-sm text-muted-foreground mb-4",
                            "Make changes to your profile here. Click save when you're done."
                        }
                        div {
                            class: "space-y-4",
                            div {
                                class: "space-y-2",
                                label {
                                    class: "text-sm font-medium",
                                    "Name"
                                }
                                input {
                                    class: "flex h-10 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                                    r#type: "text",
                                    placeholder: "Enter your name"
                                }
                            }
                            div {
                                class: "space-y-2",
                                label {
                                    class: "text-sm font-medium",
                                    "Username"
                                }
                                input {
                                    class: "flex h-10 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                                    r#type: "text",
                                    placeholder: "@username"
                                }
                            }
                        }
                        div {
                            class: "flex justify-end gap-2 mt-6",
                            DialogClose {
                                class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background border border-input hover:bg-accent hover:text-accent-foreground h-10 py-2 px-4",
                                "Cancel"
                            }
                            DialogClose {
                                class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background bg-primary text-primary-foreground hover:bg-primary/90 h-10 py-2 px-4",
                                "Save Changes"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Card Examples
#[component]
fn CardExample() -> Element {
    rsx! {
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
                        class: "space-y-2",
                        div {
                            class: "flex items-center gap-2",
                            Avatar {
                                class: "h-8 w-8",
                                AvatarImage {
                                    src: "https://github.com/shadcn.png",
                                    alt: "User"
                                }
                                AvatarFallback { "CN" }
                            }
                            div {
                                p { class: "text-sm font-medium", "John Doe" }
                                p { class: "text-xs text-muted-foreground", "john@example.com" }
                            }
                        }
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
                            label {
                                class: "text-sm font-medium",
                                "Email"
                            }
                            input {
                                class: "flex h-10 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm",
                                r#type: "email",
                                placeholder: "m@example.com"
                            }
                        }
                        div {
                            class: "space-y-2",
                            label {
                                class: "text-sm font-medium",
                                "Password"
                            }
                            input {
                                class: "flex h-10 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm",
                                r#type: "password"
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

            // Pricing Card
            Card {
                class: "w-full",
                CardHeader {
                    CardTitle { "Pro Plan" }
                    CardDescription { "For professional use" }
                }
                CardContent {
                    div {
                        class: "space-y-4",
                        div {
                            p { class: "text-4xl font-bold", "$29" }
                            p { class: "text-sm text-muted-foreground", "per month" }
                        }
                        ul {
                            class: "space-y-2 text-sm",
                            li { class: "flex items-center",
                                span { class: "mr-2", "✓" }
                                "Unlimited projects"
                            }
                            li { class: "flex items-center",
                                span { class: "mr-2", "✓" }
                                "Priority support"
                            }
                            li { class: "flex items-center",
                                span { class: "mr-2", "✓" }
                                "Advanced analytics"
                            }
                        }
                    }
                }
                CardFooter {
                    Button {
                        variant: ButtonVariant::Default,
                        class: "w-full",
                        "Subscribe"
                    }
                }
            }

            // Activity Card
            Card {
                class: "w-full",
                CardHeader {
                    CardTitle { "Recent Activity" }
                    CardDescription { "Your latest actions" }
                }
                CardContent {
                    div {
                        class: "space-y-4",
                        div {
                            class: "flex items-start gap-3",
                            Badge { variant: BadgeVariant::Default, "New" }
                            div {
                                p { class: "text-sm font-medium", "Project created" }
                                p { class: "text-xs text-muted-foreground", "2 hours ago" }
                            }
                        }
                        div {
                            class: "flex items-start gap-3",
                            Badge { variant: BadgeVariant::Secondary, "Update" }
                            div {
                                p { class: "text-sm font-medium", "Settings changed" }
                                p { class: "text-xs text-muted-foreground", "5 hours ago" }
                            }
                        }
                    }
                }
                CardFooter {
                    class: "border-t",
                    Button {
                        variant: ButtonVariant::Ghost,
                        class: "w-full",
                        "View all activity"
                    }
                }
            }
        }
    }
}
