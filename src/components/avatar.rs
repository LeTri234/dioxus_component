//! Avatar Component
//!
//! A Dioxus 0.7 port of Radix UI Avatar component.
//!
//! # Example
//!
//! ```rust
//! use dioxus::prelude::*;
//! use components::{Avatar, AvatarImage, AvatarFallback};
//!
//! #[component]
//! fn MyComponent() -> Element {
//!     rsx! {
//!         Avatar {
//!             AvatarImage {
//!                 src: "https://github.com/shadcn.png",
//!                 alt: "User Avatar"
//!             }
//!             AvatarFallback {
//!                 "CN"
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::utils;

/* -------------------------------------------------------------------------------------------------
 * Avatar Context
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ImageLoadingStatus {
    Idle,
    Loading,
    Loaded,
    Error,
}

/* -------------------------------------------------------------------------------------------------
 * Avatar
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    #[props(default)]
    pub children: Element,

    #[props(optional)]
    pub class_name: Option<String>,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let image_loading_status = use_signal(|| ImageLoadingStatus::Idle);

    // Provide context to children
    use_context_provider(|| image_loading_status);

    let class_name = utils::cn(vec![
        Some("relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full"),
        props.class_name.as_deref(),
    ]);

    rsx! {
        span {
            class: "{class_name}",
            {props.children}
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * AvatarImage
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct AvatarImageProps {
    pub src: String,

    #[props(optional)]
    pub alt: Option<String>,

    #[props(optional)]
    pub class_name: Option<String>,

    #[props(optional)]
    pub referrer_policy: Option<String>,

    #[props(optional)]
    pub cross_origin: Option<String>,

    #[props(optional)]
    pub on_loading_status_change: Option<EventHandler<ImageLoadingStatus>>,
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    let mut image_loading_status = use_context::<Signal<ImageLoadingStatus>>();
    let mut local_status = use_signal(|| ImageLoadingStatus::Idle);
    let mut is_mounted = use_signal(|| false);

    // Initialize loading status
    use_effect(move || {
        if !is_mounted() {
            local_status.set(ImageLoadingStatus::Loading);
            is_mounted.set(true);
        }
    });

    // Update context when local status changes
    use_effect(move || {
        let status = local_status();
        image_loading_status.set(status);
        if let Some(handler) = &props.on_loading_status_change {
            handler.call(status);
        }
    });

    let class_name = utils::cn(vec![
        Some("aspect-square h-full w-full object-cover"),
        props.class_name.as_deref(),
    ]);

    let src = props.src.clone();
    let alt = props.alt.clone().unwrap_or_default();

    match local_status() {
        ImageLoadingStatus::Loaded | ImageLoadingStatus::Loading => rsx! {
            img {
                class: "{class_name}",
                src: "{src}",
                alt: "{alt}",
                referrerpolicy: props.referrer_policy.as_deref(),
                crossorigin: props.cross_origin.as_deref(),
                onload: move |_| {
                    local_status.set(ImageLoadingStatus::Loaded);
                },
                onerror: move |_| {
                    local_status.set(ImageLoadingStatus::Error);
                },
            }
        },
        _ => rsx! {},
    }
}

/* -------------------------------------------------------------------------------------------------
 * AvatarFallback
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    #[props(default)]
    pub children: Element,

    #[props(optional)]
    pub class_name: Option<String>,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    let image_loading_status = use_context::<Signal<ImageLoadingStatus>>();

    let class_name = utils::cn(vec![
        Some("flex h-full w-full items-center justify-center rounded-full bg-muted"),
        props.class_name.as_deref(),
    ]);

    // Only render if image is not loaded
    if image_loading_status() != ImageLoadingStatus::Loaded {
        rsx! {
            span {
                class: "{class_name}",
                {props.children}
            }
        }
    } else {
        rsx! {}
    }
}
