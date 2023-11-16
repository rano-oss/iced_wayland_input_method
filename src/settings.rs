use iced_runtime::command::platform_specific::wayland::{
    layer_surface::SctkLayerSurfaceSettings, window::SctkWindowSettings, 
    input_method_popup::InputMethodPopupSettings,
};

#[derive(Debug)]
pub struct Settings<Flags> {
    /// The data needed to initialize an [`Application`].
    ///
    /// [`Application`]: crate::Application
    pub flags: Flags,
    /// optional keyboard repetition config
    pub kbd_repeat: Option<u32>,
    /// optional name and size of a custom pointer theme
    pub ptr_theme: Option<(String, u32)>,
    /// surface
    pub surface: InitialSurface,
    /// whether the application should exit on close of all windows
    pub exit_on_close_request: bool,
}

#[derive(Debug, Clone)]
pub enum InitialSurface {
    LayerSurface(SctkLayerSurfaceSettings),
    XdgWindow(SctkWindowSettings),
    InputMethodPopup(InputMethodPopupSettings),
    None,
}

impl Default for InitialSurface {
    fn default() -> Self {
        Self::LayerSurface(SctkLayerSurfaceSettings::default())
    }
}
