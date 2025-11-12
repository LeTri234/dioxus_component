//! # Accordion Component
//!
//! A vertically stacked set of interactive headings that each reveal an associated section of content.
//!
//! ## Example
//!
//! ```rust
//! use dioxus::prelude::*;
//! use crate::components::accordion::*;
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         Accordion {
//!             accordion_type: AccordionType::Single { collapsible: true },
//!             AccordionItem {
//!                 value: "item-1",
//!                 AccordionTrigger { "Is it accessible?" }
//!                 AccordionContent {
//!                     "Yes. It adheres to the WAI-ARIA design pattern."
//!                 }
//!             }
//!             AccordionItem {
//!                 value: "item-2",
//!                 AccordionTrigger { "Is it styled?" }
//!                 AccordionContent {
//!                     "Yes. It comes with default styles that match the other components."
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use crate::utils;
use dioxus::prelude::*;

// Import component CSS
const ACCORDION_CSS: &str = include_str!("./accordion.css");

/* -------------------------------------------------------------------------------------------------
 * Accordion Types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, PartialEq)]
pub enum AccordionType {
    Single { collapsible: bool },
    Multiple,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AccordionOrientation {
    Vertical,
    Horizontal,
}

impl AccordionOrientation {
    fn as_str(&self) -> &'static str {
        match self {
            AccordionOrientation::Vertical => "vertical",
            AccordionOrientation::Horizontal => "horizontal",
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * Accordion
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    #[props(default)]
    pub children: Element,

    /// Type of accordion (single or multiple items open)
    #[props(default = AccordionType::Single { collapsible: false })]
    pub accordion_type: AccordionType,

    /// Default value for single accordion
    #[props(optional)]
    pub default_value: Option<String>,

    /// Default values for multiple accordion
    #[props(optional)]
    pub default_values: Option<Vec<String>>,

    /// Controlled value for single accordion
    #[props(optional)]
    pub value: Option<Signal<String>>,

    /// Controlled values for multiple accordion
    #[props(optional)]
    pub values: Option<Signal<Vec<String>>>,

    /// Whether the accordion is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Orientation of the accordion
    #[props(default = AccordionOrientation::Vertical)]
    pub orientation: AccordionOrientation,

    /// Additional CSS classes
    #[props(optional)]
    pub class: Option<String>,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    // State management for single accordion
    let single_value = use_signal(|| props.default_value.clone().unwrap_or_default());

    // State management for multiple accordion
    let multiple_values = use_signal(|| props.default_values.clone().unwrap_or_default());

    use_context_provider(|| AccordionContext {
        accordion_type: props.accordion_type.clone(),
        single_value,
        multiple_values,
        controlled_value: props.value,
        controlled_values: props.values,
        disabled: props.disabled,
        orientation: props.orientation,
    });

    rsx! {
        // Inject component styles
        style { {ACCORDION_CSS} }

        div {
            class: if let Some(cls) = props.class.as_ref() { cls.as_str() } else { "" },
            "data-orientation": "{props.orientation.as_str()}",
            {props.children}
        }
    }
}

#[derive(Clone)]
struct AccordionContext {
    accordion_type: AccordionType,
    single_value: Signal<String>,
    multiple_values: Signal<Vec<String>>,
    controlled_value: Option<Signal<String>>,
    controlled_values: Option<Signal<Vec<String>>>,
    disabled: bool,
    orientation: AccordionOrientation,
}

impl AccordionContext {
    fn is_open(&self, value: &str) -> bool {
        match &self.accordion_type {
            AccordionType::Single { .. } => {
                if let Some(controlled) = self.controlled_value {
                    controlled() == value
                } else {
                    (self.single_value)() == value
                }
            }
            AccordionType::Multiple => {
                if let Some(controlled) = self.controlled_values {
                    controlled().contains(&value.to_string())
                } else {
                    (self.multiple_values)().contains(&value.to_string())
                }
            }
        }
    }

    fn toggle(&mut self, value: &str) {
        match &self.accordion_type {
            AccordionType::Single { collapsible } => {
                if let Some(mut controlled) = self.controlled_value {
                    let current = controlled();
                    if current == value && *collapsible {
                        *controlled.write() = String::new();
                    } else if current != value {
                        *controlled.write() = value.to_string();
                    }
                } else {
                    let current = (self.single_value)();
                    if current == value && *collapsible {
                        *self.single_value.write() = String::new();
                    } else if current != value {
                        *self.single_value.write() = value.to_string();
                    }
                }
            }
            AccordionType::Multiple => {
                if let Some(mut controlled) = self.controlled_values {
                    let mut current = controlled();
                    if let Some(pos) = current.iter().position(|v| v == value) {
                        current.remove(pos);
                    } else {
                        current.push(value.to_string());
                    }
                    *controlled.write() = current;
                } else {
                    let mut current = (self.multiple_values)();
                    if let Some(pos) = current.iter().position(|v| v == value) {
                        current.remove(pos);
                    } else {
                        current.push(value.to_string());
                    }
                    *self.multiple_values.write() = current;
                }
            }
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * AccordionItem
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
    #[props(default)]
    pub children: Element,

    /// Unique value for this accordion item
    pub value: String,

    /// Whether this item is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[props(optional)]
    pub class: Option<String>,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let context = use_context::<AccordionContext>();
    let is_disabled = context.disabled || props.disabled;

    let class_name = utils::cn(vec![Some("border-b"), props.class.as_deref()]);

    let is_open = context.is_open(&props.value);

    use_context_provider(|| AccordionItemContext {
        value: props.value.clone(),
        disabled: is_disabled,
        accordion_context: context.clone(),
    });

    rsx! {
        div {
            class: "{class_name}",
            "data-state": if is_open { "open" } else { "closed" },
            "data-orientation": "{context.orientation.as_str()}",
            {props.children}
        }
    }
}

#[derive(Clone)]
struct AccordionItemContext {
    value: String,
    disabled: bool,
    accordion_context: AccordionContext,
}

impl AccordionItemContext {
    fn is_open(&self) -> bool {
        self.accordion_context.is_open(&self.value)
    }
}

/* -------------------------------------------------------------------------------------------------
 * AccordionTrigger
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct AccordionTriggerProps {
    #[props(default)]
    pub children: Element,

    /// Additional CSS classes
    #[props(optional)]
    pub class: Option<String>,
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    let mut accordion_context = use_context::<AccordionContext>();
    let item_context = use_context::<AccordionItemContext>();

    let base_class = "flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180";

    let class_name = utils::cn(vec![Some(base_class), props.class.as_deref()]);

    let value = item_context.value.clone();

    rsx! {
        h3 {
            class: "flex",
            button {
                class: "{class_name}",
                "type": "button",
                "data-state": if item_context.is_open() { "open" } else { "closed" },
                "data-orientation": "{accordion_context.orientation.as_str()}",
                disabled: item_context.disabled,
                onclick: move |_| {
                    if !item_context.disabled {
                        accordion_context.toggle(&value);
                    }
                },

                {props.children}

                // Chevron Down Icon from Lucide
                svg {
                    class: "size-4 shrink-0 transition-transform duration-200",
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "24",
                    height: "24",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",

                    path { d: "m6 9 6 6 6-6" }
                }
            }
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * AccordionContent
 * -----------------------------------------------------------------------------------------------*/

#[derive(Props, Clone, PartialEq)]
pub struct AccordionContentProps {
    #[props(default)]
    pub children: Element,

    /// Additional CSS classes
    #[props(optional)]
    pub class: Option<String>,
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    let item_context = use_context::<AccordionItemContext>();
    let is_open = item_context.is_open();

    let base_class =
        "overflow-hidden text-sm transition-all duration-300 ease-[cubic-bezier(0.87,0,0.13,1)]";
    let class_name = utils::cn(vec![Some(base_class), props.class.as_deref()]);

    let style = if is_open {
        "max-height: 1000px; opacity: 1;"
    } else {
        "max-height: 0; opacity: 0;"
    };

    rsx! {
        div {
            class: "{class_name}",
            "data-state": if is_open { "open" } else { "closed" },
            role: "region",
            style: "{style}",

            div {
                class: "pb-4 pt-0",
                {props.children}
            }
        }
    }
}
