mod yuv444_texture;
#[path ="../vertex.rs"]
mod vertex;
use vertex::Vertex;

pub struct VideoRenderPipeline {
  pub render_pipeline: wgpu::RenderPipeline,
  pub bind_group: wgpu::BindGroup,
  pub texture: yuv444_texture::VideoYUV444Texture,
}

impl VideoRenderPipeline {
  pub fn new(
    device: &wgpu::Device,
    label: Option<&str>,
    config: &wgpu::SurfaceConfiguration,
    dimensions: (u32, u32),
  ) -> Self {
    
    let texture = yuv444_texture::VideoYUV444Texture::new(&device, label, dimensions);

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("Video Render Pipeline Bind Group Layout"),
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
          },
          count: None,
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
          },
          count: None,
        },
        wgpu::BindGroupLayoutEntry {
          binding: 2,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
          },
          count: None,
        },
        wgpu::BindGroupLayoutEntry {
          binding: 3,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Sampler(
            wgpu::SamplerBindingType::Filtering,
          ),
          count: None,
        },
      ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("Video Render Pipeline Bind Group"),
      layout: &bind_group_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&texture.component_y.view),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::TextureView(&texture.component_u.view),
        },
        wgpu::BindGroupEntry {
          binding: 2,
          resource: wgpu::BindingResource::TextureView(&texture.component_v.view),
        },
        wgpu::BindGroupEntry {
          binding: 3,
          resource: wgpu::BindingResource::Sampler(&texture.sampler),
        },
      ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Video Render Pipeline Layout"),
      bind_group_layouts: &[&bind_group_layout],
      push_constant_ranges: &[],
    });

    let shader = device.create_shader_module(&wgpu::include_wgsl!("yuv444_shader.wgsl"));

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Video Render Pipeline"),
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main", // 1.
        buffers: &[
            Vertex::desc(),
        ], // 2.
      },
      fragment: Some(wgpu::FragmentState { // 3.
          module: &shader,
          entry_point: "fs_main",
          targets: &[wgpu::ColorTargetState { // 4.
              format: config.format,
              blend: Some(wgpu::BlendState{
                color: wgpu::BlendComponent{
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,},
                alpha: wgpu::BlendComponent::OVER
            }),
              write_mask: wgpu::ColorWrites::ALL,
          }],
      }),
      primitive: wgpu::PrimitiveState {
          topology: wgpu::PrimitiveTopology::TriangleList, // 1.
          strip_index_format: None,
          front_face: wgpu::FrontFace::Ccw, // 2.
          cull_mode: Some(wgpu::Face::Back),
          // 如果将该字段设置为除了 Fill 之外的任何值，都需要 Features::NON_FILL_POLYGON_MODE
          polygon_mode: wgpu::PolygonMode::Fill,
          // 需要 Features::DEPTH_CLIP_ENABLE
          unclipped_depth: false,
          // 需要 Features::CONSERVATIVE_RASTERIZATION
          conservative: false,
      },
      depth_stencil: None, // 1.
      multisample: wgpu::MultisampleState {
          count: 1, // 2.
          mask: !0, // 3.
          alpha_to_coverage_enabled: false, // 4.
      },
      multiview: None, // 5.
    });

    Self{
      render_pipeline,
      bind_group,
      texture,
    }
  }
}
