use crate::{dochters::gereedschap::anderscherminfo};
use eframe::egui::{
    containers, Align, CentralPanel, Color32, Context, FontId, Layout, Pos2, Stroke, StrokeKind,
    TextStyle, ViewportBuilder, ViewportClass, ViewportId,
};
use ferrishot_xcap::Monitor;

use super::{
    gereedschap::{Xpos4Monitor, Ypos4Monitor},
    globaal::{ENKELSCHERM, MONITOR_WISSEL},
};

///Als de cursor op het niet-actieve_scherm komt wordt met een bericht-venster gewaarschuwd
///dat het andere scherm beschikbaar is voor schermkopie
pub fn andere_functie(ctx: Context) {
//    if ANDERE_SCHERM.get() == false {
//        return;
//    };
    if MONITOR_WISSEL.get() {  // anders blijft de melding staan bij het wisselen!
        return;
    }
    if Monitor::all().unwrap().len() < 2 || ENKELSCHERM.get() {
        return;
    };
    //if LINKS_BOVEN.get().is_some() && RECHTS_ONDER.get().is_some() {
    //    return;
    //};
    let kort = "Selecteer uit andere scherm\nof kies Optie\n'Andere scherm' !";
    ctx.show_viewport_deferred(
        //deze functie heeft 3 argumenten nodig: (Viewport_id, ViewportBuilder, vieuwport_ui_cb)
        ViewportId::from_hash_of("andere-scherm"),
        ViewportBuilder::default()
            .with_title("Andere Scherm")
            .with_position(Pos2 {
                x: anderscherminfo().xpos() + 200.0,
                y: anderscherminfo().ypos() + 600.0,
            })
            .with_visible(true)
            .with_decorations(false)
            .with_always_on_top()
            .with_inner_size([400.0, 120.0]),
        move |ctx, class| {
            assert!(
                class == ViewportClass::Deferred,
                "This egui backend doesn't support multiple viewports"
            );
            let frame_stijl = containers::Frame {
                fill: Color32::LIGHT_BLUE,
                ..Default::default()
            };
            CentralPanel::default().frame(frame_stijl).show(ctx, |ui| {
                ui.style_mut() //Changes apply to this Ui and its subsequent children
                    .text_styles
                    .insert(
                        TextStyle::Heading,
                        FontId::new(30.0, eframe::epaint::FontFamily::Proportional),
                    );
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    //ui.label("\n");
                    ui.heading(&*kort);
                });
                let painter = ui.painter();
                let stroke = Stroke {
                    width: 5.0,
                    color: Color32::RED,
                };
                painter.rect_stroke(ui.max_rect(), 0.0, stroke, StrokeKind::Inside);
            });
        },
    );
}

