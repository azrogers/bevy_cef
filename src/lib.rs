#![allow(clippy::type_complexity)]

mod common;
mod cursor_icon;
mod keyboard;
mod mute;
mod navigation;
mod system_param;
mod webview;
mod zoom;

use std::path::PathBuf;

use crate::common::{LocalHostPlugin, MessageLoopPlugin, WebviewCoreComponentsPlugin};
use crate::cursor_icon::SystemCursorIconPlugin;
use crate::keyboard::KeyboardPlugin;
use crate::mute::AudioMutePlugin;
use crate::prelude::{IpcPlugin, NavigationPlugin, WebviewPlugin};
use crate::zoom::ZoomPlugin;
use bevy::prelude::*;
use bevy_remote::RemotePlugin;

pub mod prelude {
    pub use crate::{CefPlugin, RunOnMainThread, common::*, navigation::*, webview::prelude::*};
    pub use bevy_cef_core::prelude::execute_render_process;
}

pub struct RunOnMainThread;

#[derive(Default)]
pub struct CefConfig {
    pub browser_subprocess_path: Option<PathBuf>,
}

#[derive(Default)]
pub struct CefPlugin {
    pub config: CefConfig,
}

impl Plugin for CefPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LocalHostPlugin,
            MessageLoopPlugin::new(&self.config),
            WebviewCoreComponentsPlugin,
            WebviewPlugin,
            IpcPlugin,
            KeyboardPlugin,
            SystemCursorIconPlugin,
            NavigationPlugin,
            ZoomPlugin,
            AudioMutePlugin,
        ));
        if !app.is_plugin_added::<RemotePlugin>() {
            app.add_plugins(RemotePlugin::default());
        }
    }
}
