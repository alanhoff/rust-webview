rust-webview
============
⚠️ This project is a work in progress. Very alpha. Really.

This crate uses C bindings from [zserge/webview](1) to display a Rust controllable
webview for rendering modern UI interfaces using web technology. Included features:

* Render HTML5, JavaScript and CSS in a full-fledged browser environment
* Call Rust from JavaScript and JavaScript from Rust
* Cross-platform: uses WebKit (Cocoa) on macOS, WebKit (GTK) on Linux and MSHTML on Windows
* Controllable window properties: title, width, height and fullscreen
* Compile to a single small binary: `hello_world` example is less than `200KB` when packed
* Controllable dialogs: alerts (info, warning, error), open and save files
* Embeddable content for offline usage
* Can render arbitrary HTML or a full external website
* Think about it as an Electron app but `121MB` smaller

### Example

```rust
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
```

![Screenshot](https://raw.githubusercontent.com/alanhoff/rust-webview/master/assets/screenshot.png)

### ISC License

```
Copyright 2018 Alan Hoffmeister <alanhoffmeister@gmail.com>

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.
```

[1]: https://github.com/zserge/webview
