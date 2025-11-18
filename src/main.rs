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
