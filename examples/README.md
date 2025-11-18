# Dioxus Components Showcase

An interactive showcase of all Dioxus components with live demos and examples.

## Running the Showcase

### Using Dioxus CLI (Recommended)

```bash
dx serve --example showcase
```

### Using Cargo

```bash
# For web
cargo run --example showcase --features web

# For desktop
cargo run --example showcase --features desktop
```

## Structure

The showcase is organized into several pages:

### Getting Started

- **Landing Page** (`/`) - Overview of all components with quick navigation
- **Installation** (`/installation`) - Step-by-step setup guide

### Component Demos

- **Accordion** (`/components/accordion`) - Collapsible content panels
- **Avatar** (`/components/avatar`) - User profile images with fallback
- **Badge** (`/components/badge`) - Small status indicators
- **Button** (`/components/button`) - Interactive buttons with variants
- **Card** (`/components/card`) - Flexible container component
- **Checkbox** (`/components/checkbox`) - Three-state checkbox inputs
- **Dialog** (`/components/dialog`) - Modal dialogs and alerts
- **Empty** (`/components/empty`) - Empty state placeholders
- **Portal** (`/components/portal`) - Render content outside DOM hierarchy
- **Spinner** (`/components/spinner`) - Loading indicators
- **Tooltip** (`/components/tooltip`) - Hover information popups

## Features

- 🎨 **Live Demos** - See components in action
- 📝 **Code Examples** - Copy-paste ready code snippets
- 📱 **Responsive Design** - Works on all screen sizes
- 🔍 **Easy Navigation** - Sidebar with organized sections
- 🚀 **Built with Dioxus 0.7** - Latest framework features

## Development

The example is structured as follows:

```
examples/
├── main.rs          # Entry point with router and layout
└── pages/           # Individual demo pages
    ├── mod.rs
    ├── landing.rs
    ├── installation.rs
    └── *_demo.rs    # Component demo pages
```

Each component demo page follows a consistent structure:

1. Title and description
2. Live component examples
3. Code snippets
4. Usage notes
