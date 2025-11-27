use std::cell::RefCell;

use eframe::egui::{
    containers, Align, CentralPanel, Color32, Context, FontId, Layout, Pos2, RichText, Stroke, StrokeKind, TextStyle, ViewportBuilder, ViewportClass, ViewportId
};

use crate::dochters::{gereedschap::doelscherminfo, globaal::BERICHT};

use super::globaal::{BERICHT_MULTI_SIZE};

///Hiermee wordt het bericht verzonden naar de bericht-functie
pub fn popup_bericht(tekst: &str) {
    //    ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close);  // poging om twee berichten tegelijk te blokkeren
    //    mogelik beter: bericht veranderen in zelfde bericht, maar hoe?
    //    testen: is actief??  (door te kijken hoe het bericht nu is ...) dan alleen bericten veranderen
    //    door in uitsnede 166 de handeliding te blokkeren wordt het bericht 'wachten' zichtbaar (eindelijk!)
    //    println!("SINGLE  {}", tekst);
    if tekst.len() == 0 {
        BERICHT.set(vec![]);
    } else {
        BERICHT.set(vec![tekst.to_string()]);
    }
}

pub fn _popup_multi_string(lijst: Vec<String>) {  // veel-regel-bericht aangeleverd als vector van Strings; niet in gebruik
    BERICHT.set(lijst.clone());
}

pub fn popup_multi_str(lijst: &[&str]) {            // veel regel-bericht aangeleverd als array van &str; afkomstig uit constanten in globaal.rs
    let mut lst: Vec<String> = Vec::new();
    for s in lijst {
        lst.push(s.to_string());
    }
    BERICHT.set(lst);
}

/*
pub fn popup_direct(ctx: Context, tekst: &str) {   // dit werkt niet; het moet vanuit update !!
    if tekst.len()== 0 {return};
    popup_bericht(ctx.clone(), tekst);
    bericht_functie(ctx);
}
*/

///De bericht_functie zet het bericht op het scherm in een eigen venster.
pub fn bericht_functie(ctx: Context) -> bool {
    //    if OPSLAAN.get()== true {popup_bericht(ctx.clone(), "")};  // De OPSLAAN-functie zal nieuwe filenamen maken: oude wissen!
    //if OPSLAAN.get()== true {return false;}
    //if OPTIES_ON.get() == false {
    //    return false;
    //}
    //    bericht_altijd(ctx, "")
    //}
    //pub fn bericht_altijd(ctx: Context, altijd_tekst: &str) -> bool {
    //    if altijd_tekst!= "" {
    //        popup_bericht(ctx.clone() , altijd_tekst);
    //    }
    let rc: RefCell<Vec<String>> = BERICHT.with(|v| v.clone()); // krijg de RefCell
    let berichten = rc.into_inner(); // en pak hem uit
    if berichten.len() == 0 {
        return false;
    }
    let mut berichtje = "prr".to_string();
    if berichten.len() == 1 {
        berichtje = berichten[0].clone();
        if berichtje.len() == 0 {
            if BERICHT.with(|v| v.clone()).into_inner().len() == 1 {
                println!("NULVEC")
            }
            //        println!("LEEG >>{}<<", berichten[0].clone());
            return false;
        }
    }
    if berichten.len() == 1 {
        berichtje = berichten[0].clone();
    }
    ctx.show_viewport_deferred(
        //deze functie heeft 3 argumenten nodig: (Viewport_id, ViewportBuilder, vieuwport_ui_cb)
        ViewportId::from_hash_of("bericht_viewport"),
        ViewportBuilder::default()
            .with_title("Popup bericht")
            .with_position(Pos2{ x: doelscherminfo().x().unwrap() as f32 + doelscherminfo().width().unwrap() as f32 - BERICHT_MULTI_SIZE[0] -8.0,
                                     y: doelscherminfo().height().unwrap() as f32 - BERICHT_MULTI_SIZE[1] - 30.0})
            .with_visible(true)
            .with_close_button(berichten.len()<=1)  // meerregel-bericht (= handleiding!) krijgt geen sluit-knop want wordt gesloten met de eerste muisklik
            .with_maximize_button(false)  // deze knop is niet zinvol
            .with_minimize_button(false)  // deze knop is niet zinvol
            .with_decorations(true)
            .with_always_on_top()
            .with_inner_size(BERICHT_MULTI_SIZE),
        move |ctx, class| {
            assert!(
                class == ViewportClass::Deferred,
                "This egui backend doesn't support multiple viewports"
            );
            let frame_stijl = containers::Frame {
                fill: Color32::WHITE,
                stroke: Stroke {
                    width: 1.0,
                    color: Color32::BLACK,
                },
                ..Default::default()
            };
            if ctx.input(|i| i.viewport().close_requested()) {
                BERICHT.set(Vec::new());
            }
            CentralPanel::default().frame(frame_stijl).show(ctx, |ui| {
                //ctx.request_repaint_after_secs(0.2);
                ui.style_mut() //Changes apply to this Ui and its subsequent children
                    .text_styles
                    .insert(
                        TextStyle::Heading,
                        FontId::new(30.0, eframe::epaint::FontFamily::Proportional),
                    );
                ui.style_mut() //Changes apply to this Ui and its subsequent children
                    .text_styles
                    .insert(
                        TextStyle::Button,
                        FontId::new(18.0, eframe::epaint::FontFamily::Proportional),
                    );
                ui.style_mut() //Changes apply to this Ui and its subsequent children
                    .text_styles
                    .insert(
                        TextStyle::Body, // t.b.v. label
                        FontId::new(14.0, eframe::epaint::FontFamily::Proportional),
                    );
                ui.label("");
                if berichten.len() == 1 {
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        ui.heading(&berichtje.clone()); // berichtje komt op scherm, stijl 'Heading'
                    });
                } else {
                    for regel in berichten.clone() {
                        //ui.label("   ".to_string()+ &regel);
                        ui.label(
                            RichText::new("   ".to_string() + &regel)
                                .text_style(TextStyle::Body)
                                .strong()
                                .color(Color32::BLACK),
                        );
                    }
                };
                let painter = ui.painter();
                let stroke = Stroke {
                    width: 5.0,
                    color: Color32::RED,
                };
                painter.rect_stroke(ui.max_rect(), 0.0, stroke, StrokeKind::Inside);
            });
        },
    );
    true
}
