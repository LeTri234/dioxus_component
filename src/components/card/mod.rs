use crate::utils::cn;
use dioxus::prelude::*;

/// A Card component that provides a container with consistent styling.
///
/// # Example
/// ```rust
/// rsx! {
///     Card {
///         class: "w-[350px]",
///         CardHeader {
///             CardTitle { "Card Title" }
///             CardDescription { "Card Description" }
///         }
///         CardContent {
///             p { "Card content goes here." }
///         }
///         CardFooter {
///             p { "Card footer" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Card(
    /// Additional CSS classes to apply to the card
    class: Option<String>,
    /// The content to render inside the card
    children: Element,
) -> Element {
    let classes = cn([
        Some("bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm"),
        class.as_deref(),
    ]);

    rsx! {
        div {
            "data-slot": "card",
            class: "{classes}",
            {children}
        }
    }
}

/// A CardHeader component that provides a header section for the card.
///
/// Automatically handles layout for title, description, and action components.
#[component]
pub fn CardHeader(
    /// Additional CSS classes to apply to the header
    class: Option<String>,
    /// The content to render inside the header
    children: Element,
) -> Element {
    let classes = cn([
        Some("grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6 has-[data-slot=card-action]:grid-cols-[1fr_auto] [&.border-b]:pb-6"),
        class.as_deref(),
    ]);

    rsx! {
        div {
            "data-slot": "card-header",
            class: "{classes}",
            {children}
        }
    }
}

/// A CardTitle component for the card's main heading.
#[component]
pub fn CardTitle(
    /// Additional CSS classes to apply to the title
    class: Option<String>,
    /// The content to render inside the title
    children: Element,
) -> Element {
    let classes = cn([Some("leading-none font-semibold"), class.as_deref()]);

    rsx! {
        div {
            "data-slot": "card-title",
            class: "{classes}",
            {children}
        }
    }
}

/// A CardDescription component for supplementary text in the card header.
#[component]
pub fn CardDescription(
    /// Additional CSS classes to apply to the description
    class: Option<String>,
    /// The content to render inside the description
    children: Element,
) -> Element {
    let classes = cn([Some("text-muted-foreground text-sm"), class.as_deref()]);

    rsx! {
        div {
            "data-slot": "card-description",
            class: "{classes}",
            {children}
        }
    }
}

/// A CardAction component for action buttons or controls in the header.
///
/// Automatically positions itself in the top-right corner of the CardHeader.
#[component]
pub fn CardAction(
    /// Additional CSS classes to apply to the action
    class: Option<String>,
    /// The content to render inside the action area
    children: Element,
) -> Element {
    let classes = cn([
        Some("col-start-2 row-span-2 row-start-1 self-start justify-self-end"),
        class.as_deref(),
    ]);

    rsx! {
        div {
            "data-slot": "card-action",
            class: "{classes}",
            {children}
        }
    }
}

/// A CardContent component for the main content area of the card.
#[component]
pub fn CardContent(
    /// Additional CSS classes to apply to the content
    class: Option<String>,
    /// The content to render inside the content area
    children: Element,
) -> Element {
    let classes = cn([Some("px-6"), class.as_deref()]);

    rsx! {
        div {
            "data-slot": "card-content",
            class: "{classes}",
            {children}
        }
    }
}

/// A CardFooter component for the footer section of the card.
#[component]
pub fn CardFooter(
    /// Additional CSS classes to apply to the footer
    class: Option<String>,
    /// The content to render inside the footer
    children: Element,
) -> Element {
    let classes = cn([
        Some("flex items-center px-6 [&.border-t]:pt-6"),
        class.as_deref(),
    ]);

    rsx! {
        div {
            "data-slot": "card-footer",
            class: "{classes}",
            {children}
        }
    }
}
