extern crate web_view;

use web_view::*;

fn main() {
    let size = (700, 400);
    let resizable = true;
    let debug = true;
    let titlebar_transparent = true;
    let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
    let userdata = ();

    let html = format!(r#"
    <!DOCTYPE html>
    <html>

    <head>

    <title>Realtime communication with WebRTC</title>
    <style>{css}</style>
    </head>

    <body>

    <h1>Realtime communication with WebRTC</h1>

    <script>{snake}</script>
    </script></head><body><script type="text/javascript">Elm.Main.fullscreen()</script>
    </body>

    </html>

    "#,
    css = include_str!("../www/min.css"),
    snake = include_str!("../www/snake.js"));
    // js = include_str!("../www/dist.js"));

    run(
        "",
        Content::Html(html),
        Some(size),
        resizable,
        debug,
        titlebar_transparent,
        move |mut webview| {
            webview.set_background_color(0.11, 0.12, 0.13, 1.0);
        },
        frontend_cb,
        userdata
    );
}
