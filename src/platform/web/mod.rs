mod http_fetcher;
mod workflow;

use std::panic;

use log::Level;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::platform::web::WindowBuilderExtWebSys;
use winit::window::{Window, WindowBuilder};

use console_error_panic_hook;
pub use instant::Instant;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Window as WebSysWindow;

// WebGPU
#[cfg(not(feature = "web-webgl"))]
pub const COLOR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;

// WebGL
#[cfg(feature = "web-webgl")]
pub const COLOR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

use crate::io::tile_cache::TileCache;
use crate::io::workflow::Workflow;
pub use http_fetcher::PlatformHttpFetcher;

#[wasm_bindgen(start)]
pub fn start() {
    if let Err(_) = console_log::init_with_level(Level::Info) {
        // Failed to initialize logging. No need to log a message.
    }
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub async fn run(workflow_ptr: *mut Workflow) {
    let workflow: Box<Workflow> = unsafe { Box::from_raw(workflow_ptr) };
    let event_loop = EventLoop::new();

    let web_window: WebSysWindow = web_sys::window().unwrap();
    let document = web_window.document().unwrap();
    let body = document.body().unwrap();
    let builder = WindowBuilder::new();
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("mapr")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let window: Window = builder
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap();

    window.set_inner_size(LogicalSize {
        width: body.client_width(),
        height: body.client_height(),
    });

    // Either call forget or the main loop to keep worker loop alive
    crate::main_loop::setup(
        window,
        event_loop,
        Box::new(workflow.tile_request_dispatcher),
        Box::new(workflow.layer_result_receiver),
        Box::new(TileCache::new()),
    )
    .await;
    // std::mem::forget(workflow);
}
