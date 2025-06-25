use wgpu::util::DeviceExt;
use winit::window::Window;
use crate::rendering::pipeline::Pipeline;
use crate::input::Input;
use glam::{Mat4, Vec3};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pipeline: Pipeline,
    pub input: Input,
    pub camera_position: Vec3,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_yaw: f32,
    camera_pitch: f32,
    cursor_visible: bool,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let camera_position = Vec3::new(0.0, 0.0, 2.0);
        let view = Mat4::look_at_rh(camera_position, Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
        let proj = Mat4::perspective_rh(f32::to_radians(45.0), size.width as f32 / size.height as f32, 0.1, 100.0);
        let camera_uniform = CameraUniform {
            view_proj: (proj * view).to_cols_array_2d(),
        };
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let pipeline = Pipeline::new(&device, &config, &queue, &camera_bind_group_layout);
        let input = Input::new();
        window.set_cursor_visible(false); // Hide cursor initially

        Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            input,
            camera_position,
            camera_buffer,
            camera_bind_group,
            camera_yaw: 0.0,
            camera_pitch: 0.0,
            cursor_visible: false,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            let view = Mat4::look_at_rh(self.camera_position, self.camera_position + self.get_forward(), Vec3::Y);
            let proj = Mat4::perspective_rh(f32::to_radians(45.0), new_size.width as f32 / new_size.height as f32, 0.1, 100.0);
            let camera_uniform = CameraUniform {
                view_proj: (proj * view).to_cols_array_2d(),
            };
            self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
        }
    }

    fn get_forward(&self) -> Vec3 {
        let (sin_pitch, cos_pitch) = self.camera_pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.camera_yaw.sin_cos();
        Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize()
    }

    pub fn update(&mut self, window: &Window) {
        let speed = 0.1;
        let forward = self.get_forward();
        let right = forward.cross(Vec3::Y).normalize();
        let up = Vec3::Y;
        if self.input.forward {
            self.camera_position += forward * speed;
        }
        if self.input.backward {
            self.camera_position -= forward * speed;
        }
        if self.input.left {
            self.camera_position -= right * speed;
        }
        if self.input.right {
            self.camera_position += right * speed;
        }
        if self.input.up {
            self.camera_position += up * speed;
        }
        if self.input.down {
            self.camera_position -= up * speed;
        }
        let sensitivity = 0.002;
        self.camera_yaw += self.input.mouse_delta.0 * sensitivity; // Mouse right decreases yaw
        self.camera_pitch = (self.camera_pitch - self.input.mouse_delta.1 * sensitivity).clamp(-1.5, 1.5); // Mouse down decreases pitch
        self.input.clear_mouse_delta();
        if self.input.toggle_inventory {
            self.cursor_visible = !self.cursor_visible;
            window.set_cursor_visible(self.cursor_visible);
        }
        let view = Mat4::look_at_rh(self.camera_position, self.camera_position + self.get_forward(), Vec3::Y);
        let proj = Mat4::perspective_rh(f32::to_radians(45.0), self.size.width as f32 / self.size.height as f32, 0.1, 100.0);
        let camera_uniform = CameraUniform {
            view_proj: (proj * view).to_cols_array_2d(),
        };
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.pipeline.render(&mut encoder, &view, &self.camera_bind_group);

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
}