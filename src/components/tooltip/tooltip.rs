//! # Tooltip Component
//!
//! A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.
//!
//! ## Example
//!
//! ```rust
//! use dioxus::prelude::*;
//! use crate::components::tooltip::*;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         TooltipProvider {
//!             Tooltip {
//!                 TooltipTrigger {
//!                     button { "Hover me" }
//!                 }
//!                 TooltipContent {
//!                     side: TooltipSide::Top,
//!                     "This is a tooltip"
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use crate::utils;
use dioxus::prelude::*;

const DEFAULT_DELAY_DURATION: u64 = 400;
const SKIP_DELAY_DURATION: u64 = 300;

/* -------------------------------------------------------------------------------------------------
 * TooltipProvider
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProviderProps {
    #[props(default)]
    pub children: Element,

    /// The duration from when the pointer enters the trigger until the tooltip gets opened.
    #[props(default = DEFAULT_DELAY_DURATION)]
    pub delay_duration: u64,

    /// How much time a user has to enter another trigger without incurring a delay again.
    #[props(default = SKIP_DELAY_DURATION)]
    pub skip_delay_duration: u64,

    /// When true, trying to hover the content will result in the tooltip closing as the pointer leaves the trigger.
    #[props(default = false)]
    pub disable_hoverable_content: bool,
}

#[component]
pub fn TooltipProvider(props: TooltipProviderProps) -> Element {
    let is_open_delayed = use_signal(|| true);
    let is_pointer_in_transit = use_signal(|| false);

    use_context_provider(|| TooltipProviderContext {
        is_open_delayed,
        delay_duration: props.delay_duration,
        skip_delay_duration: props.skip_delay_duration,
        is_pointer_in_transit,
        disable_hoverable_content: props.disable_hoverable_content,
    });

    rsx! { {props.children} }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct TooltipProviderContext {
    is_open_delayed: Signal<bool>,
    delay_duration: u64,
    skip_delay_duration: u64,
    is_pointer_in_transit: Signal<bool>,
    disable_hoverable_content: bool,
}

/* -------------------------------------------------------------------------------------------------
 * Tooltip
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    #[props(default)]
    pub children: Element,

    /// The controlled open state of the tooltip.
    #[props(optional)]
    pub open: Option<Signal<bool>>,

    /// The open state of the tooltip when it is initially rendered.
    #[props(default = false)]
    pub default_open: bool,

    /// The duration from when the pointer enters the trigger until the tooltip gets opened.
    #[props(optional)]
    pub delay_duration: Option<u64>,

    /// When true, trying to hover the content will result in the tooltip closing.
    #[props(optional)]
    pub disable_hoverable_content: Option<bool>,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let provider_context = use_context::<TooltipProviderContext>();

    let open = use_signal(|| props.default_open);
    let was_open_delayed = use_signal(|| false);
    let open_timer = use_signal(|| None::<i32>);

    let delay_duration = props
        .delay_duration
        .unwrap_or(provider_context.delay_duration);
    let disable_hoverable_content = props
        .disable_hoverable_content
        .unwrap_or(provider_context.disable_hoverable_content);

    use_context_provider(|| TooltipContext {
        open,
        delay_duration,
        disable_hoverable_content,
        was_open_delayed,
        open_timer,
        is_open_delayed: provider_context.is_open_delayed,
        skip_delay_duration: provider_context.skip_delay_duration,
    });

    // Cleanup timer on unmount - moved into the effect

    rsx! {
        div {
            class: "inline-block relative",
            {props.children}
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct TooltipContext {
    open: Signal<bool>,
    delay_duration: u64,
    disable_hoverable_content: bool,
    was_open_delayed: Signal<bool>,
    open_timer: Signal<Option<i32>>,
    is_open_delayed: Signal<bool>,
    skip_delay_duration: u64,
}

#[allow(dead_code)]
impl TooltipContext {
    fn get_state_attribute(&self) -> &'static str {
        if *self.open.read() {
            if *self.was_open_delayed.read() {
                "delayed-open"
            } else {
                "instant-open"
            }
        } else {
            "closed"
        }
    }

    fn handle_open(&mut self) {
        if let Some(timer_id) = *self.open_timer.read() {
            clear_timeout(timer_id);
        }
        *self.open_timer.write() = None;
        *self.was_open_delayed.write() = false;
        *self.open.write() = true;
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"Tooltip opened".into());
    }

    fn handle_close(&mut self) {
        if let Some(timer_id) = *self.open_timer.read() {
            clear_timeout(timer_id);
        }
        *self.open_timer.write() = None;
        *self.open.write() = false;
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"Tooltip closed".into());
    }

    fn handle_delayed_open(&mut self) {
        if let Some(timer_id) = *self.open_timer.read() {
            clear_timeout(timer_id);
        }

        let delay = self.delay_duration;
        let mut open = self.open;
        let mut was_delayed = self.was_open_delayed;
        let mut timer = self.open_timer;

        let timer_id = set_timeout(
            move || {
                *was_delayed.write() = true;
                *open.write() = true;
                *timer.write() = None;
            },
            delay,
        );

        *self.open_timer.write() = Some(timer_id);
    }

    fn on_trigger_enter(&mut self) {
        // Cancel any pending close timer
        let timer_val = *self.open_timer.read();
        if let Some(timer_id) = timer_val {
            clear_timeout(timer_id);
            *self.open_timer.write() = None;
        }

        // Only start opening if not already open
        if !*self.open.read() {
            if *self.is_open_delayed.read() {
                self.handle_delayed_open();
            } else {
                self.handle_open();
            }
        }
    }

    fn on_trigger_leave(&mut self) {
        // Clear any pending open timer if tooltip hasn't opened yet
        let timer_val = *self.open_timer.read();
        if let Some(timer_id) = timer_val {
            clear_timeout(timer_id);
            *self.open_timer.write() = None;
        }

        // Don't close immediately - give time to move to content
        // Set a delay to close, which will be cancelled if mouse enters content
        if !self.disable_hoverable_content && *self.open.read() {
            let mut context = *self;
            let close_timer_id = set_timeout(
                move || {
                    context.handle_close();
                },
                300, // 300ms grace period to move to content
            );
            *self.open_timer.write() = Some(close_timer_id);
        } else if !*self.open.read() {
            // If not open yet, just cancel opening
            self.handle_close();
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * TooltipTrigger
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct TooltipTriggerProps {
    #[props(default)]
    pub children: Element,

    #[props(optional)]
    pub class: Option<String>,

    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let mut context = use_context::<TooltipContext>();
    let mut is_pointer_down = use_signal(|| false);

    let class_name = utils::cn(vec![props.class.as_deref()]);

    rsx! {
        span {
            class: "{class_name}",
            "data-state": "{context.get_state_attribute()}",
            aria_describedby: if *context.open.read() { "tooltip-content" } else { "" },

            onmouseenter: move |_| {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&"Mouse entered trigger".into());
                context.on_trigger_enter();
            },

            onmouseleave: move |_| {
                context.on_trigger_leave();
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&"Mouse left trigger".into());
            },

            onmousedown: move |_| {
                if *context.open.read() {
                    context.handle_close();
                }
                *is_pointer_down.write() = true;
            },

            onmouseup: move |_| {
                *is_pointer_down.write() = false;
            },

            onfocus: move |_| {
                if !is_pointer_down() {
                    context.handle_open();
                }
            },

            onblur: move |_| {
                context.handle_close();
            },

            onkeydown: move |evt| {
                if evt.key().to_string() == "Escape" {
                    context.handle_close();
                }
            },

            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
                // Close on click/activation
                context.handle_close();
            },

            {props.children}
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * TooltipContent
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum TooltipSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipSide {
    #[allow(dead_code)]
    fn as_str(&self) -> &'static str {
        match self {
            TooltipSide::Top => "top",
            TooltipSide::Right => "right",
            TooltipSide::Bottom => "bottom",
            TooltipSide::Left => "left",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TooltipContentProps {
    #[props(default)]
    pub children: Element,

    #[props(optional)]
    pub class: Option<String>,

    /// The preferred side of the trigger to render against when open.
    #[props(default = TooltipSide::Top)]
    pub side: TooltipSide,

    /// The distance in pixels from the trigger.
    #[props(default = 4)]
    pub side_offset: i32,

    /// A more descriptive label for accessibility purpose.
    #[props(optional)]
    pub aria_label: Option<String>,
}

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let context = use_context::<TooltipContext>();

    // Base styling inspired by Radix UI
    let base_class = "tooltip-content absolute z-50 rounded px-[15px] py-[10px] text-[15px] leading-none select-none bg-white text-primary shadow-[hsl(206_22%_7%_/_35%)_0px_10px_38px_-10px,_hsl(206_22%_7%_/_20%)_0px_10px_20px_-15px] dark:bg-gray-800 dark:text-white";

    let position_class = match props.side {
        TooltipSide::Top => "bottom-full left-1/2 -translate-x-1/2 mb-2",
        TooltipSide::Right => "left-full top-1/2 -translate-y-1/2 ml-2",
        TooltipSide::Bottom => "top-full left-1/2 -translate-x-1/2 mt-2",
        TooltipSide::Left => "right-full top-1/2 -translate-y-1/2 mr-2",
    };

    let class_name = utils::cn(vec![
        Some(base_class),
        Some(position_class),
        props.class.as_deref(),
    ]);

    // Close tooltip if trigger is scrolled or if Escape is pressed
    // Note: This is a simplified version. Full implementation would need proper event listeners

    rsx! {
        if *context.open.read() {
            div {
                id: "tooltip-content",
                role: "tooltip",
                class: "{class_name}",
                "data-state": "{context.get_state_attribute()}",
                "data-side": "{props.side.as_str()}",

                onmouseenter: move |_| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        web_sys::console::log_1(&format!("Mouse entered content - state: {}, side: {}", context.get_state_attribute(), props.side.as_str()).into());
                    }
                    // Cancel any pending close timer when entering content
                    let mut ctx = context;
                    let timer_val = *ctx.open_timer.read();
                    if let Some(timer_id) = timer_val {
                        clear_timeout(timer_id);
                        *ctx.open_timer.write() = None;
                    }
                },

                onmouseleave: move |_| {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&"Mouse left tooltip content".into());
                    // Set a delay before closing to allow moving back to trigger
                    let mut ctx = context;
                    let close_timer_id = set_timeout(
                        move || {
                            ctx.handle_close();
                        },
                        300, // 300ms grace period to move back to trigger
                    );
                    *ctx.open_timer.write() = Some(close_timer_id);
                },

                {props.children}
            }
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * TooltipArrow
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct TooltipArrowProps {
    #[props(optional)]
    pub class: Option<String>,

    #[props(default = 10)]
    pub width: u32,

    #[props(default = 5)]
    pub height: u32,
}

#[component]
pub fn TooltipArrow(props: TooltipArrowProps) -> Element {
    let class_name = utils::cn(vec![Some("fill-primary"), props.class.as_deref()]);

    rsx! {
        svg {
            width: "{props.width}",
            height: "{props.height}",
            view_box: "0 0 30 10",
            preserve_aspect_ratio: "none",
            class: "{class_name}",

            polygon { points: "0,0 30,0 15,10" }
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * Helper functions for WASM environment
 * -----------------------------------------------------------------------------------------------*/

#[cfg(target_arch = "wasm32")]
fn set_timeout<F>(f: F, delay_ms: u64) -> i32
where
    F: FnOnce() + 'static,
{
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    let closure = Closure::once(f);
    let window = web_sys::window().expect("no global window exists");
    let id = window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            delay_ms as i32,
        )
        .expect("should set timeout");
    closure.forget();
    id
}

#[cfg(not(target_arch = "wasm32"))]
fn set_timeout<F>(_f: F, _delay_ms: u64) -> i32
where
    F: FnOnce() + 'static,
{
    0
}

#[cfg(target_arch = "wasm32")]
fn clear_timeout(id: i32) {
    if let Some(window) = web_sys::window() {
        window.clear_timeout_with_handle(id);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_timeout(_id: i32) {}
