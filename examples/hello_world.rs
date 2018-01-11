extern crate webview;

use webview::{Content, WebView};

fn main() {
    let view = WebView::new(
        "My awesome title",                     // The title of the window
        Content::Html("<h1>Hello world!</h1>"), // The content to display
        200,                                    // Width
        100,                                    // Height
        false,                                  // Resizable?
        false,                                  // Debugable?
    ).unwrap();

    // Starts the event loop
    view.join();
}
