mod renderer;
mod shader;
mod uniform;
mod volume;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 380.0]),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Raycaster",
        options,
        Box::new(|cc| Box::new(renderer::Renderer::new(cc))),
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hounsfield_normalization() {
        let input: Vec<u16> = vec![4095, 0, 2047];
        let expected: Vec<u8> = vec![255, 0, 127];
        let result: Vec<u8> = input
            .iter()
            .map(|&x| volume::Volume::normalize_hounsfield_units(x))
            .collect();
        println!("{:?}", result);

        assert_eq!(expected, result);
    }
}
