use crate::utils::cn;
use dioxus::prelude::*;

/// An Empty state component for displaying "no content" states.
///
/// # Example
/// ```rust
/// rsx! {
///     Empty {
///         EmptyHeader {
///             EmptyMedia {
///                 variant: EmptyMediaVariant::Icon,
///                 // Icon here
///             }
///             EmptyTitle { "No results found" }
///             EmptyDescription { "Try adjusting your search" }
///         }
///         EmptyContent {
///             Button { "Clear filters" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Empty(
    /// Additional CSS classes to apply to the empty state
    class: Option<String>,
    /// The content to render inside the empty state
    children: Element,
) -> Element {
    let classes = cn([
        Some("flex min-w-0 flex-1 flex-col items-center justify-center gap-6 rounded-lg border-dashed p-6 text-center text-balance md:p-12"),
        class.as_deref(),
    ]);

    rsx! {
        div {
            "data-slot": "empty",
            class: "{classes}",
            {children}
        }
    }
}

/// Header section of the Empty state, typically containing media, title, and description.
#[component]
pub fn EmptyHeader(
    /// Additional CSS classes to apply to the header
    class: Option<String>,
    /// The content to render inside the header
    children: Element,
) -> Element {
    let classes = cn([
        Some("flex max-w-sm flex-col items-center gap-2 text-center"),
        class.as_deref(),
    ]);

    rsx! {
        div {
            "data-slot": "empty-header",
            class: "{classes}",
            {children}
        }
    }
}

/// Media variants for the EmptyMedia component
#[derive(Clone, Copy, PartialEq, Default)]
pub enum EmptyMediaVariant {
    /// Transparent background (default)
    #[default]
    Default,
    /// Icon with muted background
    Icon,
}

impl EmptyMediaVariant {
    fn as_str(&self) -> &str {
        match self {
            EmptyMediaVariant::Default => "default",
            EmptyMediaVariant::Icon => "icon",
        }
    }

    fn classes(&self) -> &str {
        match self {
            EmptyMediaVariant::Default => "bg-transparent",
            EmptyMediaVariant::Icon => "bg-muted text-foreground flex size-10 shrink-0 items-center justify-center rounded-lg [&_svg:not([class*='size-'])]:size-6",
        }
    }
}

/// Media container for icons or images in the Empty state.
#[component]
pub fn EmptyMedia(
    /// Visual style variant
    #[props(default)]
    variant: EmptyMediaVariant,
    /// Additional CSS classes to apply to the media
    class: Option<String>,
    /// The content to render inside the media container
    children: Element,
) -> Element {
    let classes = cn([
        Some("flex shrink-0 items-center justify-center mb-2 [&_svg]:pointer-events-none [&_svg]:shrink-0"),
        Some(variant.classes()),
        class.as_deref(),
    ]);

    rsx! {
        div {
            "data-slot": "empty-icon",
            "data-variant": variant.as_str(),
            class: "{classes}",
            {children}
        }
    }
}

/// Title for the Empty state.
#[component]
pub fn EmptyTitle(
    /// Additional CSS classes to apply to the title
    class: Option<String>,
    /// The content to render inside the title
    children: Element,
) -> Element {
    let classes = cn([Some("text-lg font-medium tracking-tight"), class.as_deref()]);

    rsx! {
        div {
            "data-slot": "empty-title",
            class: "{classes}",
            {children}
        }
    }
}

/// Description text for the Empty state.
#[component]
pub fn EmptyDescription(
    /// Additional CSS classes to apply to the description
    class: Option<String>,
    /// The content to render inside the description
    children: Element,
) -> Element {
    let classes = cn([
        Some("text-muted-foreground [&>a:hover]:text-primary text-sm/relaxed [&>a]:underline [&>a]:underline-offset-4"),
        class.as_deref(),
    ]);

    rsx! {
        div {
            "data-slot": "empty-description",
            class: "{classes}",
            {children}
        }
    }
}

/// Content area for action buttons or additional content.
#[component]
pub fn EmptyContent(
    /// Additional CSS classes to apply to the content
    class: Option<String>,
    /// The content to render inside the content area
    children: Element,
) -> Element {
    let classes = cn([
        Some("flex w-full max-w-sm min-w-0 flex-col items-center gap-4 text-sm text-balance"),
        class.as_deref(),
    ]);

    rsx! {
        div {
            "data-slot": "empty-content",
            class: "{classes}",
            {children}
        }
    }
}
