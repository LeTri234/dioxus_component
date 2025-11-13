# Dioxus Components Documentation

A comprehensive guide to using the Dioxus 0.7 component library. All components are built with Tailwind CSS v4 and follow Radix UI design patterns.

## Table of Contents

1. [Accordion](#accordion)
2. [Avatar](#avatar)
3. [Badge](#badge)
4. [Button](#button)
5. [Card](#card)
6. [Checkbox](#checkbox)
7. [Dialog](#dialog)
8. [Empty](#empty)
9. [Portal](#portal)
10. [Spinner](#spinner)
11. [Tooltip](#tooltip)

---

## Accordion

A vertically stacked set of interactive headings that each reveal an associated section of content. Supports both single and multiple open items.

### Basic Usage

```rust
use dioxus::prelude::*;
use crate::components::{Accordion, AccordionItem, AccordionTrigger, AccordionContent, AccordionType};

#[component]
fn App() -> Element {
    rsx! {
        Accordion {
            accordion_type: AccordionType::Single { collapsible: true },
            AccordionItem {
                value: "item-1",
                AccordionTrigger { "Is it accessible?" }
                AccordionContent {
                    "Yes. It adheres to the WAI-ARIA design pattern."
                }
            }
            AccordionItem {
                value: "item-2",
                AccordionTrigger { "Is it styled?" }
                AccordionContent {
                    "Yes. It comes with default styles that match the other components."
                }
            }
        }
    }
}
```

### Props

#### Accordion

| Prop             | Type                          | Default                         | Description                                           |
| ---------------- | ----------------------------- | ------------------------------- | ----------------------------------------------------- |
| `accordion_type` | `AccordionType`               | `Single { collapsible: false }` | Controls whether single or multiple items can be open |
| `default_value`  | `Option<String>`              | `None`                          | Initial value for single accordion                    |
| `default_values` | `Option<Vec<String>>`         | `None`                          | Initial values for multiple accordion                 |
| `value`          | `Option<Signal<String>>`      | `None`                          | Controlled value for single accordion                 |
| `values`         | `Option<Signal<Vec<String>>>` | `None`                          | Controlled values for multiple accordion              |
| `disabled`       | `bool`                        | `false`                         | Disables all accordion items                          |
| `orientation`    | `AccordionOrientation`        | `Vertical`                      | Layout orientation                                    |
| `class`          | `Option<String>`              | `None`                          | Additional CSS classes                                |

#### AccordionItem

| Prop       | Type             | Default  | Description                    |
| ---------- | ---------------- | -------- | ------------------------------ |
| `value`    | `String`         | Required | Unique identifier for the item |
| `disabled` | `bool`           | `false`  | Disables this specific item    |
| `class`    | `Option<String>` | `None`   | Additional CSS classes         |

#### AccordionTrigger

| Prop    | Type             | Default | Description            |
| ------- | ---------------- | ------- | ---------------------- |
| `class` | `Option<String>` | `None`  | Additional CSS classes |

#### AccordionContent

| Prop    | Type             | Default | Description            |
| ------- | ---------------- | ------- | ---------------------- |
| `class` | `Option<String>` | `None`  | Additional CSS classes |

### Variants

```rust
// Single accordion (only one item open at a time)
Accordion {
    accordion_type: AccordionType::Single { collapsible: false },
    // Items here...
}

// Single accordion with collapse support
Accordion {
    accordion_type: AccordionType::Single { collapsible: true },
    // Items here...
}

// Multiple accordion (many items can be open)
Accordion {
    accordion_type: AccordionType::Multiple,
    // Items here...
}
```

### Controlled State

```rust
let mut selected = use_signal(|| "item-1".to_string());

rsx! {
    Accordion {
        accordion_type: AccordionType::Single { collapsible: true },
        value: selected,
        // Items here...
    }
}
```

---

## Avatar

Displays a user's avatar image or initials as a fallback.

### Basic Usage

```rust
use dioxus::prelude::*;
use crate::components::{Avatar, AvatarImage, AvatarFallback};

#[component]
fn App() -> Element {
    rsx! {
        Avatar {
            AvatarImage {
                src: "https://github.com/shadcn.png",
                alt: "User Avatar"
            }
            AvatarFallback {
                "CN"
            }
        }
    }
}
```

### Props

#### Avatar

| Prop    | Type             | Default | Description            |
| ------- | ---------------- | ------- | ---------------------- |
| `class` | `Option<String>` | `None`  | Additional CSS classes |

#### AvatarImage

| Prop                       | Type                   | Default  | Description                          |
| -------------------------- | ---------------------- | -------- | ------------------------------------ |
| `src`                      | `String`               | Required | Image URL                            |
| `alt`                      | `Option<String>`       | `None`   | Alt text for accessibility           |
| `class`                    | `Option<String>`       | `None`   | Additional CSS classes               |
| `referrer_policy`          | `Option<String>`       | `None`   | Referrer policy for image request    |
| `cross_origin`             | `Option<String>`       | `None`   | Cross-origin policy                  |
| `on_loading_status_change` | `Option<EventHandler>` | `None`   | Callback when loading status changes |

#### AvatarFallback

| Prop    | Type             | Default | Description            |
| ------- | ---------------- | ------- | ---------------------- |
| `class` | `Option<String>` | `None`  | Additional CSS classes |

### Image Loading States

The avatar component tracks image loading with states:

- `Idle` - Initial state
- `Loading` - Image is being loaded
- `Loaded` - Image successfully loaded
- `Error` - Image failed to load (shows fallback)

```rust
Avatar {
    AvatarImage {
        src: "https://example.com/avatar.jpg",
        alt: "Avatar",
        on_loading_status_change: move |status| {
            match status {
                ImageLoadingStatus::Loaded => println!("Image loaded!"),
                ImageLoadingStatus::Error => println!("Image failed to load"),
                _ => {}
            }
        }
    }
    AvatarFallback {
        "AB"
    }
}
```

---

## Badge

A flexible badge component with multiple styling variants.

### Basic Usage

```rust
use dioxus::prelude::*;
use crate::components::{Badge, BadgeVariant};

#[component]
fn App() -> Element {
    rsx! {
        Badge { "Default" }
        Badge {
            variant: BadgeVariant::Secondary,
            "Secondary"
        }
        Badge {
            variant: BadgeVariant::Destructive,
            "Destructive"
        }
        Badge {
            variant: BadgeVariant::Outline,
            "Outline"
        }
    }
}
```

### Props

| Prop      | Type                   | Default   | Description                                           |
| --------- | ---------------------- | --------- | ----------------------------------------------------- |
| `variant` | `BadgeVariant`         | `Default` | Badge color and style                                 |
| `class`   | `Option<String>`       | `None`    | Additional CSS classes                                |
| `as_`     | `String`               | `"span"`  | HTML element to render (`span`, `a`, `button`, `div`) |
| `href`    | `Option<String>`       | `None`    | URL for link badges                                   |
| `onclick` | `Option<EventHandler>` | `None`    | Click handler                                         |

### Variants

```rust
// Default - Primary color
Badge { "Default" }

// Secondary - Gray background
Badge {
    variant: BadgeVariant::Secondary,
    "Secondary"
}

// Destructive - Red background
Badge {
    variant: BadgeVariant::Destructive,
    "Destructive"
}

// Outline - Border only
Badge {
    variant: BadgeVariant::Outline,
    "Outline"
}
```

### As Different Elements

```rust
// As link
Badge {
    as_: "a".to_string(),
    href: "https://example.com",
    "Link Badge"
}

// As button
Badge {
    as_: "button".to_string(),
    onclick: move |_| {
        println!("Badge clicked!");
    },
    "Clickable Badge"
}
```

---

## Button

A flexible button component with multiple variants and sizes.

### Basic Usage

```rust
use dioxus::prelude::*;
use crate::components::{Button, ButtonVariant, ButtonSize};

#[component]
fn App() -> Element {
    rsx! {
        Button { "Click me" }
        Button {
            variant: ButtonVariant::Destructive,
            "Delete"
        }
        Button {
            variant: ButtonVariant::Outline,
            "Cancel"
        }
    }
}
```

### Props

| Prop           | Type                    | Default    | Description                         |
| -------------- | ----------------------- | ---------- | ----------------------------------- |
| `variant`      | `Option<ButtonVariant>` | `Default`  | Button style variant                |
| `size`         | `Option<ButtonSize>`    | `Default`  | Button size                         |
| `class`        | `Option<String>`        | `None`     | Additional CSS classes              |
| `as_`          | `Option<String>`        | `"button"` | HTML element to render              |
| `disabled`     | `Option<bool>`          | `false`    | Disable the button                  |
| `href`         | `Option<String>`        | `None`     | URL for link buttons                |
| `onclick`      | `Option<EventHandler>`  | `None`     | Click handler                       |
| `aria_invalid` | `Option<bool>`          | `false`    | Mark as invalid for form validation |

### Button Variants

```rust
// Default - Primary color (recommended)
Button { "Default" }

// Destructive - Red, for delete/dangerous actions
Button {
    variant: ButtonVariant::Destructive,
    "Delete"
}

// Outline - Bordered, low emphasis
Button {
    variant: ButtonVariant::Outline,
    "Cancel"
}

// Secondary - Gray, secondary action
Button {
    variant: ButtonVariant::Secondary,
    "Secondary"
}

// Ghost - No background, text only
Button {
    variant: ButtonVariant::Ghost,
    "Ghost"
}

// Link - Underlined text
Button {
    variant: ButtonVariant::Link,
    "Link"
}
```

### Button Sizes

```rust
// Default - Medium (9 height)
Button { "Default" }

// Small - Compact (8 height)
Button {
    size: ButtonSize::Sm,
    "Small"
}

// Large - Spacious (10 height)
Button {
    size: ButtonSize::Lg,
    "Large"
}

// Icon - Square for icon buttons (24px)
Button {
    size: ButtonSize::Icon,
    // SVG icon here
}

// IconSm - Small square icon (20px)
Button {
    size: ButtonSize::IconSm,
    // SVG icon here
}

// IconLg - Large square icon (40px)
Button {
    size: ButtonSize::IconLg,
    // SVG icon here
}
```

### Link Button

```rust
Button {
    as_: "a".to_string(),
    href: "https://example.com",
    "Go to Example"
}
```

### Disabled State

```rust
Button {
    disabled: Some(true),
    "Disabled"
}
```

### With Icon

```rust
Button {
    class: "gap-2",
    svg {
        // Icon SVG
    }
    "Button with Icon"
}
```

---

## Card

A flexible card container component with optional header, content, and footer sections. Perfect for displaying grouped information, forms, and content blocks.

### Basic Usage

```rust
use dioxus::prelude::*;
use crate::components::{
    Card, CardHeader, CardTitle, CardDescription,
    CardContent, CardFooter, CardAction
};

#[component]
fn App() -> Element {
    rsx! {
        Card {
            class: "w-[350px]",
            CardHeader {
                CardTitle { "Notifications" }
                CardDescription { "You have 3 unread messages." }
            }
            CardContent {
                p { "Check your inbox for updates." }
            }
            CardFooter {
                Button { "Mark all as read" }
            }
        }
    }
}
```

### Card with Action

Add action buttons to the header:

```rust
Card {
    CardHeader {
        CardTitle { "Team Members" }
        CardDescription { "Invite and manage your team." }
        CardAction {
            Button {
                variant: ButtonVariant::Ghost,
                "+"
            }
        }
    }
    CardContent {
        // Team member list
    }
}
```

### Props

#### Card

| Prop       | Type             | Default  | Description            |
| ---------- | ---------------- | -------- | ---------------------- |
| `class`    | `Option<String>` | `None`   | Additional CSS classes |
| `children` | `Element`        | Required | Card content           |

#### CardHeader

| Prop       | Type             | Default  | Description                                 |
| ---------- | ---------------- | -------- | ------------------------------------------- |
| `class`    | `Option<String>` | `None`   | Additional CSS classes                      |
| `children` | `Element`        | Required | Header content (title, description, action) |

#### CardTitle

| Prop       | Type             | Default  | Description            |
| ---------- | ---------------- | -------- | ---------------------- |
| `class`    | `Option<String>` | `None`   | Additional CSS classes |
| `children` | `Element`        | Required | Title text             |

#### CardDescription

| Prop       | Type             | Default  | Description            |
| ---------- | ---------------- | -------- | ---------------------- |
| `class`    | `Option<String>` | `None`   | Additional CSS classes |
| `children` | `Element`        | Required | Description text       |

#### CardAction

| Prop       | Type             | Default  | Description                |
| ---------- | ---------------- | -------- | -------------------------- |
| `class`    | `Option<String>` | `None`   | Additional CSS classes     |
| `children` | `Element`        | Required | Action buttons or controls |

#### CardContent

| Prop       | Type             | Default  | Description            |
| ---------- | ---------------- | -------- | ---------------------- |
| `class`    | `Option<String>` | `None`   | Additional CSS classes |
| `children` | `Element`        | Required | Main content           |

#### CardFooter

| Prop       | Type             | Default  | Description            |
| ---------- | ---------------- | -------- | ---------------------- |
| `class`    | `Option<String>` | `None`   | Additional CSS classes |
| `children` | `Element`        | Required | Footer content         |

### Card Examples

#### Stats Card

```rust
Card {
    CardHeader {
        CardTitle { "Total Revenue" }
        CardDescription { "January - December 2024" }
    }
    CardContent {
        p { class: "text-3xl font-bold", "$45,231.89" }
        p { class: "text-sm text-muted-foreground",
            "+20.1% from last year"
        }
    }
}
```

#### Form Card

```rust
Card {
    CardHeader {
        CardTitle { "Create Account" }
        CardDescription { "Enter your details to get started." }
    }
    CardContent {
        div {
            class: "space-y-4",
            // Form fields
        }
    }
    CardFooter {
        class: "border-t",
        Button { "Create Account" }
    }
}
```

#### Pricing Card

```rust
Card {
    CardHeader {
        CardTitle { "Pro Plan" }
        CardDescription { "For professional use" }
    }
    CardContent {
        div {
            p { class: "text-4xl font-bold", "$29" }
            p { class: "text-sm text-muted-foreground", "per month" }
        }
        ul {
            class: "space-y-2",
            li { "✓ Unlimited projects" }
            li { "✓ Priority support" }
            li { "✓ Advanced analytics" }
        }
    }
    CardFooter {
        Button { "Subscribe" }
    }
}
```

### Features

- ✅ Flexible layout with optional sections
- ✅ Header with title, description, and action support
- ✅ Automatic grid layout for header actions
- ✅ Customizable through Tailwind classes
- ✅ Consistent spacing and styling
- ✅ Border support for header and footer
- ✅ Responsive design patterns
- ✅ Shadow and rounded corners

### Best Practices

1. **Use semantic structure**: Include CardHeader, CardContent, and CardFooter for clear organization
2. **Consistent widths**: Apply width classes to Card for consistent sizing
3. **Action placement**: Use CardAction for buttons in the header
4. **Border separators**: Add `border-t` or `border-b` classes to CardFooter/CardHeader for visual separation
5. **Spacing**: The Card automatically handles internal spacing with `gap-6`
6. **Content padding**: CardContent and CardFooter include horizontal padding automatically
7. **Responsive layouts**: Use grid classes on parent containers for responsive card layouts

### Styling

The Card component uses these base styles:

- Background: `bg-card`
- Text color: `text-card-foreground`
- Border: `border` with rounded corners `rounded-xl`
- Shadow: `shadow-sm`
- Internal spacing: `gap-6` between sections
- Padding: `py-6` vertical, `px-6` on content/footer

---

## Checkbox

A three-state checkbox component with full Radix UI architecture parity. Supports checked, unchecked, and indeterminate states with comprehensive accessibility features.

### Basic Usage

```rust
use dioxus::prelude::*;
use crate::components::{Checkbox, CheckboxIndicator, CheckboxLabel, CheckedState};

#[component]
fn App() -> Element {
    rsx! {
        div {
            class: "flex items-center gap-2",
            Checkbox {
                default_checked: CheckedState::Unchecked,
                id: Some("terms".to_string()),
                CheckboxIndicator {}
            }
            CheckboxLabel {
                for_id: Some("terms".to_string()),
                "I accept the terms and conditions"
            }
        }
    }
}
```

### Provider Pattern (Radix UI Style)

```rust
use dioxus::prelude::*;
use crate::components::{CheckboxProvider, CheckboxTrigger, CheckboxIndicator, CheckedState};

#[component]
fn App() -> Element {
    rsx! {
        CheckboxProvider {
            default_checked: CheckedState::Unchecked,
            onchange: move |state| println!("Changed to: {:?}", state),
            CheckboxTrigger {
                CheckboxIndicator {}
            }
        }
    }
}
```

### Checkbox Props

#### Checkbox (Convenience Component)

| Prop              | Type                                 | Default     | Description                     |
| ----------------- | ------------------------------------ | ----------- | ------------------------------- |
| `checked`         | `Option<CheckedState>`               | `None`      | Controlled checked state        |
| `default_checked` | `CheckedState`                       | `Unchecked` | Initial state when uncontrolled |
| `onchange`        | `Option<EventHandler<CheckedState>>` | `None`      | Callback when state changes     |
| `disabled`        | `bool`                               | `false`     | Disable the checkbox            |
| `required`        | `bool`                               | `false`     | Mark as required field          |
| `name`            | `Option<String>`                     | `None`      | Form field name                 |
| `form`            | `Option<String>`                     | `None`      | Form ID to associate with       |
| `id`              | `Option<String>`                     | `None`      | HTML id attribute               |
| `value`           | `String`                             | `"on"`      | Form submission value           |
| `class`           | `Option<String>`                     | `None`      | Additional CSS classes          |

#### CheckboxProvider

| Prop              | Type                                 | Default     | Description                     |
| ----------------- | ------------------------------------ | ----------- | ------------------------------- |
| `checked`         | `Option<CheckedState>`               | `None`      | Controlled checked state        |
| `default_checked` | `CheckedState`                       | `Unchecked` | Initial state when uncontrolled |
| `onchange`        | `Option<EventHandler<CheckedState>>` | `None`      | Callback when state changes     |
| `disabled`        | `bool`                               | `false`     | Disable the checkbox            |
| `required`        | `bool`                               | `false`     | Mark as required field          |
| `name`            | `Option<String>`                     | `None`      | Form field name                 |
| `form`            | `Option<String>`                     | `None`      | Form ID to associate with       |
| `value`           | `String`                             | `"on"`      | Form submission value           |

#### CheckboxTrigger

| Prop        | Type                                  | Default | Description            |
| ----------- | ------------------------------------- | ------- | ---------------------- |
| `class`     | `Option<String>`                      | `None`  | Additional CSS classes |
| `id`        | `Option<String>`                      | `None`  | HTML id attribute      |
| `onclick`   | `Option<EventHandler<MouseEvent>>`    | `None`  | Custom click handler   |
| `onkeydown` | `Option<EventHandler<KeyboardEvent>>` | `None`  | Custom keydown handler |

#### CheckboxIndicator

| Prop          | Type             | Default | Description                 |
| ------------- | ---------------- | ------- | --------------------------- |
| `class`       | `Option<String>` | `None`  | Additional CSS classes      |
| `force_mount` | `bool`           | `false` | Keep mounted for animations |

#### CheckboxLabel

| Prop     | Type             | Default | Description               |
| -------- | ---------------- | ------- | ------------------------- |
| `for_id` | `Option<String>` | `None`  | ID of associated checkbox |
| `class`  | `Option<String>` | `None`  | Additional CSS classes    |

### Checked States

The checkbox supports three states via the `CheckedState` enum:

```rust
// Unchecked - Empty checkbox
Checkbox {
    default_checked: CheckedState::Unchecked,
    CheckboxIndicator {}
}

// Checked - Shows checkmark (✓)
Checkbox {
    default_checked: CheckedState::Checked,
    CheckboxIndicator {}
}

// Indeterminate - Shows dash (−) for mixed/partial selection
Checkbox {
    default_checked: CheckedState::Indeterminate,
    CheckboxIndicator {}
}
```

### Controlled State

```rust
let mut checked = use_signal(|| CheckedState::Unchecked);

rsx! {
    div {
        Checkbox {
            checked: Some(checked()),
            onchange: move |state| *checked.write() = state,
            CheckboxIndicator {}
        }
        p { "State: {checked:?}" }
    }
}
```

### Provider Pattern (Advanced)

For complex scenarios requiring the full Radix UI architecture:

```rust
CheckboxProvider {
    default_checked: CheckedState::Unchecked,
    name: Some("newsletter".to_string()),
    onchange: move |state| {
        println!("Checkbox changed to: {:?}", state);
    },
    CheckboxTrigger {
        id: Some("newsletter".to_string()),
        CheckboxIndicator {}
    }
}
CheckboxLabel {
    for_id: Some("newsletter".to_string()),
    "Subscribe to newsletter"
}
```

### Form Integration

```rust
form {
    id: "signup",
    onsubmit: move |evt| {
        evt.prevent_default();
        // Handle form submission
    },

    div {
        class: "flex items-center gap-2",
        Checkbox {
            name: Some("terms".to_string()),
            form: Some("signup".to_string()),
            required: true,
            CheckboxIndicator {}
        }
        CheckboxLabel {
            for_id: Some("terms".to_string()),
            "I accept the terms *"
        }
    }

    button {
        r#type: "submit",
        "Submit"
    }
}
```

### Disabled State

```rust
Checkbox {
    default_checked: CheckedState::Checked,
    disabled: true,
    CheckboxIndicator {}
}
```

### Select All Pattern (Indeterminate)

```rust
let mut all = use_signal(|| CheckedState::Unchecked);
let mut item1 = use_signal(|| CheckedState::Unchecked);
let mut item2 = use_signal(|| CheckedState::Unchecked);

// Update parent based on children
use_effect(move || {
    let i1 = item1();
    let i2 = item2();

    if i1 == CheckedState::Checked && i2 == CheckedState::Checked {
        all.set(CheckedState::Checked);
    } else if i1 == CheckedState::Unchecked && i2 == CheckedState::Unchecked {
        all.set(CheckedState::Unchecked);
    } else {
        all.set(CheckedState::Indeterminate);  // Mixed state
    }
});

rsx! {
    div {
        // Parent "Select All" checkbox
        Checkbox {
            checked: Some(all()),
            onchange: move |state| {
                all.set(state);
                item1.set(state);
                item2.set(state);
            },
            CheckboxIndicator {}
        }
        "Select All"

        // Child checkboxes
        div {
            class: "ml-4 space-y-2",
            div {
                Checkbox {
                    checked: Some(item1()),
                    onchange: move |state| item1.set(state),
                    CheckboxIndicator {}
                }
                "Item 1"
            }
            div {
                Checkbox {
                    checked: Some(item2()),
                    onchange: move |state| item2.set(state),
                    CheckboxIndicator {}
                }
                "Item 2"
            }
        }
    }
}
```

### Custom Indicator

Replace the default checkmark with custom content:

```rust
Checkbox {
    default_checked: CheckedState::Unchecked,
    CheckboxIndicator {
        // Custom emoji or text instead of SVG
        "✅"
    }
}
```

### Accessibility

The checkbox component includes full WAI-ARIA support:

- **role="checkbox"** - Proper semantic role
- **aria-checked** - States: `"true"`, `"false"`, or `"mixed"` (indeterminate)
- **aria-required** - Marks required fields
- **Keyboard Navigation**:
  - **Tab** - Focus/unfocus the checkbox
  - **Space** - Toggle checked state
  - **Enter** - Prevented (per WAI-ARIA spec)

```rust
// Accessible checkbox with label
Checkbox {
    id: Some("agree".to_string()),
    required: true,
    CheckboxIndicator {}
}
CheckboxLabel {
    for_id: Some("agree".to_string()),
    "I agree to the terms (required)"
}
```

### Component Architecture

The checkbox follows Radix UI's composition pattern:

```
CheckboxProvider (state context)
└── CheckboxTrigger (button[role="checkbox"])
    ├── CheckboxIndicator (visual indicator)
    └── CheckboxBubbleInput (hidden form input)
```

**Components:**

- **CheckboxProvider** - Manages state and provides context
- **CheckboxTrigger** - Interactive button element
- **CheckboxIndicator** - Shows checkmark or dash icon
- **CheckboxBubbleInput** - Hidden input for form submission (automatic)
- **Checkbox** - Convenience wrapper (Provider + Trigger)
- **CheckboxLabel** - Accessible label helper

### Advanced Features

**Form Reset Support:**
Native HTML form reset automatically restores the checkbox to its `default_checked` state.

**Event Composition:**
Custom event handlers can be combined with built-in behavior:

```rust
CheckboxTrigger {
    onclick: move |evt| {
        println!("Custom click handler");
        // Built-in toggle still works
    },
    onkeydown: move |evt| {
        println!("Key pressed: {:?}", evt.key());
    },
    CheckboxIndicator {}
}
```

**Force Mount:**
Keep the indicator mounted even when unchecked (useful for animations):

```rust
CheckboxIndicator {
    force_mount: true,
    // Indicator always rendered, visibility controlled by CSS
}
```

---

## Dialog

A fully accessible modal dialog component that follows WAI-ARIA design patterns. Features include modal overlays, focus management, keyboard controls (Escape to close), and backdrop click handling.

### Basic Usage

```rust
use dioxus::prelude::*;
use dioxus_components::{
    Dialog, DialogTrigger, DialogOverlay, DialogContent,
    DialogTitle, DialogDescription, DialogClose
};

#[component]
fn App() -> Element {
    rsx! {
        Dialog {
            default_open: false,
            modal: true,
            DialogTrigger {
                class: "btn btn-primary",
                "Open Dialog"
            }
            DialogOverlay {}
            DialogContent {
                class: "dialog-content",
                DialogTitle {
                    class: "text-lg font-bold",
                    "Dialog Title"
                }
                DialogDescription {
                    class: "text-sm text-gray-600",
                    "This is a description of the dialog content."
                }
                div {
                    class: "space-y-4",
                    p { "Dialog body content goes here." }
                }
                div {
                    class: "flex justify-end gap-2",
                    DialogClose {
                        class: "btn btn-secondary",
                        "Cancel"
                    }
                    DialogClose {
                        class: "btn btn-primary",
                        "Confirm"
                    }
                }
            }
        }
    }
}
```

### Props

#### Dialog (Root Component)

| Prop             | Type                         | Default  | Description                                                |
| ---------------- | ---------------------------- | -------- | ---------------------------------------------------------- |
| `open`           | `Option<bool>`               | `None`   | Controlled open state (when provided, makes it controlled) |
| `default_open`   | `bool`                       | `false`  | Whether the dialog is initially open (uncontrolled mode)   |
| `on_open_change` | `Option<EventHandler<bool>>` | `None`   | Callback when open state changes                           |
| `modal`          | `bool`                       | `true`   | If true, shows overlay, traps focus, and locks body scroll |
| `children`       | `Element`                    | required | Child components (Trigger, Overlay, Content)               |

**Controlled vs Uncontrolled:**

- **Uncontrolled**: Only provide `default_open` (component manages state internally)
- **Controlled**: Provide `open` and `on_open_change` (you manage state externally)

#### DialogTrigger

| Prop         | Type             | Default  | Description              |
| ------------ | ---------------- | -------- | ------------------------ |
| `attributes` | `Vec<Attribute>` | `[]`     | Standard HTML attributes |
| `children`   | `Element`        | required | Button content           |

Automatically receives proper ARIA attributes:

- `aria-haspopup="dialog"`
- `aria-expanded` (true/false)
- `aria-controls` (links to content ID)
- `data-state` (open/closed)

#### DialogOverlay

| Prop         | Type             | Default  | Description              |
| ------------ | ---------------- | -------- | ------------------------ |
| `class`      | `String`         | `""`     | Additional CSS classes   |
| `attributes` | `Vec<Attribute>` | `[]`     | Standard HTML attributes |
| `children`   | `Element`        | optional | Custom overlay content   |

Default styling: Fixed position with dark semi-transparent background.

#### DialogContent

| Prop                     | Type             | Default  | Description                    |
| ------------------------ | ---------------- | -------- | ------------------------------ |
| `class`                  | `String`         | `""`     | Additional CSS classes         |
| `container`              | `String`         | `"body"` | CSS selector for portal target |
| `close_on_outside_click` | `bool`           | `true`   | Close when clicking backdrop   |
| `close_on_escape`        | `bool`           | `true`   | Close when pressing Escape key |
| `attributes`             | `Vec<Attribute>` | `[]`     | Standard HTML attributes       |
| `children`               | `Element`        | required | Dialog content                 |

Automatically receives ARIA attributes:

- `role="dialog"`
- `aria-modal` (true for modal dialogs)
- `aria-labelledby` (references title)
- `aria-describedby` (references description)

#### DialogTitle

| Prop         | Type             | Default  | Description              |
| ------------ | ---------------- | -------- | ------------------------ |
| `class`      | `String`         | `""`     | Additional CSS classes   |
| `attributes` | `Vec<Attribute>` | `[]`     | Standard HTML attributes |
| `children`   | `Element`        | required | Title text               |

Renders as `<h2>` element with unique ID for accessibility.

#### DialogDescription

| Prop         | Type             | Default  | Description              |
| ------------ | ---------------- | -------- | ------------------------ |
| `class`      | `String`         | `""`     | Additional CSS classes   |
| `attributes` | `Vec<Attribute>` | `[]`     | Standard HTML attributes |
| `children`   | `Element`        | required | Description text         |

Renders as `<p>` element with unique ID for accessibility.

#### DialogClose

| Prop         | Type             | Default  | Description              |
| ------------ | ---------------- | -------- | ------------------------ |
| `attributes` | `Vec<Attribute>` | `[]`     | Standard HTML attributes |
| `children`   | `Element`        | required | Button content           |

Closes the dialog when clicked.

### Alert Dialog Example

```rust
#[component]
fn DeleteConfirmation() -> Element {
    rsx! {
        Dialog {
            modal: true,
            DialogTrigger {
                class: "btn btn-destructive",
                "Delete Account"
            }
            DialogOverlay {}
            DialogContent {
                class: "max-w-md",
                DialogTitle {
                    class: "text-red-600 font-bold",
                    "⚠️ Are you absolutely sure?"
                }
                DialogDescription {
                    "This action cannot be undone. This will permanently "
                    "delete your account and remove your data from our servers."
                }
                div {
                    class: "flex justify-end gap-2 mt-6",
                    DialogClose {
                        class: "btn btn-secondary",
                        "Cancel"
                    }
                    DialogClose {
                        class: "btn btn-destructive",
                        "Delete Account"
                    }
                }
            }
        }
    }
}
```

### Form Dialog Example

```rust
#[component]
fn EditProfile() -> Element {
    rsx! {
        Dialog {
            DialogTrigger {
                class: "btn btn-primary",
                "Edit Profile"
            }
            DialogOverlay {}
            DialogContent {
                DialogTitle { "Edit Profile" }
                DialogDescription {
                    "Make changes to your profile here. Click save when you're done."
                }
                div {
                    class: "space-y-4 my-4",
                    div {
                        label {
                            class: "block text-sm font-medium mb-1",
                            "Name"
                        }
                        input {
                            class: "input",
                            r#type: "text",
                            placeholder: "Enter your name"
                        }
                    }
                    div {
                        label {
                            class: "block text-sm font-medium mb-1",
                            "Email"
                        }
                        input {
                            class: "input",
                            r#type: "email",
                            placeholder: "email@example.com"
                        }
                    }
                }
                div {
                    class: "flex justify-end gap-2",
                    DialogClose {
                        class: "btn btn-secondary",
                        "Cancel"
                    }
                    DialogClose {
                        class: "btn btn-primary",
                        "Save Changes"
                    }
                }
            }
        }
    }
}
```

### Features

- ✅ **Modal & Non-Modal Support**: Full modal with overlay and focus trap, or non-modal mode
- ✅ **Focus Trap**: Automatically traps focus within modal dialogs using Tab key handling
- ✅ **Body Scroll Lock**: Prevents scrolling outside modal when open (with layout shift prevention)
- ✅ **Controlled/Uncontrolled**: Can be controlled externally or manage state internally
- ✅ **Keyboard Controls**: Escape key to close with proper cleanup (configurable)
- ✅ **Click Outside**: Backdrop click to close (configurable)
- ✅ **Portal Rendering**: Uses Portal component to render outside parent DOM
- ✅ **Screen Reader Announcements**: Proper ARIA labeling with Title and Description
- ✅ **Accessibility**: Full WAI-ARIA dialog pattern implementation
- ✅ **Context API**: Shared state between all dialog components
- ✅ **Auto Focus**: Automatically focuses first focusable element on open
- ✅ **State Callbacks**: `on_open_change` for reacting to state changes

### Use Cases

1. **Confirmation Dialogs** - Delete confirmations, logout prompts
2. **Forms** - Edit profile, settings, data entry
3. **Alerts** - Important notifications requiring acknowledgment
4. **Media Viewers** - Image galleries, video players
5. **Complex Interactions** - Multi-step wizards, detailed views

### Implementation Details

The Dialog component uses:

- **Context API** to share state between trigger, content, and close buttons
- **Portal component** to render content at the document root
- **JavaScript event listeners** for Escape key handling
- **Signal** for reactive open/closed state
- **Memos** for generating unique ARIA IDs
- **Scrollbar width compensation** to prevent layout shift when locking scroll

**Scroll Lock Implementation:**
When a modal dialog opens, the component:

1. Calculates the scrollbar width (`window.innerWidth - document.documentElement.clientWidth`)
2. Sets `overflow: hidden` on the body
3. Adds `padding-right` equal to scrollbar width to prevent layout shift
4. Stores original values for restoration
5. Restores everything when dialog closes

This ensures the page content doesn't shift horizontally when the scrollbar disappears.

The component is split into composable parts following the Radix UI pattern:

- `Dialog` - Root provider with state management
- `DialogTrigger` - Button to open dialog
- `DialogOverlay` - Semi-transparent backdrop (modal only)
- `DialogContent` - Main container with Portal
- `DialogTitle` - Accessible title (required)
- `DialogDescription` - Accessible description (recommended)
- `DialogClose` - Button(s) to close dialog

### Best Practices

1. **Always include DialogTitle** for screen reader users
2. **Add DialogDescription** for context
3. **Use semantic button text** ("Delete", not "Yes")
4. **Consider close_on_outside_click** based on context
5. **Test keyboard navigation** (Tab, Escape)
6. **Style focus indicators** for accessibility
7. **Use appropriate variants** (destructive for delete actions)
8. **Keep content concise** in alert dialogs
9. **Validate forms** before allowing dialog close
10. **Provide clear actions** (primary vs secondary buttons)

### Accessibility

- ✅ **WAI-ARIA Dialog Pattern** compliant
- ✅ `role="dialog"` on content
- ✅ `aria-modal="true"` for modal dialogs
- ✅ `aria-labelledby` references title
- ✅ `aria-describedby` references description
- ✅ Keyboard support (Escape to close)
- ✅ Focus management (stays within dialog)
- ✅ Screen reader announcements
- ✅ Backdrop click handling

### Styling Recommendations

```css
/* Base dialog content styles */
.dialog-content {
  position: fixed;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  background: white;
  padding: 1.5rem;
  border-radius: 0.5rem;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  max-width: 500px;
  width: 90vw;
  max-height: 85vh;
  overflow-y: auto;
  z-index: 9999;
}

/* Overlay/backdrop */
.dialog-overlay {
  position: fixed;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: 9998;
  animation: fadeIn 150ms ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  .dialog-content {
    background: #1f2937;
    color: white;
  }
}
```

---

## Empty

An Empty state component for displaying "no content" scenarios. Perfect for search results, empty lists, inbox zero states, and other situations where you need to communicate the absence of data.

### Basic Usage

```rust
use dioxus::prelude::*;
use crate::components::{
    Empty, EmptyHeader, EmptyMedia, EmptyMediaVariant,
    EmptyTitle, EmptyDescription, EmptyContent
};

#[component]
fn App() -> Element {
    rsx! {
        Empty {
            EmptyHeader {
                EmptyMedia {
                    variant: EmptyMediaVariant::Icon,
                    // Icon SVG here
                }
                EmptyTitle { "No results found" }
                EmptyDescription { "Try adjusting your search" }
            }
            EmptyContent {
                Button { "Clear filters" }
            }
        }
    }
}
```

### Empty with Icon Variant

```rust
Empty {
    class: "min-h-[300px]",
    EmptyHeader {
        EmptyMedia {
            variant: EmptyMediaVariant::Icon,
            svg {
                class: "size-6",
                // SVG icon
            }
        }
        EmptyTitle { "No files found" }
        EmptyDescription { "You haven't uploaded any files yet." }
    }
    EmptyContent {
        Button { "Upload file" }
    }
}
```

### Simple Empty (No Icon)

```rust
Empty {
    EmptyHeader {
        EmptyTitle { "No items" }
        EmptyDescription { "There are no items to display at this time." }
    }
}
```

### Props

#### Empty

| Prop       | Type             | Default  | Description            |
| ---------- | ---------------- | -------- | ---------------------- |
| `class`    | `Option<String>` | `None`   | Additional CSS classes |
| `children` | `Element`        | Required | Empty state content    |

#### EmptyHeader

| Prop       | Type             | Default  | Description                                |
| ---------- | ---------------- | -------- | ------------------------------------------ |
| `class`    | `Option<String>` | `None`   | Additional CSS classes                     |
| `children` | `Element`        | Required | Header content (media, title, description) |

#### EmptyMedia

| Prop       | Type                | Default   | Description            |
| ---------- | ------------------- | --------- | ---------------------- |
| `variant`  | `EmptyMediaVariant` | `Default` | Visual style variant   |
| `class`    | `Option<String>`    | `None`    | Additional CSS classes |
| `children` | `Element`           | Required  | Icon or image content  |

#### EmptyTitle

| Prop       | Type             | Default  | Description            |
| ---------- | ---------------- | -------- | ---------------------- |
| `class`    | `Option<String>` | `None`   | Additional CSS classes |
| `children` | `Element`        | Required | Title text             |

#### EmptyDescription

| Prop       | Type             | Default  | Description            |
| ---------- | ---------------- | -------- | ---------------------- |
| `class`    | `Option<String>` | `None`   | Additional CSS classes |
| `children` | `Element`        | Required | Description text       |

#### EmptyContent

| Prop       | Type             | Default  | Description                          |
| ---------- | ---------------- | -------- | ------------------------------------ |
| `class`    | `Option<String>` | `None`   | Additional CSS classes               |
| `children` | `Element`        | Required | Action buttons or additional content |

### Media Variants

```rust
pub enum EmptyMediaVariant {
    Default,  // Transparent background
    Icon,     // Muted background with icon styling
}
```

| Variant   | Description                                    | Use Case                          |
| --------- | ---------------------------------------------- | --------------------------------- |
| `Default` | Transparent background                         | For custom styled icons or images |
| `Icon`    | Muted background with size-10, rounded corners | For standard icon displays        |

### Empty State Examples

#### Search Results

```rust
Empty {
    EmptyHeader {
        EmptyMedia {
            variant: EmptyMediaVariant::Icon,
            // Search icon
        }
        EmptyTitle { "No results found" }
        EmptyDescription {
            "We couldn't find what you're looking for. Try adjusting your search."
        }
    }
    EmptyContent {
        Button { "Clear search" }
    }
}
```

#### Inbox Zero

```rust
Empty {
    EmptyHeader {
        EmptyMedia {
            variant: EmptyMediaVariant::Icon,
            // Mail icon
        }
        EmptyTitle { "Inbox zero!" }
        EmptyDescription { "You're all caught up. Great work!" }
    }
}
```

#### No Data

```rust
Empty {
    EmptyHeader {
        EmptyMedia {
            variant: EmptyMediaVariant::Icon,
            // Chart icon
        }
        EmptyTitle { "No data available" }
        EmptyDescription {
            "Start collecting data to see insights and analytics here."
        }
    }
    EmptyContent {
        div {
            class: "flex gap-2",
            Button { "Get started" }
            Button { variant: ButtonVariant::Outline, "Learn more" }
        }
    }
}
```

### Features

- ✅ Flexible layout with optional sections
- ✅ Two media variant styles (default, icon)
- ✅ Centered, balanced text layout
- ✅ Support for icons, images, or custom media
- ✅ Action button support in content area
- ✅ Responsive padding and spacing
- ✅ Link styling in descriptions
- ✅ Customizable through Tailwind classes

### Best Practices

1. **Use descriptive titles**: Be clear about what's missing
2. **Provide context**: Explain why the state is empty
3. **Offer actions**: Include buttons to help users move forward
4. **Choose appropriate icons**: Use icons that match the context
5. **Keep it simple**: Don't overcomplicate empty states
6. **Use EmptyMedia variant**: Use `Icon` variant for consistent styling
7. **Min-height**: Add `min-h-[300px]` or similar to prevent layout shifts
8. **Border styling**: Use `border` or `border-dashed` classes for visual boundaries

### Styling

The Empty component uses these base styles:

- Layout: `flex flex-col items-center justify-center`
- Spacing: `gap-6` between sections, `p-6` padding (md: `p-12`)
- Text: `text-center text-balance` for readable, centered text
- Border: `border-dashed rounded-lg` for subtle boundaries
- Media (Icon): `size-10 rounded-lg bg-muted` with proper icon sizing

---

## Portal

A Portal component that renders children into a different part of the DOM tree, equivalent to React's `ReactDOM.createPortal`. This is useful for modals, overlays, and tooltips that need to break out of their parent container's DOM hierarchy.

### Basic Usage

```rust
use dioxus::prelude::*;
use dioxus_components::Portal;

#[component]
fn App() -> Element {
    let mut show_modal = use_signal(|| false);

    rsx! {
        div {
            button {
                onclick: move |_| show_modal.set(true),
                "Show Modal"
            }

            if show_modal() {
                Portal {
                    container: "body",
                    class: "modal-overlay",
                    div {
                        class: "fixed inset-0 bg-black/50 flex items-center justify-center",
                        onclick: move |_| show_modal.set(false),
                        div {
                            class: "bg-white p-6 rounded-lg",
                            onclick: move |e| e.stop_propagation(),
                            h2 { "Modal Title" }
                            p { "This content is rendered in document.body!" }
                            button {
                                onclick: move |_| show_modal.set(false),
                                "Close"
                            }
                        }
                    }
                }
            }
        }
    }
}
```

### Props

| Prop        | Type             | Default  | Description                                                         |
| ----------- | ---------------- | -------- | ------------------------------------------------------------------- |
| `container` | `String`         | `"body"` | CSS selector for target container (e.g., "body", "#root", ".modal") |
| `class`     | `Option<String>` | `None`   | Additional CSS classes for the portal wrapper                       |
| `id`        | `Option<String>` | `None`   | ID attribute for the portal wrapper                                 |

### Features

- **DOM Manipulation**: Uses JavaScript to physically move the portal element to the specified container
- **Client-Side Only**: Renders only after mounting to avoid SSR issues
- **Flexible Positioning**: Content can be positioned using CSS (typically `fixed` or `absolute`)
- **Event Handling**: Events bubble normally within the portal content
- **Multiple Portals**: Can render multiple portals to different containers

### Common Use Cases

#### Modal Dialog

```rust
Portal {
    container: "body",
    div {
        class: "fixed inset-0 z-50 bg-black/50 flex items-center justify-center",
        div {
            class: "bg-card p-6 rounded-lg shadow-xl max-w-md",
            h2 { class: "text-xl font-bold mb-4", "Confirm Action" }
            p { class: "mb-4", "Are you sure you want to proceed?" }
            div {
                class: "flex gap-2 justify-end",
                button { "Cancel" }
                button { "Confirm" }
            }
        }
    }
}
```

#### Dropdown Menu

```rust
Portal {
    container: "body",
    div {
        class: "absolute",
        style: "top: {y}px; left: {x}px;",
        div {
            class: "bg-card border rounded-md shadow-lg p-2",
            button { class: "w-full text-left px-3 py-2 hover:bg-accent", "Option 1" }
            button { class: "w-full text-left px-3 py-2 hover:bg-accent", "Option 2" }
            button { class: "w-full text-left px-3 py-2 hover:bg-accent", "Option 3" }
        }
    }
}
```

#### Toast Notification

```rust
Portal {
    container: "body",
    div {
        class: "fixed top-4 right-4 z-50",
        div {
            class: "bg-card border rounded-lg shadow-lg p-4 max-w-sm",
            div { class: "font-semibold", "Notification" }
            div { class: "text-sm text-muted-foreground", "Action completed successfully!" }
        }
    }
}
```

### Implementation Details

The Portal component:

1. Generates a unique ID for the portal element
2. Waits for the component to mount (client-side only)
3. Uses JavaScript (`onmounted` event) to move the portal element to the target container
4. The content is physically moved in the DOM tree, not just positioned with CSS
5. This matches React's `createPortal` behavior exactly

### Best Practices

- Use `container: "body"` for modals and overlays that should appear above all content
- Add `z-index` classes to ensure proper stacking order
- Use `fixed` positioning for overlays that cover the viewport
- Implement click-outside-to-close by adding `onclick` to the overlay with `e.stop_propagation()` on the content
- Clean up any event listeners when the portal unmounts

### Accessibility

- Ensure keyboard navigation works within the portal
- Manage focus when opening/closing portals
- Add ARIA attributes for screen readers (`role`, `aria-modal`, etc.)
- Trap focus within modals to prevent tabbing to background content

---

## Spinner

A loading spinner component using animated icon.

### Basic Usage

```rust
use dioxus::prelude::*;
use crate::components::{Spinner, SpinnerSize};

#[component]
fn App() -> Element {
    rsx! {
        Spinner {}
        Spinner {
            size: SpinnerSize::Large,
            color: "text-primary"
        }
    }
}
```

### Props

| Prop    | Type             | Default          | Description            |
| ------- | ---------------- | ---------------- | ---------------------- |
| `size`  | `SpinnerSize`    | `Small`          | Spinner size           |
| `color` | `Option<String>` | `"text-current"` | Tailwind color class   |
| `class` | `Option<String>` | `None`           | Additional CSS classes |

### Sizes

```rust
// Small (16px)
Spinner { size: SpinnerSize::Small }

// Medium (24px)
Spinner { size: SpinnerSize::Medium }

// Large (32px)
Spinner { size: SpinnerSize::Large }

// XLarge (48px)
Spinner { size: SpinnerSize::XLarge }
```

### Colors

```rust
// Primary color
Spinner { color: "text-primary" }

// Destructive (red)
Spinner { color: "text-destructive" }

// Secondary
Spinner { color: "text-secondary-foreground" }

// White (for dark backgrounds)
Spinner { color: "text-white" }
```

### In Buttons

```rust
Button {
    class: "gap-2",
    Spinner { size: SpinnerSize::Small, color: "text-primary-foreground" }
    "Loading..."
}
```

---

## Tooltip

Displays additional information when hovering over or focusing on an element.

### Basic Usage

```rust
use dioxus::prelude::*;
use crate::components::{TooltipProvider, Tooltip, TooltipTrigger, TooltipContent, TooltipSide};

#[component]
fn App() -> Element {
    rsx! {
        TooltipProvider {
            Tooltip {
                TooltipTrigger {
                    button { "Hover me" }
                }
                TooltipContent {
                    side: TooltipSide::Top,
                    "This is a tooltip"
                }
            }
        }
    }
}
```

### Props

#### TooltipProvider

| Prop                        | Type   | Default | Description                           |
| --------------------------- | ------ | ------- | ------------------------------------- |
| `delay_duration`            | `u64`  | `400`   | Delay before tooltip opens (ms)       |
| `skip_delay_duration`       | `u64`  | `300`   | Grace period before delay resets (ms) |
| `disable_hoverable_content` | `bool` | `false` | Don't allow hovering the content      |

#### Tooltip

| Prop                        | Type                   | Default | Description               |
| --------------------------- | ---------------------- | ------- | ------------------------- |
| `open`                      | `Option<Signal<bool>>` | `None`  | Controlled open state     |
| `default_open`              | `bool`                 | `false` | Initial open state        |
| `delay_duration`            | `Option<u64>`          | `None`  | Override provider delay   |
| `disable_hoverable_content` | `Option<bool>`         | `None`  | Override provider setting |

#### TooltipTrigger

| Prop      | Type                   | Default | Description            |
| --------- | ---------------------- | ------- | ---------------------- |
| `class`   | `Option<String>`       | `None`  | Additional CSS classes |
| `onclick` | `Option<EventHandler>` | `None`  | Click handler          |

#### TooltipContent

| Prop          | Type             | Default | Description                  |
| ------------- | ---------------- | ------- | ---------------------------- |
| `side`        | `TooltipSide`    | `Top`   | Position relative to trigger |
| `side_offset` | `i32`            | `4`     | Distance from trigger (px)   |
| `class`       | `Option<String>` | `None`  | Additional CSS classes       |
| `aria_label`  | `Option<String>` | `None`  | Accessibility label          |

### Tooltip Sides

```rust
// Top - Appears above trigger
TooltipContent { side: TooltipSide::Top, "Tooltip" }

// Right - Appears to the right
TooltipContent { side: TooltipSide::Right, "Tooltip" }

// Bottom - Appears below trigger
TooltipContent { side: TooltipSide::Bottom, "Tooltip" }

// Left - Appears to the left
TooltipContent { side: TooltipSide::Left, "Tooltip" }
```

### With Arrow

```rust
use crate::components::TooltipArrow;

TooltipContent {
    side: TooltipSide::Top,
    TooltipArrow {}
    "Tooltip text"
}
```

### Controlled State

```rust
let mut is_open = use_signal(|| false);

TooltipProvider {
    Tooltip {
        open: is_open,
        TooltipTrigger {
            button { "Hover me" }
        }
        TooltipContent {
            "This is a controlled tooltip"
        }
    }
}
```

### Multiple Tooltips

```rust
TooltipProvider {
    div {
        class: "flex gap-4",

        Tooltip {
            TooltipTrigger { "Hover 1" }
            TooltipContent { "Tooltip 1" }
        }

        Tooltip {
            TooltipTrigger { "Hover 2" }
            TooltipContent { "Tooltip 2" }
        }
    }
}
```

---

## Common Patterns

### Custom Styling

All components support the `class` prop for additional Tailwind classes:

```rust
Button {
    class: "rounded-full px-6 py-3 text-lg",
    "Styled Button"
}
```

### Dark Mode Support

All components include dark mode variants via Tailwind CSS:

```rust
// Automatically supports dark mode with dark: prefix
Badge {
    variant: BadgeVariant::Destructive,
    "Dark mode ready"
}
```

### Composition

Combine multiple components:

```rust
div {
    class: "flex items-center gap-4",

    Avatar {
        AvatarImage { src: "https://example.com/avatar.jpg" }
        AvatarFallback { "JD" }
    }

    div {
        h3 { "John Doe" }
        Badge { "Active" }
    }
}
```

### Event Handling

```rust
Button {
    onclick: move |_| {
        println!("Button clicked!");
    },
    "Click me"
}
```

---

## Accessibility

All components follow WAI-ARIA design patterns:

- **Accordion**: ARIA roles, keyboard navigation, focus management
- **Avatar**: Image alt text, fallback for loading states
- **Badge**: Semantic HTML with proper color contrast
- **Button**: Disabled state, aria-invalid for forms, keyboard accessible
- **Spinner**: ARIA label for screen readers
- **Tooltip**: ARIA described by, keyboard triggers

---

## Styling Guide

Components use Tailwind CSS v4 with a comprehensive theme:

### Colors

- `primary` / `primary-foreground` - Main brand color
- `secondary` / `secondary-foreground` - Secondary actions
- `destructive` / `destructive-foreground` - Dangerous actions (red)
- `accent` / `accent-foreground` - Accent highlights
- `muted` / `muted-foreground` - Disabled or secondary text
- `background` / `foreground` - Base colors
- `border` / `input` - Form elements

### Customization

Update theme colors in `tailwind.css`:

```css
@theme {
  --color-primary: hsl(221.2 83.2% 53.3%);
  --color-primary-foreground: hsl(210 40% 98%);
  /* ... more colors ... */
}
```

---

## Contributing

When creating new components:

1. Create a subfolder in `/src/components/{component_name}/`
2. Add three files:
   - `{component_name}.rs` - Component logic
   - `{component_name}.css` - Component styles
   - `mod.rs` - Module exports
3. Update `/src/components/mod.rs` to export the new component
4. Add CSS import to `/tailwind.css`
5. Add documentation to this file

---

## Resources

- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Radix UI Primitives](https://www.radix-ui.com/primitives)
- [WAI-ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
