extern crate cc;

fn main() {
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=WebKit");

    cc::Build::new()
        .define("WEBVIEW_COCOA", "1")
        .file("library/webview.c")
        .flag("-x")
        .flag("objective-c")
        .compile("libwebview.a");
}
