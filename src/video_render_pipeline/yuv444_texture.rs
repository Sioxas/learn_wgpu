use anyhow::*;

pub struct ColorComponentTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub size: wgpu::Extent3d,
}

impl ColorComponentTexture {
    pub fn new(
        device: &wgpu::Device,
        label: Option<&str>,
        dimensions: (u32, u32),
    ) -> Self {
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            view,
            size
        }
    }
}

pub struct VideoYUV444Texture {
    pub component_y: ColorComponentTexture,
    pub component_u: ColorComponentTexture,
    pub component_v: ColorComponentTexture,
    pub sampler: wgpu::Sampler,
    pub size: wgpu::Extent3d,
}

impl VideoYUV444Texture {
    pub fn new(
        device: &wgpu::Device,
        label: Option<&str>,
        dimensions: (u32, u32),
    ) -> Self {
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let component_y = ColorComponentTexture::new(&device, Some(format!("{}componentY", label.unwrap()).as_str()), dimensions);
        let component_u = ColorComponentTexture::new(&device, Some(format!("{}componentU", label.unwrap()).as_str()), dimensions);
        let component_v = ColorComponentTexture::new(&device, Some(format!("{}componentV", label.unwrap()).as_str()), dimensions);

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            component_y,
            component_u,
            component_v,
            sampler,
            size
        }
    }

    pub fn update(&self, queue: &wgpu::Queue, data: &[u8]) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.component_y.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(self.size.width),
                rows_per_image: std::num::NonZeroU32::new(self.size.height),
            },
            self.size,
        );

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.component_u.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            data,
            wgpu::ImageDataLayout {
                offset: (self.size.width * self.size.height) as u64,
                bytes_per_row: std::num::NonZeroU32::new(self.size.width),
                rows_per_image: std::num::NonZeroU32::new(self.size.height),
            },
            self.size,
        );

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.component_v.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            data,
            wgpu::ImageDataLayout {
                offset: (self.size.width * self.size.height * 2) as u64,
                bytes_per_row: std::num::NonZeroU32::new(self.size.width),
                rows_per_image: std::num::NonZeroU32::new(self.size.height),
            },
            self.size,
        );
    }
}
