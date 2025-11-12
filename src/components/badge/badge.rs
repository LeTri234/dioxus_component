//! # Badge Component
//!
//! A flexible badge component with multiple variant options.
//!
//! ## Example
//!
//! ```rust
//! use dioxus::prelude::*;
//! use crate::components::badge::*;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Badge {
//!             "Default"
//!         }
//!         Badge {
//!             variant: BadgeVariant::Secondary,
//!             "Secondary"
//!         }
//!         Badge {
//!             variant: BadgeVariant::Destructive,
//!             "Destructive"
//!         }
//!         Badge {
//!             variant: BadgeVariant::Outline,
//!             "Outline"
//!         }
//!     }
//! }
//! ```

use crate::utils;
use dioxus::prelude::*;

/* -------------------------------------------------------------------------------------------------
 * Badge Variant
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    fn as_str(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "border-transparent bg-primary text-primary-foreground [a&]:hover:bg-primary/90",
            BadgeVariant::Secondary => "border-transparent bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90",
            BadgeVariant::Destructive => "border-transparent bg-destructive text-white [a&]:hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
            BadgeVariant::Outline => "text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground",
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * Badge
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default)]
    pub children: Element,

    /// Badge variant
    #[props(default = BadgeVariant::Default)]
    pub variant: BadgeVariant,

    /// Additional CSS classes
    #[props(optional)]
    pub class: Option<String>,

    /// Optional HTML tag to render as (e.g., "a", "button", "span")
    #[props(default = "span".to_string())]
    pub as_: String,

    /// Href for links
    #[props(optional)]
    pub href: Option<String>,

    /// Click handler
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let base_class = "inline-flex items-center justify-center rounded-full border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden";

    let class_name = utils::cn(vec![
        Some(base_class),
        Some(props.variant.as_str()),
        props.class.as_deref(),
    ]);

    let tag = props.as_.as_str();

    match tag {
        "a" => rsx! {
            a {
                class: "{class_name}",
                href: props.href.as_deref().unwrap_or("#"),
                onclick: move |e| {
                    if let Some(cb) = &props.onclick {
                        cb.call(e);
                    }
                },
                {props.children}
            }
        },
        "button" => rsx! {
            button {
                class: "{class_name}",
                onclick: move |e| {
                    if let Some(cb) = &props.onclick {
                        cb.call(e);
                    }
                },
                {props.children}
            }
        },
        "div" => rsx! {
            div {
                class: "{class_name}",
                onclick: move |e| {
                    if let Some(cb) = &props.onclick {
                        cb.call(e);
                    }
                },
                {props.children}
            }
        },
        _ => rsx! {
            span {
                class: "{class_name}",
                onclick: move |e| {
                    if let Some(cb) = &props.onclick {
                        cb.call(e);
                    }
                },
                {props.children}
            }
        },
    }
}
