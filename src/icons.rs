use qmetaobject::{QObject, qt_base_class, qt_property, QString};

pub const ZOOM_IN_ICON : &str = r#"
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-zoom-in"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line><line x1="11" y1="8" x2="11" y2="14"></line><line x1="8" y1="11" x2="14" y2="11"></line></svg>
"#;

pub const ZOOM_OUT_ICON : &str = r#"
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-zoom-out"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line><line x1="8" y1="11" x2="14" y2="11"></line></svg>
"#;

#[derive(QObject)]
pub struct IconProvider {
    base: qt_base_class!(trait QObject),
    zoom_in_icon: qt_property!(QString; CONST),
    zoom_out_icon: qt_property!(QString; CONST),
}

impl Default for IconProvider {
    fn default() -> Self {
        Self {
            base: Default::default(),
            zoom_in_icon: QString::from(ZOOM_IN_ICON),
            zoom_out_icon: QString::from(ZOOM_OUT_ICON),
        }
    }
}