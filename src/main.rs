// GPUI Tutorial: Creating a Native GUI Application in Rust
// This tutorial demonstrates how to build a native-loooking GUI
// dialog application using the GPUI library in Rust.
//
// We'll cover: components, event handling, styling and window management.
//
// =======================================================================
// KEY CONCEPTS COVERED IN THIS TUTORIAL
// =======================================================================
//
// 1. COMPONENTS: Structs that implement the Render trait
//      - Backdrop: A simple full-screen overlay
//      - DialogBox: A complex component with state and event handling
//
// 2. RENDER TRAIT: Defines how a component looks
//      - Returns an element tree using the builder pattern
//
// 3. ELEMENT BUILDER: Chain methods to configure elements
//      - Layout: .flex(), .flex_col(), .justify_center()
//      - Sizing: .w(), .h(), .size_full()
//      - Styling: .bg(), .text_color(), .rounded()
//      - Events: .on_mouse_up(), .on_key_down()
//
// 4. EVENT HANDLING: Respond to user input
//      - cs.listener() wraps methods as event handlers
//      - Hanlders recieve event, window, and context parameters
//
// 5. WINDOW MANAGEMENT: Create and configure windows
//      - WindowOptions: configures behavior and appearances
//      - WindowKind::Popup for floating modal windows
//      - Transparent backgrounds for custom shapes
//
// 6. COLORS: Multiple ways to specify colours
//      - rgb(0xRRGGBB): Opaque RGB color
//      - hsla(hue, saturation, lightness): HSL color with alpha
//
// 7. SIZING: Use px() for pixel-based dimensions
//      - px(10.0): 10 pixels
//      - relative(1.4): Relative to parent size (for line-height, etc)
//
// =======================================================================
//

// Import all GPUI types and traits
use gpui::*;

// ======================================================================
// BACKDROP COMPONENT
// ======================================================================
// The Backdrop creates a semi-transparent overlay behind the dialog.
// This is a common pattern in modal dialogs to dim the background and
// focus user attention on the dialog itself.

struct Backdrop;

// The Render trait is required for all GPUI components that display UI.
// It has one method: render(), which returns the component's visual representation.

impl Render for Backdrop {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // div() creates a container element (similar to HTML's <div>)
        // GPUI uses a builder pattern where you chain methods to configure the element.
        div()
            .size_full() // Full width and height
            .bg(hsla(0.0, 0.0, 0.0, 0.3)) // Semi-transparent black background (30% opacity)
                                          // hsla(hue, saturation, lightness, alpha)
    }
}

// ======================================================================
// DIALOG BOX COMPONENT
// ======================================================================
// The main dialog component that displays content and interactive buttons.

struct DialogBox;

// Implementation block for event handlers
// In GPUI, event handlers are methods that receive events and can modify state

impl DialogBox {
    // Mouse event handler for the "OK" button
    // Parameters:
    // - &mut self: mutable reference to this component
    // - MouseUpEvent: the event that triggered this handler
    // - &mut Window: reference to the window (unuused here)
    // - cx: Context provides access to app-level operations

    fn on_ok_clicked(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        cx.quit(); // Quit the application
    }

    // Mouse event handler for the "Cancel" button
    // Parameters:
    // - &mut self: mutable reference to this component
    // - MouseUpEvent: the event that triggered this handler
    // - &mut Window: reference to the window (unuused here)
    // - cx: Context provides access to app-level operations
    fn on_cancel_clicked(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        cx.quit(); // Quit the application
    }
}
