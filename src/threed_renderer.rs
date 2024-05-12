use crate::shader::Shader;
use crate::volume::Volume;
use glow::{HasContext, NativeBuffer, NativeTexture, NativeVertexArray};
use nalgebra::{Matrix4, Vector3};
use std::{mem, sync::Arc};
use three_d::{Context, Window};

pub struct ThreeRenderer {
    start_time: std::time::Instant,
    frame_count: u32,
    pub gl: Arc<three_d::Context>,
    pub vbo: Option<NativeBuffer>,
    pub vao: Option<NativeVertexArray>,
    pub ebo: Option<NativeBuffer>,
    pub texture: Option<NativeTexture>,
    pub volume: Volume,
    pub mip_shader: bool,
    pub camera_y: f32,
    pub camera_z: f32,
    pub width: f32,
    pub height: f32,
}

impl ThreeRenderer {
    pub fn new(context: Context, width: f32, height: f32) -> Self {
        let arc_context = Arc::new(context.clone());
        let mut renderer = ThreeRenderer {
            gl: arc_context,
            vao: None,
            vbo: None,
            ebo: None,
            texture: None,
            frame_count: 0,
            start_time: std::time::Instant::now(),
            volume: Volume::new(),
            mip_shader: false,
            camera_y: 0.0,
            camera_z: -2.5,
            width,
            height,
        };
        renderer.create_vao();
        renderer.create_vbo();
        renderer.create_ebo();
        renderer.create_texture();
        renderer
    }
    pub fn create_vao(&mut self) {
        unsafe {
            self.vao = self.gl.create_vertex_array().ok();
            self.gl.bind_vertex_array(self.vao);
        }
    }
    pub fn create_vbo(&mut self) {
        unsafe {
            self.vbo = self.gl.create_buffer().ok();
            self.gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&self.volume.vertex_data),
                glow::STATIC_DRAW,
            );
        }
    }
    pub fn create_ebo(&mut self) {
        unsafe {
            self.ebo = self.gl.create_buffer().ok();
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.ebo);
            self.gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&self.volume.indices),
                glow::STATIC_DRAW,
            );
            self.gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                3 * mem::size_of::<f32>() as i32,
                0,
            );
            self.gl.enable_vertex_attrib_array(0);
        }
    }
    pub fn create_texture(&mut self) {
        unsafe {
            self.texture = self.gl.create_texture().ok();
            self.gl.bind_texture(glow::TEXTURE_3D, self.texture);
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_3D,
                glow::TEXTURE_WRAP_R,
                glow::CLAMP_TO_EDGE as i32,
            );

            self.gl.tex_image_3d(
                glow::TEXTURE_3D,
                0,
                glow::RGB as i32,
                self.volume.width as i32,
                self.volume.height as i32,
                self.volume.depth as i32,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                Some(bytemuck::cast_slice(&self.volume.texture_data)),
            );
            self.gl.generate_mipmap(glow::TEXTURE_3D);
        }
    }

    pub fn update(&self) {
        // Create local variables to ensure thread safety
        let texture = self.texture;
        let vao = self.vao;
        let indices_length = self.volume.indices.len();
        let mip_shader = self.mip_shader;
        let camera_y = self.camera_y;
        let camera_z = self.camera_z;

        let fragment_shader = if mip_shader {
            "shaders/mip_shader.glsl"
        } else {
            "shaders/raycaster.glsl"
        };
        let shaders = Shader::load_from_file("shaders/vertex_shader.glsl", fragment_shader);
        let vs = shaders.compile_shader(&self.gl, shaders.get_vertex(), glow::VERTEX_SHADER);
        let fs = shaders.compile_shader(&self.gl, shaders.get_fragment(), glow::FRAGMENT_SHADER);
        let program = shaders.link_program(&self.gl, vs, fs);
        unsafe {
            &self.gl.bind_texture(glow::TEXTURE_3D, texture);
            shaders.use_program(&self.gl, program);

            // Set uniforms
            let fov_radians = 45.0_f32.to_radians();
            let aspect_ratio = self.width / self.height;
            let model_matrix =
                nalgebra_glm::rotate(&Matrix4::identity(), 0.0, &Vector3::new(0.0, 1.0, 0.0));
            let cam_pos = Vector3::new(camera_y, 0.0, camera_z);

            let view_matrix = nalgebra_glm::look_at(
                &cam_pos,
                &Vector3::new(0.0, 0.0, 0.0),
                &Vector3::new(0.0, 1.0, 0.0),
            );

            let projection_matrix =
                nalgebra_glm::perspective(fov_radians, aspect_ratio, 0.1, 100.0);
            Shader::set_uniform_value(&self.gl, program, "camPos", cam_pos);
            Shader::set_uniform_value(&self.gl, program, "M", model_matrix);
            Shader::set_uniform_value(&self.gl, program, "V", view_matrix);
            Shader::set_uniform_value(&self.gl, program, "P", projection_matrix);

            self.gl.enable(glow::DEPTH_TEST);
            self.gl.bind_vertex_array(vao);
            self.gl.draw_elements(
                glow::TRIANGLES,
                indices_length as i32,
                glow::UNSIGNED_INT,
                0,
            );
            if self.gl.get_error() != glow::NO_ERROR {
                println!("Error: {}", self.gl.get_error());
            }
        }
    }
}
