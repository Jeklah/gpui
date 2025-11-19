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

    // Keyboard event handler for ESC key
    // This allows users to close the dialog with the keyboard
    // Parameters:
    // - &mut self: mutable reference to this component
    // - KeyDownEvent: the event that triggered this handler
    // - &mut Window: reference to the window (unuused here)
    // - cx: Context provides access to app-level operations

    fn on_escape(&mut self, _: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        cx.quit(); // Quit the application
    }
}

// Implement the Render trait to define how the dialog looks
impl Render for DialogBox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // ======================================================
        // OUTER CONTAINER
        // ======================================================
        // This is the full-window container that centers the dialog.
        // We use flexbox layout (similar to CSS flexbox) to center content.

        div()
            .flex() // Enable flexbox layout
            .size_full() // Take up the full window size
            .justify_center() // Center content horizontally
            .items_center() // Center content vertically
            // Attach keyboard event handler for ESC key
            // cx.listener() converts a method into an event listener
            .on_key_down(cx.listener(Self::on_escape))
            .child(
                // ==================================================
                // DIALOG CONTAINER
                // ==================================================
                // This is the actual dialog box with all its chrome (titlebar, content, buttons)
                div()
                    .flex()
                    .flex_col() // Stack children vertically
                    .rounded(px(10.0)) // 10px rounded corners
                    .shadow_lg() // Large shadow for elevation/depth
                    .overflow_hidden() // Clip children to rounded corners
                    .w_full() // Fill parent width
                    .h_full() // Fill parent height
                    .child(
                        // ==================================================
                        // TITLEBAR WITH TRAFFIC LIGHTS
                        // ==================================================
                        // macOS dialogs have a gray titlebar with three colored buttons
                        // (red, yellow, green) on the left side.
                        div()
                            .flex() // Horizontal layout
                            .items_center() // Vertically center items
                            .h(px(22.0)) // 22px height (macOS standard)
                            .w_full() // Full width
                            .bg(rgb(0xE8E8E8)) // Light gray background
                            .border_b_1() // 1px border on bottom
                            .border_color(rgb(0xD0D0D0)) // Darker gray border
                            .px_3() // Horizontal padding
                            .gap_2() // 8px gap between items
                            .child(
                                // RED CLOSE BUTTON
                                // Clicking this will close the dialog
                                div()
                                    .w(px(12.0)) // 12px diameter
                                    .h(px(12.0)) // 12px diameter
                                    .rounded_full() // Fully rounded (circle)
                                    .bg(rgb(0xFF5F57)) // Red color
                                    .border_1() // 1px border
                                    .border_color(rgb(0xE04943)) // Darker red border
                                    .cursor_pointer() // Show pointer cursor on hover
                                    // Attach click handler
                                    .on_mouse_up(
                                        MouseButton::Left,
                                        cx.listener(Self::on_cancel_clicked),
                                    ),
                            )
                            .child(
                                // YELLOW MINIMIZE BUTTON
                                // In this example, it's decorative only
                                div()
                                    .w(px(12.0))
                                    .h(px(12.0))
                                    .rounded_full()
                                    .bg(rgb(0xFFBD2E))
                                    .border_1()
                                    .border_color(rgb(0xDEA123)), // Darker yellow border
                            )
                            .child(
                                // GREEN MAXIMIZE BUTTON
                                // In this example, it's decorative only
                                div()
                                    .w(px(12.0))
                                    .h(px(12.0))
                                    .rounded_full()
                                    .bg(rgb(0x28C940))
                                    .border_1()
                                    .border_color(rgb(0x1AAB29)), // Darker green border
                            ),
                    )
                    .child(
                        // ==================================================
                        // MAIN CONTENT AREA
                        // ==================================================
                        // This contains the dialog message and action buttons
                        div()
                            .flex()
                            .flex_col() // Stack message and buttons vertically
                            .bg(rgb(0xEFEFEF)) // Light gray background (macOS style)
                            .flex_1() // Take up remaining space
                            .px_6() // 24px horizontal padding
                            .py_5() // 20px vertical padding
                            .child(
                                // =================================================
                                // MESSAGE TEXT CONTAINER
                                // =================================================
                                div()
                                    .flex() // Enable flex layout
                                    .flex_1() // Expand to fill available space
                                    .items_center() // Center text vertically
                                    .px_3() // 12px horizontal padding
                                    .py_4() // 16px vertical padding
                                    .child(
                                        // THE ACTIAL TEXT
                                        // In GPUI, text styling is applied via methods
                                        div()
                                            .text_size(px(13.0)) // 13px font size
                                            .text_color(rgb(0x000000)) // Black text color
                                            .font_weight(FontWeight::NORMAL) // Normal weight
                                            .line_height(relative(1.4)) // 1.4 line spacing
                                            .child("Hello world!"), // The text content
                                    ),
                            )
                            .child(
                                // =================================================
                                // ACTION BUTTONS CONTAINER
                                // =================================================
                                // macOS convention: buttons are right-aligned, with Cancel on
                                // the left and the primary action (OK) on the right.
                                div()
                                    .flex() // Horizontal layout
                                    .gap_3() // 12px gap between buttons
                                    .justify_end() // Right-align buttons
                                    .w_full() // Full width
                                    .mt_3() // 12px top margin
                                    .child(
                                        // CANCEL BUTTON
                                        // Secondary action button - uses white/gray style
                                        div()
                                            .flex()
                                            .items_center() // Center text vertically
                                            .justify_center() // Center text horizontally
                                            .px_6() // 24px horizontal padding
                                            .h(px(32.0)) // 32px height
                                            .min_w(px(90.0)) // Minimum width 90px
                                            .bg(rgb(0xFFFFFF)) // White background
                                            .text_color(rgb(0x000000)) // Black text
                                            .text_size(px(13.0)) // 13px font size
                                            .font_weight(FontWeight::NORMAL) // Normal weight
                                            .rounded(px(6.0)) // 6px rounded corners
                                            .border_1() // 1px border
                                            .border_color(rgb(0xB8B8B8)) // Gray border
                                            .cursor_pointer() // Pointer cursor on hover
                                            .shadow_sm() // Small shadow
                                            // Hover state: slightly darken the background
                                            // The closure receives a mutable style object
                                            .hover(|style| style.bg(rgb(0xF8F8F8)))
                                            // Attach click handler
                                            .on_mouse_up(
                                                MouseButton::Left,
                                                cx.listener(Self::on_cancel_clicked),
                                            )
                                            .child("Cancel"), // Button text
                                    )
                                    .child(
                                        // OK BUTTON
                                        // Primary action button - uses blue style
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .px_6()
                                            .h(px(32.0))
                                            .min_w(px(90.0))
                                            .bg(rgb(0x007AFF)) // Blue background
                                            .text_color(rgb(0xFFFFFF)) // White text
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::NORMAL)
                                            .rounded(px(6.0)) // 6px rounded corners
                                            .cursor_pointer() // Pointer cursor on hover
                                            .shadow_sm() // Small shadow
                                            // Hover state: darken the blue background
                                            .hover(|style| style.bg(rgb(0x0068DB)))
                                            // Attach click handler
                                            .on_mouse_up(
                                                MouseButton::Left,
                                                cx.listener(Self::on_ok_clicked),
                                            )
                                            .child("Ok"), // Button text
                                    ),
                            ),
                    ),
            )
    }
}

