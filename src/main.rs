use clap::Parser;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value = "https://blog.ggymm.com")]
    url: String,
}

fn main() -> wry::Result<()> {
    let args = Args::parse();

    let event = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("ATLAS")
        .with_inner_size(tao::dpi::LogicalSize::new(1440.0, 960.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(1200.0, 800.0))
        .build(&event)
        .unwrap();

    let builder = WebViewBuilder::new().with_url(args.url);

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let _webview = builder.build(&window)?;
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

    event.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
