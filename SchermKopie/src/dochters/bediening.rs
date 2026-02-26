use std::cell::Cell;

use eframe::egui::{
    Align, CentralPanel, Color32, Context, FontId, Layout, Pos2, RichText, TextStyle, ViewportBuilder, containers
};

use crate::dochters::{gereedschap::hoofdscherm, globaal::{
        BEELD_OPTION_TEXTURE, GESTART, Gestart, MENU_TYPE, MenuType, OPSLAAN, OPSLAAN_VENSTER, OPSLAAN_VRAAG, OPTIES_ON, RETOUR_NAAR_UITSNEDE, SCHOONMAKEN 
    }};

use super::{
    gereedschap::MonitorFunctions,
    globaal::{BERICHT, MAAK_UITSNEDE},
};

thread_local! {   // benaderen met NAAM.get()  of NAAM.set(value) - heeft in elke thread een eigen waarde
    pub static RESULT_OPSLAAN: Cell<bool> = Cell::new(false);
}

pub fn bediening_functie(ctx: Context) -> bool {
    // Deze zelfde functie wordt, met enkele wijzigingen, zoals een andere achtergrondkleur, gebruikt in uitsnede-app en ook in pijlen-functie
    if GESTART.get()!=Gestart::Running {
        if GESTART.get()== Gestart::Tweede && MAAK_UITSNEDE.get()==true {
            println!("BEDIENING VENSTER    ronde: {:?}  {:4.1}  {:4.1}", GESTART.get(), hoofdscherm().scale(), ctx.pixels_per_point());
            println!("In de eerste ronde zijn schaal en pixels_per_point ongelijk, en worden afmetingen en plaats van bediening onjuist");
            println!("Na de eerste ronde zijn schaal en pixels_per_point gelijk geworden; daarom wordt bediening iin de eerste ronde overgeslagen" );
            println!("Deze informatie is nodig bij verder ontwikkelen, daarom wordt het hier weergegeven");
        } else {
            return false;
        }
    }
    RESULT_OPSLAAN.set(false);
    if SCHOONMAKEN.get() {return false};
    if MENU_TYPE.get() == MenuType::Rand {   // optie 'rand' gekozen
        return RESULT_OPSLAAN.get();         // false !!
    };
    if OPSLAAN_VRAAG.get() {
        BERICHT.set(vec![]); // zet 'Handleiding' uit
    }
    ctx.request_repaint(); // anders werkt 'optie aan-uit' pas nadat de muis weer buiten het window 'opslaan-of-stoppern' komt
    let schot = MAAK_UITSNEDE.get() == true; // zelf.uitsnede_viewport.load(Ordering::SeqCst);
    let innersize = [300.0, 160.0];
    let positie= Pos2{
        x: (hoofdscherm().xpos() + hoofdscherm().xsize())/ ctx.pixels_per_point() - innersize[0]- 2.0,
        y: hoofdscherm().ypos()/ hoofdscherm().scale()
    };
    //println!("BD {:?} {:?} {:?} {:?} {:?}", positie, hoofdscherm().xpos() , hoofdscherm().xsize(), hoofdscherm().scale(), ctx.pixels_per_point() );
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
                if GESTART.get()!=Gestart::Running {println!("CPppp  {:2.2}",  ctx.pixels_per_point());}
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
                        OPSLAAN_VRAAG.set(false);
                        if MAAK_UITSNEDE.get() {
                            OPSLAAN.set(true); // uitsnede_app: scherm hoeft niet schoon, want beeld gebruikt uit geheugen dus zonder overlay van opties etc
                        } else {
                            SCHOONMAKEN.set(true); // pijlen_app moet eerst scherm schoonmaken van opties en opslaan-vraag-venster
                        }
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
                            RESULT_OPSLAAN.set(true);       // hiermee krijgt uitsnede te horen dat de eerdere uitsnede kan worden gewist
                            MAAK_UITSNEDE.set(true);        // we blijven in of gaan terug naar uitsnede
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
    RESULT_OPSLAAN.get() // true betekent in uitsnede: uitsnede wordt gewist (links_boven=None en rechts_onder=None)
}
