use wgpu::include_wgsl;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::window::Window;
use wgpu::util::DeviceExt;
use image::GenericImageView;
use std::time::Instant;
use std::fs;


mod video_render_pipeline;
use video_render_pipeline::VideoRenderPipeline;

mod vertex;
use vertex::Vertex;


const VERTICES0: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
];

const INDICES0: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

const VERTICES1: &[Vertex] = &[
    Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 0.0], }, // A
    Vertex { position: [0.7, 1.0, 0.0], tex_coords: [1.0, 0.0], }, // B
    Vertex { position: [0.7, -0.7, 0.0], tex_coords: [1.0, 1.0], }, // C
    Vertex { position: [-1.0, -0.7, 0.0], tex_coords: [0.0, 1.0], }, // D
];

const INDICES1: &[u16] = &[
    0, 2, 1,
    0, 3, 2,
];

fn create_model(device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Model {
    Model {
        vertex_buffer: device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        ),
        num_vertices: vertices.len() as u32,
        index_buffer: device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        ),
        num_indices: indices.len() as u32,
    }
}

struct Model{
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    num_vertices: u32,
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    color: wgpu::Color,
    models: Vec<Model>,
    current_model: usize,
    current_frame_index: u32,
    video_render_pipeline: VideoRenderPipeline,
}

impl State {
    // 某些 wgpu 类型需要使用异步代码才能创建
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // instance 变量是到 GPU 的 handle
        // Backends::all 对应 Vulkan + Metal + DX12 + 浏览器的 WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // 是否追踪 API 调用路径
        ).await.unwrap();

        // let diffuse_image = image::open("meme.jpg").unwrap();
        // let diffuse_texture = video_texture::VideoTexture::new(&device, Some("meme video"), (650, 487)).unwrap();
        
        

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        let video_render_pipeline = VideoRenderPipeline::new(&device, Some("YUV_Video_Pipeline"), &config, (650, 487));

        let color = wgpu::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };

        let models = vec![
            create_model(&device, &VERTICES0, &INDICES0),
            create_model(&device, &VERTICES1, &INDICES1),
        ];

        Self {
            surface,
            device,
            queue,
            config,
            size,
            color,
            models: models,
            current_model: 1,
            current_frame_index: 1,
            video_render_pipeline
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        
        // todo!()
        // if(self.current_frame_index == 50){
        //     self.current_frame_index = 1;
        // } else {
        //     self.current_frame_index += 1;
        // }
        // let file_name = format!("frames/ezgif-frame-{:0>3}.jpg", self.current_frame_index);
        // println!("file_name: {}", file_name);
        // let frame = image::open(file_name).unwrap();
        // frame.
        let yuv444frame = fs::read("assets/meigui_yuv_444.yuv").unwrap();
        let start = Instant::now();
        // let rgba8 = frame.to_rgba8();
        let elapsed = start.elapsed();
        println!("Millis: {} ms", elapsed.as_millis());      
        self.video_render_pipeline.texture.update(&self.queue, &yuv444frame);    
    }

    fn toogle_model(&mut self) {
        // self.current_model = (self.current_model + 1) % self.models.len();
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            let model = &self.models[self.current_model];
            render_pass.set_pipeline(&self.video_render_pipeline.render_pipeline);
            render_pass.set_bind_group(0, &self.video_render_pipeline.bind_group, &[]);
            render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
            render_pass.set_index_buffer(model.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..model.num_indices, 0, 0..1);
        }
    
        // submit 方法能传入任何实现了 IntoIter 的参数
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let scale_factor = window.scale_factor();
    println!("scale_factor: {}", scale_factor);
    let mut state = pollster::block_on(State::new(&window));
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if !state.input(event) { // UPDATED!
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { 
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Space),
                            ..
                        },
                        ..
                    } => {
                        // state.toogle_model();
                        state.update();
                    }
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    WindowEvent::CursorMoved { device_id, position, modifiers } => {
                        state.color = wgpu::Color {
                            r: position.x as f64 / state.size.width as f64,
                            g: position.y as f64 / state.size.height as f64,
                            b: 0.0,
                            a: 1.0,
                        };
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // state.update();
                match state.render() {
                    Ok(_) => {}
                    // 如果发生上下文丢失，就重新配置 surface
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // 系统内存不足，此时应该退出
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // 所有其他错误（如过时、超时等）都应在下一帧解决
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // 除非手动请求，否则 RedrawRequested 只会触发一次
                window.request_redraw();
            }
            _ => {}
        }
    });
}