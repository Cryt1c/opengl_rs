use three_d::{ClearState, Context, FrameOutput, Window, WindowSettings};

mod glow_renderer;
mod shader;
mod threed_renderer;
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
    //     Box::new(|cc| Box::new(glow_renderer::GlowRenderer::new(cc))),
    // );

    let window = Window::new(WindowSettings {
        title: "Volume!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context: Context = window.gl();
    println!("{:?}", window.viewport());
    let width = window.viewport().width as f32;
    let height = window.viewport().height as f32;

    let renderer = threed_renderer::ThreeRenderer::new(context, width, height);

    window.render_loop(move |frame_input| {
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0));
        renderer.update();
        FrameOutput::default()
    });
}
