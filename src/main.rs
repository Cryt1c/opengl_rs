use three_d::{Context, FrameOutput, Window, WindowSettings};

mod renderer;
mod shader;
mod threerenderer;
mod uniform;
mod volume;

fn main() {
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 380.0]),
    //     multisampling: 4,
    //     renderer: eframe::Renderer::Glow,
    //     ..Default::default()
    // };
    // let _ = eframe::run_native(
    //     "Raycaster",
    //     options,
    //     Box::new(|cc| Box::new(renderer::Renderer::new(cc))),
    // );

    let window = Window::new(WindowSettings {
        title: "Core Triangle!".to_string(),
        #[cfg(not(target_arch = "wasm32"))]
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context: Context = window.gl();
    let width = window.size().0 as f32;
    let height = window.size().1 as f32;

    let mut renderer = threerenderer::ThreeRenderer::new(context, width, height);

    window.render_loop(move |frame_input| {
        renderer.update();
        FrameOutput::default()
    });
}
