//! # Dioxus Components Library
//!
//! A comprehensive collection of reusable Dioxus 0.7 components built with Tailwind CSS v4
//! and following Radix UI design patterns.
//!
//! ## Components
//!
//! - **Accordion** - Vertically stacked interactive headings with reveal sections
//! - **Avatar** - User avatar display with image loading and fallback support
//! - **Badge** - Flexible badge component with multiple style variants
//! - **Button** - Versatile button with 6 variants and 6 size options
//! - **Card** - Flexible card container with header, content, and footer sections
//! - **Dialog** - Accessible modal dialogs with overlay, focus trap, and keyboard handling
//! - **Portal** - Render content outside the parent DOM hierarchy (modals, overlays)
//! - **Spinner** - Loading indicator with multiple sizes and colors
//! - **Tooltip** - Hover-triggered tooltips with flexible positioning
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! dioxus_components = { version = "0.1.0", path = "." }
//! dioxus = { version = "0.7.1", features = ["web"] }
//! ```
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use dioxus::prelude::*;
//! use dioxus_components::{Button, ButtonVariant, Spinner, SpinnerSize};
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         div {
//!             class: "flex gap-4",
//!             Button {
//!                 variant: ButtonVariant::Default,
//!                 "Click me"
//!             }
//!             Spinner {
//!                 size: SpinnerSize::Large,
//!                 color: "text-primary"
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## Styling
//!
//! All components use Tailwind CSS v4. Import the component styles in your `tailwind.css`:
//!
//!
//! For a complete example, see the `examples/` directory.

pub mod components;
pub mod utils;

// Re-export commonly used items
pub use components::{
    accordion::{
        Accordion, AccordionContent, AccordionItem, AccordionOrientation, AccordionTrigger,
        AccordionType,
    },
    avatar::{Avatar, AvatarFallback, AvatarImage, ImageLoadingStatus},
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
    checkbox::{
        Checkbox, CheckboxBubbleInput, CheckboxContext, CheckboxIndicator, CheckboxLabel,
        CheckboxProvider, CheckboxTrigger, CheckedState,
    },
    dialog::{
        Dialog, DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogTitle,
        DialogTrigger,
    },
    portal::Portal,
    spinner::{Spinner, SpinnerSize},
    tooltip::{
        Tooltip, TooltipArrow, TooltipContent, TooltipProvider, TooltipSide, TooltipTrigger,
    },
};

pub use utils::cn;
