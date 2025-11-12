use crate::components::portal::Portal;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

/* -------------------------------------------------------------------------------------------------
 * Dialog Context
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct DialogContext {
    open: Signal<bool>,
    modal: bool,
    content_id: String,
    title_id: String,
    description_id: String,
    on_open_change: Option<EventHandler<bool>>,
}

/* -------------------------------------------------------------------------------------------------
 * Dialog (Root)
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Dialog(
    /// Controlled open state
    open: Option<bool>,
    /// Whether the dialog is open by default (uncontrolled)
    #[props(default = false)]
    default_open: bool,
    /// Callback when open state changes (for controlled mode)
    on_open_change: Option<EventHandler<bool>>,
    /// Whether the dialog is modal (blocks interaction with content behind it)
    #[props(default = true)]
    modal: bool,
    children: Element,
) -> Element {
    // Controlled vs Uncontrolled state
    let mut internal_open = use_signal(|| open.unwrap_or(default_open));

    // Sync controlled state if provided
    use_effect(move || {
        if let Some(controlled_open) = open {
            internal_open.set(controlled_open);
        }
    });

    // Generate unique IDs for accessibility
    let content_id = use_memo(move || {
        format!(
            "dialog-content-{}",
            (js_sys::Math::random() * 1_000_000_000.0) as u64
        )
    });
    let title_id = use_memo(move || {
        format!(
            "dialog-title-{}",
            (js_sys::Math::random() * 1_000_000_000.0) as u64
        )
    });
    let description_id = use_memo(move || {
        format!(
            "dialog-description-{}",
            (js_sys::Math::random() * 1_000_000_000.0) as u64
        )
    });

    let context = DialogContext {
        open: internal_open,
        modal,
        content_id: content_id().to_string(),
        title_id: title_id().to_string(),
        description_id: description_id().to_string(),
        on_open_change,
    };

    use_context_provider(|| context);

    rsx! { {children} }
}

/* -------------------------------------------------------------------------------------------------
 * DialogTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogTrigger(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<DialogContext>();
    let mut open = context.open;

    let onclick = move |_event: Event<MouseData>| {
        let new_state = !open();
        open.set(new_state);

        // Call on_open_change callback if provided
        if let Some(handler) = &context.on_open_change {
            handler.call(new_state);
        }
    };

    rsx! {
        button {
            r#type: "button",
            "aria-haspopup": "dialog",
            "aria-expanded": if open() { "true" } else { "false" },
            "aria-controls": context.content_id,
            "data-state": if open() { "open" } else { "closed" },
            onclick: onclick,
            ..attributes,
            {children}
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogOverlay
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogOverlay(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let context = use_context::<DialogContext>();
    let open = context.open;

    if !context.modal {
        return rsx! { {children} };
    }

    if !open() {
        return rsx! {};
    }

    let combined_class = if class.is_empty() {
        "dialog-overlay".to_string()
    } else {
        format!("dialog-overlay {}", class)
    };

    rsx! {
        div {
            class: combined_class,
            "data-state": if open() { "open" } else { "closed" },
            style: "position: fixed; inset: 0; background-color: rgba(0, 0, 0, 0.5); z-index: 9998; pointer-events: auto;",
            ..attributes,
            {children}
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = String::new())] class: String,
    /// Container element selector for the portal (default: "body")
    #[props(default = "body".to_string())]
    container: String,
    /// Whether clicking outside should close the dialog
    #[props(default = true)]
    close_on_outside_click: bool,
    /// Whether pressing Escape should close the dialog
    #[props(default = true)]
    close_on_escape: bool,
    children: Element,
) -> Element {
    let context = use_context::<DialogContext>();
    let mut open = context.open;
    let content_id = context.content_id.clone();
    let is_modal = context.modal;

    if !open() {
        return rsx! {};
    }

    let combined_class = if class.is_empty() {
        "dialog-content".to_string()
    } else {
        format!("dialog-content {}", class)
    };

    // Clone context fields we need for the JSX
    let modal = context.modal;
    let content_id_for_jsx = context.content_id.clone();
    let title_id_for_jsx = context.title_id.clone();
    let description_id_for_jsx = context.description_id.clone();
    let on_open_change = context.on_open_change.clone();

    // Body scroll lock for modal dialogs (with layout shift prevention)
    use_effect(move || {
        if open() && is_modal {
            // Lock body scroll using JavaScript with scrollbar width compensation
            let lock_scroll_js = r#"
                (function() {
                    if (!document.body) return;
                    
                    // Calculate scrollbar width before hiding it
                    const scrollbarWidth = window.innerWidth - document.documentElement.clientWidth;
                    
                    // Store original values for restoration
                    window._originalOverflow = document.body.style.overflow;
                    window._originalPaddingRight = document.body.style.paddingRight;
                    
                    // Lock scroll and compensate for scrollbar width
                    document.body.style.overflow = 'hidden';
                    if (scrollbarWidth > 0) {
                        document.body.style.paddingRight = scrollbarWidth + 'px';
                    }
                })();
            "#;

            if let Ok(_) = js_sys::eval(lock_scroll_js) {
                // Scroll is locked with layout shift prevention
            }
        } else {
            // Unlock scroll and restore original padding
            let unlock_scroll_js = r#"
                (function() {
                    if (!document.body) return;
                    
                    // Restore original values
                    document.body.style.overflow = window._originalOverflow || '';
                    document.body.style.paddingRight = window._originalPaddingRight || '';
                    
                    // Clean up stored values
                    delete window._originalOverflow;
                    delete window._originalPaddingRight;
                })();
            "#;

            let _ = js_sys::eval(unlock_scroll_js);
        }
    });

    // Focus trap for modal dialogs
    use_effect(move || {
        if !open() || !is_modal {
            return;
        }

        let dialog_id = content_id.clone();

        // Set up focus trap using JavaScript
        let trap_js = format!(
            r#"
            (function() {{
                const dialog = document.getElementById('{}');
                if (!dialog) return;
                
                // Focus first focusable element
                const focusableElements = dialog.querySelectorAll(
                    'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
                );
                if (focusableElements.length > 0) {{
                    focusableElements[0].focus();
                }}
                
                // Set up focus trap
                const handleTab = (e) => {{
                    if (e.key !== 'Tab') return;
                    
                    const focusable = Array.from(focusableElements);
                    const firstFocusable = focusable[0];
                    const lastFocusable = focusable[focusable.length - 1];
                    
                    if (e.shiftKey) {{
                        if (document.activeElement === firstFocusable) {{
                            lastFocusable.focus();
                            e.preventDefault();
                        }}
                    }} else {{
                        if (document.activeElement === lastFocusable) {{
                            firstFocusable.focus();
                            e.preventDefault();
                        }}
                    }}
                }};
                
                dialog.addEventListener('keydown', handleTab);
                
                // Store cleanup function
                window._dialogFocusTrapCleanup = () => {{
                    dialog.removeEventListener('keydown', handleTab);
                }};
            }})();
            "#,
            dialog_id
        );

        let _ = js_sys::eval(&trap_js);
    });

    // Enhanced Escape key handler
    use_effect(move || {
        if !close_on_escape || !open() {
            return;
        }

        let escape_handler_js = format!(
            r#"
            (function() {{
                const handleEscape = (e) => {{
                    if (e.key === 'Escape') {{
                        e.preventDefault();
                        e.stopPropagation();
                        // This will be handled by the Dialog state
                    }}
                }};
                
                document.addEventListener('keydown', handleEscape);
                
                window._dialogEscapeCleanup = () => {{
                    document.removeEventListener('keydown', handleEscape);
                }};
            }})();
            "#
        );

        let _ = js_sys::eval(&escape_handler_js);

        // Also set up Dioxus event handler
        let ctx_for_handler = context.clone();
        let closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if e.key() == "Escape" {
                    e.prevent_default();
                    e.stop_propagation();
                    open.set(false);

                    // Call on_open_change callback if provided
                    if let Some(handler) = &ctx_for_handler.on_open_change {
                        handler.call(false);
                    }
                }
            }) as Box<dyn FnMut(_)>);

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let _ = document
                    .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            }
        }

        closure.forget();
    });

    // Handle backdrop click
    let on_backdrop_click = move |_event: Event<MouseData>| {
        if close_on_outside_click {
            open.set(false);

            // Call on_open_change callback if provided
            if let Some(handler) = &on_open_change {
                handler.call(false);
            }
        }
    };

    rsx! {
        Portal {
            container,
            // Backdrop overlay
            if modal {
                div {
                    class: "dialog-backdrop",
                    style: "position: fixed; inset: 0; z-index: 9998;",
                    onclick: on_backdrop_click,
                }
            }
            // Dialog content
            div {
                role: "dialog",
                id: content_id_for_jsx,
                "aria-labelledby": title_id_for_jsx,
                "aria-describedby": description_id_for_jsx,
                "aria-modal": if modal { "true" } else { "false" },
                "data-state": if open() { "open" } else { "closed" },
                class: combined_class,
                style: "position: fixed; z-index: 9999;",
                tabindex: "-1",
                ..attributes,
                {children}
            }
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogTitle
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogTitle(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let context = use_context::<DialogContext>();

    let combined_class = if class.is_empty() {
        "dialog-title".to_string()
    } else {
        format!("dialog-title {}", class)
    };

    rsx! {
        h2 {
            id: context.title_id,
            class: combined_class,
            ..attributes,
            {children}
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogDescription
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogDescription(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let context = use_context::<DialogContext>();

    let combined_class = if class.is_empty() {
        "dialog-description".to_string()
    } else {
        format!("dialog-description {}", class)
    };

    rsx! {
        p {
            id: context.description_id,
            class: combined_class,
            ..attributes,
            {children}
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogClose
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let context = use_context::<DialogContext>();
    let mut open = context.open;

    let onclick = move |_event: Event<MouseData>| {
        open.set(false);

        // Call on_open_change callback if provided
        if let Some(handler) = &context.on_open_change {
            handler.call(false);
        }
    };

    rsx! {
        button {
            r#type: "button",
            onclick: onclick,
            ..attributes,
            {children}
        }
    }
}
