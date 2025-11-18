use dioxus::prelude::*;
use dioxus_components::*;

#[component]
pub fn CheckboxDemo() -> Element {
    rsx! {
        div {
            class: "space-y-8",

            h1 { class: "text-4xl font-bold tracking-tight", "Checkbox" }
            p { class: "text-lg text-muted-foreground",
                "Three-state checkbox (checked/unchecked/indeterminate) with full accessibility."
            }

            // Basic Checkboxes
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Basic Checkboxes" }
                div {
                    class: "space-y-4 p-6 border rounded-lg",
                    div {
                        class: "flex items-center gap-2",
                        Checkbox {
                            default_checked: CheckedState::Unchecked,
                            id: Some("check1".to_string()),
                            CheckboxIndicator {}
                        }
                        CheckboxLabel {
                            for_id: Some("check1".to_string()),
                            "Unchecked"
                        }
                    }
                    div {
                        class: "flex items-center gap-2",
                        Checkbox {
                            default_checked: CheckedState::Checked,
                            id: Some("check2".to_string()),
                            CheckboxIndicator {}
                        }
                        CheckboxLabel {
                            for_id: Some("check2".to_string()),
                            "Checked"
                        }
                    }
                    div {
                        class: "flex items-center gap-2",
                        Checkbox {
                            default_checked: CheckedState::Indeterminate,
                            id: Some("check3".to_string()),
                            CheckboxIndicator {}
                        }
                        CheckboxLabel {
                            for_id: Some("check3".to_string()),
                            "Indeterminate"
                        }
                    }
                }
            }

            // Disabled States
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Disabled States" }
                div {
                    class: "space-y-4 p-6 border rounded-lg",
                    div {
                        class: "flex items-center gap-2",
                        Checkbox {
                            default_checked: CheckedState::Checked,
                            disabled: true,
                            id: Some("disabled1".to_string()),
                            CheckboxIndicator {}
                        }
                        CheckboxLabel {
                            for_id: Some("disabled1".to_string()),
                            class: Some("opacity-50".to_string()),
                            "Disabled checked"
                        }
                    }
                    div {
                        class: "flex items-center gap-2",
                        Checkbox {
                            default_checked: CheckedState::Unchecked,
                            disabled: true,
                            id: Some("disabled2".to_string()),
                            CheckboxIndicator {}
                        }
                        CheckboxLabel {
                            for_id: Some("disabled2".to_string()),
                            class: Some("opacity-50".to_string()),
                            "Disabled unchecked"
                        }
                    }
                }
            }
        }
    }
}
