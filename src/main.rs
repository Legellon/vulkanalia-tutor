#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::{anyhow, Result};
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::window as vk_window;
use vulkanalia::prelude::v1_2::*;
use vulkanalia::vk::{InstanceCreateFlags, KHR_PORTABILITY_ENUMERATION_EXTENSION};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() -> Result<()> {
    pretty_env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    let mut app = unsafe { App::create(&window)? };
    let mut destroying = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared if !destroying =>
                unsafe { app.render(&window) }.unwrap(),

            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe { app.destroy(); }
            },

            _ => {}
        }
    });
}

#[derive(Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance,
}

impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let instance = create_instance(window, &entry)?;

        Ok(Self { entry, instance })
    }

    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    unsafe fn destroy(&mut self) {
        self.instance.destroy_instance(None);
    }
}

#[derive(Clone, Debug, Default)]
struct AppData {}

unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Vulkan Tutorial\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    let mut extensions: Vec<_> = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect();

    extensions.push(KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());

    let info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .flags(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR)
        .enabled_extension_names(&extensions);

    Ok(entry.create_instance(&info, None)?)
}