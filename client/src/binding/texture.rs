use super::{Binding, BindingLayout};

pub struct TextureBindingLayout {
    binding: u32,
    visibility: wgpu::ShaderStage,

    size: wgpu::Extent3d,

    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsage,
}

impl TextureBindingLayout {
    pub fn new_sampled_output(
        binding: u32,
        visibility: wgpu::ShaderStage,
        size: wgpu::Extent3d,
        format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            binding,
            visibility,
            size,

            format,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
        }
    }
}

impl BindingLayout<TextureBinding> for TextureBindingLayout {
    fn get_entry(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: self.binding,
            visibility: self.visibility,
            ty: wgpu::BindingType::SampledTexture {
                multisampled: false,
                component_type: wgpu::TextureComponentType::Float,
                dimension: wgpu::TextureViewDimension::D2Array,
            },

            count: None,
        }
    }

    fn create_binding(&self, device: &wgpu::Device) -> TextureBinding {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: self.size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,

            format: self.format,
            usage: self.usage,
        });

        TextureBinding {
            texture_view: texture.create_view(&wgpu::TextureViewDescriptor {
                label: None,
                format: None,
                dimension: Some(wgpu::TextureViewDimension::D2),
                aspect: wgpu::TextureAspect::All,
                base_mip_level: 0,
                level_count: None,
                base_array_layer: 0,
                array_layer_count: None,
            }),
        }
    }
}

/*--------------------------------------------------------------------------------------------------*/

pub struct TextureBinding {
    texture_view: wgpu::TextureView,
}

impl Binding for TextureBinding {
    fn get_resource(&self) -> wgpu::BindingResource {
        wgpu::BindingResource::TextureView(&self.texture_view)
    }
}
