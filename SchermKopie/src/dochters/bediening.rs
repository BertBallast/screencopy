use std::cell::Cell;

use eframe::egui::{
    containers, Align, CentralPanel, Color32, Context, FontId, Layout, Pos2, RichText, TextStyle,
    ViewportBuilder,
};

use crate::dochters::{
    globaal::{
        MenuType, BEELD_OPTION_TEXTURE, MENU_TYPE, OPSLAAN, OPSLAAN_VENSTER, OPSLAAN_VRAAG,
        OPTIES_ON, RETOUR_NAAR_UITSNEDE, SCHOONMAKEN,
    },
    uitsnede_app::naar_uitsnede,
};

use super::{
    gereedschap::{
        anderscherminfo, doelscherminfo, Scale4Monitor, Xpos4Monitor, Xsize4Monitor, Ypos4Monitor,
        Ysize4Monitor,
    },
    globaal::{BERICHT, MAAK_UITSNEDE},
};

thread_local! {   // benaderen met NAAM.get()  of NAAM.set(value) - heeft in elke thread een eigen waarde
pub static RESULT_OPSLAAN: Cell<bool> = Cell::new(false);
}

pub fn bediening_functie(ctx: Context, centrum: bool) -> bool {
    RESULT_OPSLAAN.set(false);
    if MENU_TYPE.get() == MenuType::Rand
    // optie rand_type menu gekozen
    {
        //|| uitsnede_niet_klaar {   // nog geen uitsnede gemaakt
        return RESULT_OPSLAAN.get();
    };

    if OPSLAAN_VRAAG.get() {
        BERICHT.set(vec![]); // zet 'Handleiding' uit
    }
    ctx.request_repaint(); // anders werkt 'optie aan-uit' pas nadat de muis weer buiten het window 'opslaan-of-stoppern' komt
    let schot = MAAK_UITSNEDE.get() == true; // zelf.uitsnede_viewport.load(Ordering::SeqCst);
    let innersize = [300.0, 160.0];
    let _positie = Pos2 {
        x: if centrum {
            doelscherminfo().xpos() + (doelscherminfo().xsize() - innersize[0]) / 2.0
        } else {
            doelscherminfo().xpos() + doelscherminfo().xsize() - innersize[0]
        } / anderscherminfo().scale()
            - 12.0,
        y: if centrum {
            doelscherminfo().ypos() + (doelscherminfo().ysize() - innersize[1]) / 2.0
        } else {
            doelscherminfo().ypos()
        } / anderscherminfo().scale(),
    };
    //dit leverde de afwijkende maten van X11 op
    //println!("DSI {:?}  {:?}  == {:?}  {:?}  == {:?}  {:?} {:?}", doelscherminfo(), positie,
    //doelscherminfo().xsize(), doelscherminfo().xpos(), doelscherminfo().ysize(), doelscherminfo().ypos(), doelscherminfo().scale());
    let positie = Pos2 {
        x: 1000.0,
        y: 100.0,
    };
    ctx.show_viewport_deferred(
        //deze functie heeft 3 argumenten nodig: (Viewport_id, ViewportBuilder, vieuwport_ui_cb)
        OPSLAAN_VENSTER.get(),
        ViewportBuilder::default()
            .with_position(positie)
            .with_title("Opslaan of stoppen")
            .with_visible(true)
            .with_always_on_top()
            .with_decorations(true)
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_close_button(false)
            .with_inner_size(innersize),
        move |ctx, _class| {
            let frame_stijl = containers::Frame {
                fill: if schot {
                    Color32::DARK_GREEN
                } else {
                    Color32::RED
                },
                ..Default::default()
            };
            CentralPanel::default().frame(frame_stijl).show(ctx, |ui| {
                ui.style_mut() //Changes apply to this Ui and its subsequent children
                    .text_styles
                    .insert(
                        TextStyle::Button,
                        FontId::new(14.0, eframe::epaint::FontFamily::Proportional),
                    );
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.label("");
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::BLUE;
                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::RED;
                    if ui
                        .button(
                            RichText::new(if schot {
                                "Selectie (opslaan en) bewerken"
                            } else {
                                "Bewerkte afbeelding opslaan"
                            })
                            .color(Color32::WHITE)
                            .strong(),
                        )
                        .clicked()
                    {
                        //popup_wachten();
//                        ctx.request_repaint_of(OPSLAAN_VENSTER.get()); // is dit nodig??
                        OPSLAAN_VRAAG.set(false);
                        if MAAK_UITSNEDE.get() {
                            OPSLAAN.set(true); // uitsnede_app: scherm hoeft niet schoon, want beeld gebruikt uit geheugen dus zonder overlay van opties etc
                        } else {
                            SCHOONMAKEN.set(true); // pijlen_app moet eerst scherm schoonmaken van opties en opslaan-vraag-venster
                        }
                        //unsafe { winapi::um::winuser::SetCursorPos(1200,1200) };
                        //                        if schot {opslaan_als_scherm_schoon(ctx.clone());}
                        //                        else {opslaan_juiste_rechthoek(RECHTHOEK_OPSLAAN.get());}
                        //                        PIJL_FUNCTIE.set(true);
                        //                        naar_pijlen(ctx.clone());
                    }
                    ui.label("");
                    if schot {
                        if ui
                            .button(
                                RichText::new("Opslaan, en nog een uitsnede kiezen")
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .clicked()
                        {
                            OPSLAAN_VRAAG.set(false);
                            OPSLAAN.set(true);
                            //opslaan_als_scherm_schoon(ctx.clone());
                            RETOUR_NAAR_UITSNEDE.set(true);
                            //                            BEELD_TEXTURE.set(None);
                            //                            SCHOON.set(false);   // nieuw
                        }
                    }
                    if ui
                        .button(
                            RichText::new(
                                //    if schot {
                                "Kies ander scherm-gedeelte", //} else { "Niet opslaan, kies ander scherm-gedeelte"}
                            )
                            .color(Color32::WHITE)
                            .strong(),
                        )
                        .clicked()
                    {
                        if MAAK_UITSNEDE.get() == false {
                            BEELD_OPTION_TEXTURE.set(None);
                            RETOUR_NAAR_UITSNEDE.set(true); // gebeurt in pijlen_app.update
                        } else {
                            RESULT_OPSLAAN.set(true);
                            MAAK_UITSNEDE.set(true);
                        }
                    };
                    ui.label("");
                    if ui
                        .button(
                            RichText::new("Optie-scherm AAN/UIT")
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .clicked()
                    {
                        OPTIES_ON.set(OPTIES_ON.get() == false);
                    };
                    if ui
                        .button(
                            RichText::new("Programma sluiten")
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .clicked()
                    {
                        std::process::exit(0);
                    };
                });
            });
        },
    );
//    if RETOUR_NAAR_UITSNEDE.get() {
//        naar_uitsnede(ctx.clone());
//        RETOUR_NAAR_UITSNEDE.set(false);
//        // dit betekent dat we na opslaan NIET terugkeren naar uitsnede!
//    }
    RESULT_OPSLAAN.get() // true betekent in uitsnede: uitsnede wordt gewist (links_boven=None en rechts_onder=None)
}