// ======================================================================
// MAIN APPLICATION ENTRY POINT
// ======================================================================
// This is where we initialize the GPUI application and create our windows.
// A GPUI app can have multiple windows, and we're creating two:
// 1. A backdrop window (full-screen, transparent overlay)
// 2. The dialog window (centered, with our DialogBox component)
// ======================================================================

fn main() {
    // Create a new GPUI application and run it
    // The closure receives an App context (cx) which provides access to
    // app-level operations like creating windows, accessing displays, etc.
    Application::new().run(|cx: &mut App| {
        // ==================================================
        // GET DISPLAY INFORMATION
        // ==================================================
        // We need to know the screen size to position our windows correctly

        let displays = cx.displays(); // Get all connected displays
        let display = displays.first().unwrap(); // Use the primary display
        let screen_size = display.bounds().size; // Get the dimensions of the display

        // ==================================================
        // CREATE BACKDROP WINDOW
        // ==================================================
        // This creates a full-screen, semi transparent overlay behind the dialog.
        // It dims the background and gives the dialog a modal appearance.

        cx.open_window(
            // WindowOptions configures how the window behaves and appears
            WindowOptions {
                // Position and size: full screen (covers entire display)
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.0), px(0.0)), // Top-left corner
                    size: screen_size,
                })),

                titlebar: None, // No titlebar (we want a borderless window)
                focus: false,   // Don't steal focus (the dialog should be focused)
                show: true,     // Make window visible immediately

                // WindowKind::PopUp creates a floating window without OS chrome
                kind: WindowKind::PopUp,

                is_movable: false,              // User can't drag this window
                display_id: Some(display.id()), // Show on the primary display

                // Transparent background lets the backdrop's semi-transparent
                // color show through
                window_background: WindowBackgroundAppearance::Transparent,

                ..Default::default() // Use default values for other options
            },
            // Window content factory: creates the Backdrop component
            // |_, cx| receives (WindowHandle, Context)
            // cx.new() creates a new component instance
            |_, cx| cx.new(|_cx| Backdrop),
        )
        .unwrap(); // Panic if window creation fails (for this example)

        // ==================================================
        // CREATE DIALOG POSITION
        // ================================================
        // We want the dialog centered on the screen.

        let dialog_width = px(460.0); // Dialog width in pixels
        let dialog_height = px(180.0); // Dialog height in pixels

        // Calculate centered position
        let x = (screen_size.width - dialog_width) / 2.0; // Horizontal center
        let y = (screen_size.height - dialog_height) / 2.0; // Vertical center

        let dialog_bounds = Bounds {
            origin: point(x, y),
            size: size(dialog_width, dialog_height),
        };

        // ==================================================
        // CREATE DIALOG WINDOW
        // ==================================================
        // This is the actual modal dialog that contains our DialogBox component

        cx.open_window(
            WindowOptions {
                // Position and size: centered on screen
                window_bounds: Some(WindowBounds::Windowed(dialog_bounds)),

                titlebar: None, // No OS titlebar (we draw our own)
                focus: true,    // This window should have keyboard focus
                show: true,     // Make visible immediately

                // PopUp windows float above other windows
                kind: WindowKind::PopUp,
                is_movable: false,              // User can't drag the dialog
                display_id: Some(display.id()), // Show on primary display

                // Transparent background allows our rounded corners and
                // shadow to render properly
                window_background: WindowBackgroundAppearance::Transparent,

                ..Default::default() // Default values for other options
            },
            // Create the DialogBox component
            |_, cx| cx.new(|_cx| DialogBox),
        )
        .unwrap(); // Panic if window creation fails
    });
}
