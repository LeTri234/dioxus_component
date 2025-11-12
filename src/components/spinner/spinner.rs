//! # Spinner Component
//!
//! A loading spinner component using the Lucide loader-2 icon.
//!
//! ## Example
//!
//! ```rust
//! use dioxus::prelude::*;
//! use crate::components::spinner::*;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Spinner {}
//!         Spinner {
//!             size: SpinnerSize::Large,
//!             color: "text-primary"
//!         }
//!     }
//! }
//! ```

use crate::utils;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SpinnerSize {
    Small,  // size-4 (16px)
    Medium, // size-6 (24px)
    Large,  // size-8 (32px)
    XLarge, // size-12 (48px)
}

impl SpinnerSize {
    fn to_class(&self) -> &'static str {
        match self {
            SpinnerSize::Small => "size-4",
            SpinnerSize::Medium => "size-6",
            SpinnerSize::Large => "size-8",
            SpinnerSize::XLarge => "size-12",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SpinnerProps {
    /// Additional CSS classes
    #[props(optional)]
    pub class: Option<String>,

    /// Size of the spinner
    #[props(default = SpinnerSize::Small)]
    pub size: SpinnerSize,

    /// Color class (e.g., "text-primary", "text-white")
    #[props(optional)]
    pub color: Option<String>,
}

#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    let color_class = props.color.as_deref().unwrap_or("text-current");

    let class_name = utils::cn(vec![
        Some(props.size.to_class()),
        Some("animate-spin"),
        Some(color_class),
        props.class.as_deref(),
    ]);

    rsx! {
        svg {
            role: "status",
            "aria-label": "Loading",
            class: "{class_name}",
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",

            path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
        }
    }
}
