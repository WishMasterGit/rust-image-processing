use std::sync::Arc;
use winit::{
    window::{Window},
};
pub(crate) struct State {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
}
impl State {
    pub async fn new(window: Arc<Window>) -> State {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();
        let size = window.inner_size();
        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];
        let state = State {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
        };
        state.configure_surface();
        state
    }
    pub fn get_window(&self) -> &Window {
        &self.window
    }
    pub fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }
    pub fn resize(&mut self, new_size:winit::dpi::PhysicalSize<u32>){
        self.size = new_size;
        self.configure_surface();
    }
    pub fn render(&mut self){
        let surf_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next surface texture");
        let texture_view = surf_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor{
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations{
                    load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                    store: wgpu::StoreOp::Store,
                }
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None
        });
        drop(renderpass);
        self.queue.submit(Some(encoder.finish()));
        self.window.pre_present_notify();
        surf_texture.present();
    }
}
