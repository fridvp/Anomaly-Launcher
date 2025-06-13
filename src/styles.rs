use eframe::egui::{
    style::{WidgetVisuals, Widgets},
    Color32, Stroke, Visuals,
};

pub struct Styles;
impl Styles {
    /*pub fn light() -> Visuals {
        Visuals {
            dark_mode: false,
            override_text_color: Some(Color32::from_rgb(25, 25, 25)),
            window_fill: Color32::from_rgb(240, 240, 240),
            widgets: Widgets {
                hovered: WidgetVisuals {
                    bg_stroke: Stroke::NONE,
                    ..Visuals::light().widgets.hovered
                },
                active: WidgetVisuals {
                    bg_stroke: Stroke::NONE,
                    ..Visuals::light().widgets.hovered
                },
                ..Visuals::light().widgets
            },
            ..Visuals::light()
        }
    }*/

    pub fn dark() -> Visuals {
        Visuals {
            dark_mode: false,
            override_text_color: Some(Color32::from_rgb(225, 225, 225)),
            //window_fill: Color32::from_rgb(30, 30, 30),
            //window_fill: Color32::TRANSPARENT,
            //panel_fill: Color32::TRANSPARENT,
            //window_fill: Color32::from_rgba_premultiplied(0, 0, 0, 0),  // Полная прозрачность
            //panel_fill: Color32::from_rgb(25, 25, 25),
            widgets: Widgets {
                hovered: WidgetVisuals {
                    bg_stroke: Stroke::NONE,
                    ..Visuals::dark().widgets.hovered
                },
                active: WidgetVisuals {
                    bg_stroke: Stroke::NONE,
                    ..Visuals::dark().widgets.hovered
                },
                ..Visuals::dark().widgets
            },
            
            ..Visuals::dark()
        }
    }
}
