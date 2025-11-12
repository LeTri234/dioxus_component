# Dioxus Components Documentation

A comprehensive guide to using the Dioxus 0.7 component library. All components are built with Tailwind CSS v4 and follow Radix UI design patterns.

## Table of Contents

1. [Accordion](#accordion)
2. [Avatar](#avatar)
3. [Badge](#badge)
4. [Button](#button)
5. [Spinner](#spinner)
6. [Tooltip](#tooltip)

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
