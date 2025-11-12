use dioxus::prelude::*;

/// Props for the Portal component
#[derive(Props, Clone, PartialEq)]
pub struct PortalProps {
    /// The content to be portaled
    children: Element,

    /// Optional container selector where the content should be appended
    /// If None, defaults to document.body
    /// Use CSS selectors like "body", "#root", ".container"
    #[props(default = "main".to_string())]
    container: String,

    /// Optional class name for the portal wrapper div
    #[props(default = None)]
    class: Option<String>,

    /// Optional id for the portal wrapper div
    #[props(default = None)]
    id: Option<String>,
}

/// Portal component that renders children into a different part of the DOM tree
///
/// This is equivalent to ReactDOM.createPortal in React.
/// It allows you to render components outside of the parent component's DOM hierarchy.
///
/// # Example
/// ```rust
/// rsx! {
///     div {
///         "This is in the normal hierarchy"
///         Portal {
///             container: "body",
///             class: "modal-overlay",
///             div {
///                 "This will be rendered directly in document.body"
///             }
///         }
///     }
/// }
/// ```
///
/// # Implementation Notes
/// This implementation uses inline JavaScript in an onmounted event to physically move
/// the portal element to the specified container in the DOM, matching React's createPortal behavior.
#[component]
pub fn Portal(props: PortalProps) -> Element {
    let mut mounted = use_signal(|| false);

    // Generate a unique ID using JavaScript's Math.random() (WASM-compatible)
    let portal_id = format!(
        "portal-{}",
        (js_sys::Math::random() * 1_000_000_000.0) as u64
    );

    // Set mounted after first render
    use_effect(move || {
        mounted.set(true);
    });

    // Only render when mounted (client-side only)
    if !mounted() {
        return rsx! {};
    }

    let container = props.container.clone();
    let portal_id_clone = portal_id.clone();

    rsx! {
        div {
            class: props.class.clone(),
            id: props.id.clone(),
            style: "position: fixed; z-index: 9999; inset: 0;",
            "data-portal-id": portal_id.clone(),
            "data-portal-container": props.container.clone(),

            // Move portal to target container when mounted
            onmounted: move |_| {
                let script = format!(
                    r#"
                    setTimeout(function() {{
                        const portalElement = document.querySelector('[data-portal-id="{}"]');
                        const targetContainer = document.querySelector('#{}');
                        
                        if (portalElement && targetContainer) {{
                            targetContainer.appendChild(portalElement);
                        }}
                    }}, 50);
                    "#,
                    portal_id_clone, container
                );

                let _ = js_sys::Function::new_no_args(&script).call0(&wasm_bindgen::JsValue::NULL);
            },

            {props.children}
        }
    }
}
