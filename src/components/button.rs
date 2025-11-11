use dioxus::prelude::*;

use crate::utils;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
            ButtonVariant::Outline => "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50",
            ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
            ButtonSize::Sm => "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5",
            ButtonSize::Lg => "h-10 rounded-md px-6 has-[>svg]:px-4",
            ButtonSize::Icon => "size-9",
            ButtonSize::IconSm => "size-8",
            ButtonSize::IconLg => "size-10",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default)]
    pub children: Element,

    // Optional extra class names from the caller.
    #[props(optional)]
    pub class_name: Option<String>,

    // Variant and size are optional; we'll fall back to defaults if not provided.
    #[props(optional)]
    pub variant: Option<ButtonVariant>,
    #[props(optional)]
    pub size: Option<ButtonSize>,

    // If provided, use this HTML tag instead of a button (e.g. "a", "div", "span").
    // Common use-case: as_ = "a" to render a link-styled button.
    #[props(optional)]
    pub as_: Option<String>,

    // Common HTML attributes used by buttons/links.
    #[props(optional)]
    pub disabled: Option<bool>,
    #[props(optional)]
    pub href: Option<String>,

    // Optional click handler.
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    // Support aria-invalid toggling similar to the TSX original.
    #[props(optional)]
    pub aria_invalid: Option<bool>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant = props.variant.unwrap_or(ButtonVariant::Default);
    let size = props.size.unwrap_or(ButtonSize::Default);
    let base = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";
    let class_name = utils::cn(vec![
        Some(base),
        Some(variant.as_str()),
        Some(size.as_str()),
        props.class_name.as_deref(),
    ]);

    let tag = props.as_.as_deref().unwrap_or("button");
    let aria_invalid_attr = props.aria_invalid.unwrap_or(false);

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
                aria_invalid: "{aria_invalid_attr}",
                {props.children}
            }
        },
        "div" => rsx! {
            div {
                class: "{class_name}",
                onclick: move |e| { if let Some(handler) = &props.onclick { handler.call(e); } },
                aria_invalid: "{aria_invalid_attr}",
                {props.children}
            }
        },
        "span" => rsx! {
            span {
                class: "{class_name}",
                onclick: move |e| { if let Some(handler) = &props.onclick { handler.call(e); } },
                aria_invalid: "{aria_invalid_attr}",
                {props.children}
            }
        },
        _ => rsx!(
            button {
                class: "{class_name}",
                disabled: "{props.disabled.unwrap_or(false)}",
                onclick: move |e| { if let Some(handler) = &props.onclick { handler.call(e); } },
                aria_invalid: "{aria_invalid_attr}",
                {props.children}
            }
        ),
    }
}
