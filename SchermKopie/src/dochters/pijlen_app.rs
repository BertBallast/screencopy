use eframe::egui::{
    self, Color32, Context, Id, Key, LayerId, Order, Painter, Pos2, Rect, Stroke, StrokeKind,
    ViewportBuilder,
};
use eframe::epaint::TextureHandle;
use image::imageops::FilterType;
use image::DynamicImage;
use std::ffi::OsString;

use crate::dochters::bericht_venster::{bericht_functie, popup_bericht, popup_multi_str};
use crate::dochters::gereedschap::Knop;
use crate::dochters::globaal::{
    HandleidingGebruik, MenuType, BEELD_OPTION_TEXTURE, BERICHT, HANDLEIDING_GEBRUIK,
    HANDLEIDING_PIJLEN, MENU_POS, MENU_TYPE, RECHTHOEK_OPSLAAN, SCHOONMAKEN, TEXT_INPUT,
};
use crate::dochters::text_input::text_inp;
use crate::dochters::{
    gereedschap::{
        bewaar_scherm_regio, color_image_to_image_buffer, dichtbij, forceer_in_rect,
        grootstescherm, image_buffer_to_color_image, load_image_from_path, maak_pijl_naam,
        naam_zonder_pad, pijltonen, plaats_in_rect, plaats_in_scherm, EigenTrait4LocalKey,
        PijlWijziging, Pijltje,
    },
    globaal::{
        BEELD_SELECTIE, BESTAAND_BEELD, DOELSCHERM, HEEL_SCHERM, KLEUR, LAATSTEBEELD, OPSLAAN,
        OPTIES_ON, PIJLENKOKER, PIJL_EDIT, PIJL_NUMMER, POSITIE_HORZ, POSITIE_VERT, RANDKLEUR,
        RANDNABEWERKEN, VERKLEIN,
    },
};

use super::bediening::bediening_functie;
use super::gereedschap::{Xpos4Monitor, Xsize4Monitor, Ypos4Monitor, Ysize4Monitor};
use super::globaal::{MAAK_UITSNEDE, OPSLAAN_VRAAG, RETOUR_NAAR_UITSNEDE};
use super::optie_venster::optie_functie;
use super::pijlen_editor::edit_pijl;
use super::uitsnede_app::naar_uitsnede;

/*
    De belangrijkste functies van pijlen_app zijn:
    - lees_schot_van_scherm(zelf, ctx.clone())  (de schermafbeelding die in uitsnede-venster is opgeslagen, wordt teruggelezen)
    - bewerk_afbeelding(ctx: Context, beeld_texture: TextureHandle) (die op zijn beurt gebruik maakt van: )
        -- muis_acties_verwerken (ctx.clone(), beeldrect); (die opzijn beurt gebruik maakt van: )
            --- bewaar_scherm_regio (zie gereedschap.rs)
        -- teken_cursor_en_pijlen (ctx.clone(), painter.clone());
    Deze functies zijn in de code gemarkeerd met *****.  Er boven en er onder is een witregel geplaatst
    Ze zijn als aparte functies uitgevoerd om enige structuur aan te brengen
*/

pub fn pijlen_app() {
    // eigenschappen van de te maken viewport worden bepaald
    let grootstescherm = grootstescherm();
    let position = Pos2 {
        x: grootstescherm.xpos(),
        y: grootstescherm.ypos(),
    };
    let windowsize = [grootstescherm.xsize() - 1.0, grootstescherm.ysize() - 1.0]; // -1.0 hier mogelijk niet nodig

    let mut options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_position(position)
            .with_title_shown(false)
            .with_decorations(false)
            .with_title_shown(false)
            .with_fullscreen(true)
            .with_titlebar_shown(false)
            .with_titlebar_shown(false)
            .with_inner_size(windowsize),
        ..Default::default() // overige eigenschappen standaard
    };
    options.run_and_return = true; // na sluiten van de viewport opnieuw openen; nodig wanneer wordt overgegaan naar ean andere monitor
                                   // pijlen_app wordt geactiveerd door het contrueren van een PijlenBeeld;
                                   // de inhoud van pijlen_app zijn hoofd_viewport wrorden bepaald door de update-functie die is geimplementeerd voor de struct PijlenBeeld (zie hieronder),
                                   // totdat de viewport wordt gesloten, zal er verder niets gebeuren dan herhaald updaten van de viewport
    let _ = eframe::run_native(
        // eframe opent de root_viewport
        "Pijlen_app",
        options,
        Box::new(|cc| Ok(Box::new(PijlenApp::new(cc, position)))),
    );
}

pub struct PijlenApp {
    positie: Pos2,
    handleiding_tonen: bool,
    pijl_punt: Option<Pos2>,
    pijl_staart: Option<Pos2>,
    pub pijl_bewerker: PijlWijziging,
}

impl Default for PijlenApp {
    fn default() -> Self {
        Self {
            positie: Pos2 { x: 0.0, y: 0.0 },
            handleiding_tonen: true,
            pijl_punt: None,
            pijl_staart: None,
            pijl_bewerker: PijlWijziging::default(),
        }
    }
}

impl PijlenApp {
    fn new(_cc: &eframe::CreationContext<'_>, position: Pos2) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut default_aangepast = Self::default();
        default_aangepast.positie = position;
        default_aangepast
    }
}

impl eframe::App for PijlenApp {
    /// De update-functie roept de hulp-windows met hun functionaliteiten aan.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if TEXT_INPUT.get() == false && PIJL_EDIT.get() == false {
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Focus); // zorgt dat focus altijd aan is, zo nee dan geen keyboard input - bijv functietoetsen en shortcuts
        }
        text_inp(ctx.clone());
        if OPSLAAN.get() {
            opslaan_juiste_rechthoek(RECHTHOEK_OPSLAAN.get(), self.positie);
        }
        if SCHOONMAKEN.get() == false {
            optie_functie(ctx.clone());
            bediening_functie(ctx.clone(), false); //werkt niet goed meer
            edit_pijl(ctx.clone(), self);
            bericht_functie(ctx.clone());
        }
        if RETOUR_NAAR_UITSNEDE.get() {
            naar_uitsnede(ctx.clone());
        }
        pijlen_functie(self, ctx.clone());
        //if OPSLAAN_VRAAG.get()== true &&  OPSLAAN.get() == false {  // na OPSLAAN-toestemming mag de opslaan functie even niet zichtbaar worden.
    }
}

/// Het pijlen_venster wordt geopend, om pijlen, bijschriften en randen aan een afbeelding toe te kunnen voegen
pub fn pijlen_functie(zelf: &mut PijlenApp, ctx: Context) {
    // lees de te bewerken file in en zet op scherm in een kader, en indien groot fullscreen
    if MAAK_UITSNEDE.get() == true {
        return;
    }
    if HANDLEIDING_GEBRUIK.get() == HandleidingGebruik::Never {
        zelf.handleiding_tonen = false;
    }
    if HANDLEIDING_GEBRUIK.get() == HandleidingGebruik::Always {
        zelf.handleiding_tonen = true;
    }
    //if HANDLEIDING_GEBRUIK.get() == HandleidingGebruik::Protocol {
    // wordt bepaald in muis_acties_vewerken: AAN tot de eerste muis-bediening, dan uit!
    //}
    if SCHOONMAKEN.get() {
        zelf.pijl_bewerker = PijlWijziging::default();
        PIJL_EDIT.set(false);
    }
    if zelf.handleiding_tonen == true {
        popup_multi_str(&HANDLEIDING_PIJLEN); // de vector wordt aangeboden aan een berichtenvenster (zie de file bericht_venster.rs)
    } else {
        if BERICHT.with(|v| v.clone()).borrow().len() > 1 {
            // een bericht vector van meer dan een item (= regels)
            // NB: een enkel item met daarin in aantal malen '\n', nieuwe regel, geeft ook een bericht met meer regels, maar wordt hier niet gewist
            popup_bericht("");
        }
    }
    match lees_opgeslagen_uitsnede(ctx.clone()) {
        // 'lees_opgeslagen_beeld' heeft een Option als resultaat: None of TextureHandle
        None => {
            BESTAAND_BEELD.set(false); // geen beeld gevonden
            naar_uitsnede(ctx.clone()); // naar uitsnede om een uitsnede te maken
        }
        Some(txh) => {
            // bewerk de opgesalgen afbeelding
            bewerk_afbeelding(
                ctx,                   // de context van de viewport
                zelf,                  // PijlenBeeld met wat gegevens en een update_functie
                txh,     // het beeld in de vorm van een texturehandle
                muis_acties_verwerken, // functie: met de muis aan de slag om pijlen en bijschriften te maken
                teken_pijlen,          // functie: en teken die pijlen in
            );
        }
    }
    //    OPSLAAN_VRAAG.set(false); // blijkt overbodig
}

/// De scherm-afbeelding, die met het uitsnede-venster is gemaakt en opgeslagen  als LAATSTEBEELD
/// wordt geopend voor bewerking. De oorspronkelijke schermkopie blijft bewaard, zodat de bewerking opnieuw kan worden gedaan.
fn lees_opgeslagen_uitsnede(ctx: Context) -> Option<TextureHandle> {
    let mut beeld_option = BEELD_SELECTIE.with(|v| v.clone()).into_inner();
    let bestandsnaam = LAATSTEBEELD.get_string();
    // bestandsnaam wordt uit de RefCell 'LAATSTEBEELD' gehaald, omgezet in een OsString, en vervolgens in een Path
    let naam = OsString::from(bestandsnaam.to_string());
    let pad = std::path::Path::new(&naam);
    if VERKLEIN.get() {
        // VERKLEIN kan true worden gemaakt in optie_venster.rs
        beeld_option = None // in dat geval de beeld_option eerst leeg gemaakt
    };
    if beeld_option.is_none() {
        match load_image_from_path(pad) {
            // LAATSTEBEELD wordt ingelezen als ColorImage
            // in deze regel wordt de file van schijf gelezen en 'ge'matched' of dat 'Ok' is of een 'Error'
            Ok(mut kleurplaat) => {
                if VERKLEIN.get() {
                    // het beeld wordt opgehaald van LAATSTEBEELD en verkleind alvorens verder te werken
                    // niet zo zinvolle en onvoldoende uitgewerkte optie om afbeelding tot 2/3 te verkleinen
                    let mut verkleinen = color_image_to_image_buffer(kleurplaat.clone());
                    verkleinen = DynamicImage::ImageRgba8(verkleinen)
                        .resize(
                            kleurplaat.width() as u32 * 2 / 3,
                            kleurplaat.height() as u32 * 2 / 3,
                            FilterType::Lanczos3,
                        )
                        .into_rgba8();
                    kleurplaat = image_buffer_to_color_image(verkleinen);
                    VERKLEIN.set(false); // bij een volgende update is verder verkleinen ongewenst!
                }
                // als het beeld te groot is voor het scherm ...
                // anders wordt het beeld volledig gevuld, dwz verkleind en in een van beide richtingen uitgerekt tot het past
                let doelscherm = DOELSCHERM.with(|v| v.clone()).into_inner();
                if kleurplaat.width() > doelscherm.xsize() as usize
                    || kleurplaat.height() > doelscherm.ysize() as usize
                // Het beeld is groter dan het scherm: noodzaak om te verkleinen
                {
                    let mut verkleinen = color_image_to_image_buffer(kleurplaat.clone());
                    // kleurplaat (type: Color_Image) wordt omgezet in 'verkeinen' (type: ImageBuffer)
                    verkleinen = DynamicImage::ImageRgba8(verkleinen) // een ImageBuffer kan worden verkleind
                        .resize(
                            doelscherm.xsize().round() as u32,
                            doelscherm.ysize().round() as u32,
                            FilterType::Lanczos3,
                        )
                        .into_rgba8();
                    kleurplaat = image_buffer_to_color_image(verkleinen); // de verkleinde ImageBuffer wordt tergugezet in 'kleurplaat'
                }
                // de al-of-niet verkleinde kleurplaat wordt omgezet in een beeld_option, en gekopieerd naar de BEELD_SELECTIE
                beeld_option = Some(ctx.load_texture("deel-shot", kleurplaat, Default::default()));
                BEELD_SELECTIE.set(beeld_option.clone());
                // de BEELD_SELECTIE wordt bewaard, zodat deze TextureHandle bij een volgende viewport-update snel beschikbaar is:
                // in regel 1 van deze functie gebeurt dat dan
            }
            Err(_foutje) => {
                println!("afbeelding niet op schijf gevonden"); // geen beeld
                RETOUR_NAAR_UITSNEDE.set(true); // (nieuwe) uitsnede maken, dwz pijlen_app verlaten en over naar uitsnede_app
                return None;
            }
        };
    }
    beeld_option // deze laatste regel wordt niet afgesloten met ';'
                 // daarmee wordt deze beeld_option als resultaat van deze functie doorgegeven als Option<TextureHandle>
}

/// De gemaakte scherm-afbeelding wordt als TextureHandle doorgegeven voor bewerking
fn bewerk_afbeelding(
    ctx: Context,                 // Context van de viewport
    zelf: &mut PijlenApp,         // PijlenApp met enkele gegevens en de update-functie
    beeld_texture: TextureHandle, // het beeld als TextureHandle
    muis_acties_verwerken: fn(
        // functie om muisacties te verwerken: pijlen zetten
        ctx: Context,
        zelf: &mut PijlenApp,
        beeldrect: Rect,
        painter: Painter,
    ),
    teken_pijlen: fn(
        // functie die de pijlen daadwerkelijk intekent'
        zelf: &mut PijlenApp,
        ctx: Context,
        beeldrect: Rect,
        painter: Painter,
    ) -> Rect,
) {
    if RETOUR_NAAR_UITSNEDE.get() {
        // elders is aangegeven dan de PijlenApp wordt afgelsoten
        return;
    }
    let painter = ctx.layer_painter(LayerId::new(Order::Background, Id::new("schilderij")));
    // bepaal plaats links-boven van het beeld in het window
    let screen = ctx.content_rect();
    // POSITIE_HORZ==0.0 betekent midden in beeld; -1.0 geheel links; +1.0 geheel rechts - wordt bepaald in opties_venster.rs; POSITIE_VERT idem voor vertikaal
    let mut links =
        (screen.width() - beeld_texture.size()[0] as f32) * (POSITIE_HORZ.get() + 1.0) / 2.0;
    let mut boven =
        (screen.height() - beeld_texture.size()[1] as f32) * (POSITIE_VERT.get() + 1.0) / 2.0;
    // correcties als links-boven buiten de schermranden zou vallen:
    links = links.max(0.0); // niet kleiner dan 0.0              links.max(0.0) = de kleinste van de twee: 'links' en '0.0'
    links = links.min(screen.width()); // niet groter dan schermbreed       links.min(...) idem
    boven = boven.max(0.0); // niet kleiner dan 0.0
    boven = boven.min(screen.height()); // niet groter dan schermhoog
                                        // MAAR:
    if HEEL_SCHERM.get() {
        // als uitsnede==HEEL_SCHERM : linksboven beginnen
        links = 0.0;
        boven = 0.0;
    }

    // bepaal de coordinaten van de afbeelding
    let mut beeldrect = Rect {
        min: Pos2 {
            x: links.clone(),
            y: boven.clone(),
        },
        max: Pos2 {
            x: links + beeld_texture.size()[0] as f32,
            y: boven + beeld_texture.size()[1] as f32,
        },
    };
    // correcties als rechts-onder buiten het scherm valt
    let dsrect = DOELSCHERM.with(|v| v.clone()).into_inner();
    beeldrect.max.x = beeldrect.max.x.min(dsrect.xsize() - 1.0);
    beeldrect.max.y = beeldrect.max.y.min(dsrect.ysize() - 1.0);

    // geef het hele scherm de gewenste randkleur
    painter.rect_filled(
        Rect {
            min: Pos2 { x: 0.0, y: 0.0 },
            max: Pos2 {
                x: ctx.content_rect().max.x,
                y: ctx.content_rect().max.y,
            },
        },
        0.0,
        RANDKLEUR.get(),
    );
    // en plaats het beeld op de juiste plaats in het scherm, standaard in het midden; met POSITIE_HORZ en POSITIE_VERT kan anders worden gekozen
    painter.image(
        beeld_texture.id(),
        beeldrect,
        Rect::from_min_max(Pos2 { x: 0.0, y: 0.0 }, Pos2 { x: 1.0, y: 1.0 }),
        Color32::WHITE,
    );

    // de afbeelding staat nu op het scherm, en kan met schifjes in het optie-venster worden verplaatst

    muis_acties_verwerken(ctx.clone(), zelf, beeldrect, painter.clone());
    beeldrect = teken_pijlen(zelf, ctx.clone(), beeldrect, painter.clone());

    RECHTHOEK_OPSLAAN.set(beeldrect); // een gewijzigde beeldrect teruggekregen van 'teken_pijlen', en vastgeld in RECHTHOEK_OPSLAAN

    if RETOUR_NAAR_UITSNEDE.get() {
        // moet een andere uitsnede worden gemaakt?
        BEELD_OPTION_TEXTURE.set(None); // dan moet deze uitsnede vervallen
    }
}

/// Muis-acties worden geinterpreteerd en omgezet in pijlen met bijschriften, in eerste instantie volgnummers
fn muis_acties_verwerken(ctx: Context, zelf: &mut PijlenApp, beeldrect: Rect, painter: Painter) {
    let screenrect = ctx.content_rect();
    let mut knoppen: Vec<Knop> = vec![];
    let menu_rand = (MENU_TYPE.get() == MenuType::Rand || MENU_TYPE.get() == MenuType::Beide)
        && MENU_POS.get() < 4;
    if SCHOONMAKEN.get() == false {
        if menu_rand {
            // MENU_POS= 4 betekent 'menu nuit tonen'
            knoppen = plaats_menu(painter.clone(), knoppen.clone());
        }
    } else {
        OPSLAAN.set(true);
        SCHOONMAKEN.set(false);
    }
    ctx.input(|k| {
        if k.key_pressed(Key::Escape) {
            if MENU_POS.get() == 4 {
                MENU_POS.set(0)
            } else {
                MENU_POS.set(4)
            };
        }
        if k.pointer.secondary_pressed() {
            std::process::exit(0);
        }
        if k.key_pressed(Key::F4) {
            // Afsluiten
            std::process::exit(0);
        }
        zelf.pijl_staart = None;
        let muis = if let Some(ms) = k.pointer.hover_pos() {
            ms
        } else {
            Pos2 { x: 100.0, y: 100.0 }
        };
        if k.pointer.primary_pressed() {
            // muisknop_Li indrukken wordt verwerkt
            zelf.handleiding_tonen = false;
            if input_menu(zelf, knoppen.clone(), Some(muis)) == true {
                // er is met de menu-knoppen een actie ondernomen
                return; // eerst die afwerken
            } else {
                zelf.pijl_bewerker = dichtbij(plaats_in_rect(muis, beeldrect.min), beeldrect.min);
                // de muis staat dichtbij een bestaande pijl
                if zelf.pijl_bewerker.pijlnummer.is_some() {
                    PIJL_NUMMER.set(Some(zelf.pijl_bewerker.pijlnummer.unwrap()));
                    if zelf.pijl_bewerker.schacht {
                        PIJL_EDIT.set(true);
                    } else {
                        PIJL_EDIT.set(false);
                    }
                    zelf.pijl_bewerker.start_muis = Some(muis);
                } else {
                    zelf.pijl_bewerker = PijlWijziging::default(); // we gaan een nieuwe pijl opstarten, nu nog zonder pijlnummer
                    zelf.pijl_punt = Some(plaats_in_rect(muis, beeldrect.min));
                }
            }
        }
        // muisknop_li loslaten wordt verwerkt
        //if PIJL_WIJZIGING.with(|v| v.clone()).into_inner().is_some() && k.pointer.primary_released()
        if k.pointer.primary_released() {
            let mut knop_gedrukt = false;
            for kn in knoppen {
                if kn.bevat(Some(muis)) {
                    knop_gedrukt = true;
                }
            }
            if knop_gedrukt == false {
                // de wijzig-instructie bij het indrukken van de muisknop wordt hier opgeroepen
                //ctx.request_repaint_of(OPSLAAN_VENSTER.get());  // is dit nodig??
                match zelf.pijl_bewerker.pijlnummer {
                    None => {
                        // nieuwe pijl!
                        zelf.pijl_staart = Some(plaats_in_rect(
                            forceer_in_rect(muis, screenrect),
                            beeldrect.min,
                        ));
                    }
                    Some(pijlnr) => {
                        // wijziging van een van de bestaande pijlen
                        let mut pk = PIJLENKOKER.take();
                        /*
                        POGING om verplaatsen pijlrug te doen met muis in bijschriften dan op de zelfde plek in het bijschrift te blijven
                        dit lukt tenzij de richtng van de pijl daarmee in een ander kwadrant komt
                                                if zelf.pijl_bewerker.schacht || zelf.pijl_bewerker.staart {
                                                    if let Some(pos) = k.pointer.interact_pos() {
                                                        let text_only = pk[pijlnr].tip== pk[pijlnr].rug;
                                                        //let pos_rect= plaats_in_scherm(pk[pijlnr].rug, beeldrect.min);
                                                        //let pos_2= Pos2 {x: pos.x + pk[pijlnr].text_rect., y: pos.y};
                                                        let tp= pk[pijlnr].tip.x>pk[pijlnr].rug.x;
                                                        let rp= pk[pijlnr].tip.y>pk[pijlnr].rug.y;
                                                        let pos_2=
                                                        if zelf.pijl_bewerker.start_muis.is_none() {pos} else {
                                                            Pos2{
                                                                x: pos.x + (if tp {-zelf.pijl_bewerker.start_muis.unwrap().x + pk[pijlnr].text_rect.max.x}
                                                                             else {-zelf.pijl_bewerker.start_muis.unwrap().x + pk[pijlnr].text_rect.min.x}),
                                                                y: pos.y - zelf.pijl_bewerker.start_muis.unwrap().y + (if rp {pk[pijlnr].text_rect.max.y} else {pk[pijlnr].text_rect.min.y})
                                                            }
                                                        };
                                                        pk[pijlnr].rug = plaats_in_rect(
                                                            forceer_in_rect(pos_2, screenrect),
                                                            beeldrect.min,
                                                        );
                                                        if text_only {
                                                            pk[pijlnr].tip= pk[pijlnr].rug;
                                                        }
                                                    }
                                                } else if zelf.pijl_bewerker.punt {
                                                    if let Some(pos) = k.pointer.interact_pos() {
                                                        pk[pijlnr].tip =
                                                            plaats_in_rect(forceer_in_rect(pos, screenrect), beeldrect.min);
                                                    }
                                                }
                        // */
                        //*
                        if zelf.pijl_bewerker.schacht && pk[pijlnr].tip == pk[pijlnr].rug {
                            if let Some(pos) = k.pointer.interact_pos() {
                                //let pos_rect= plaats_in_scherm(pk[pijlnr].rug, beeldrect.min);
                                //let pos_2= Pos2 {x: pos.x + pk[pijlnr].text_rect., y: pos.y};
                                let pos_2 = if zelf.pijl_bewerker.start_muis.is_none() {
                                    pos
                                } else {
                                    Pos2 {
                                        x: pos.x - zelf.pijl_bewerker.start_muis.unwrap().x
                                            + pk[pijlnr].text_rect.min.x,
                                        y: pos.y - zelf.pijl_bewerker.start_muis.unwrap().y
                                            + pk[pijlnr].text_rect.max.y,
                                    }
                                };
                                pk[pijlnr].rug = plaats_in_rect(
                                    forceer_in_rect(pos_2, screenrect),
                                    beeldrect.min,
                                );
                                pk[pijlnr].tip = pk[pijlnr].rug;
                            }
                        } else if zelf.pijl_bewerker.punt {
                            if let Some(pos) = k.pointer.interact_pos() {
                                pk[pijlnr].tip =
                                    plaats_in_rect(forceer_in_rect(pos, screenrect), beeldrect.min);
                            }
                        } else if zelf.pijl_bewerker.staart {
                            if let Some(pos) = k.pointer.interact_pos() {
                                pk[pijlnr].rug =
                                    plaats_in_rect(forceer_in_rect(pos, screenrect), beeldrect.min);
                            }
                        }
                        // */
                        if zelf.pijl_bewerker.punt || zelf.pijl_bewerker.staart {
                            zelf.pijl_bewerker = PijlWijziging::default();
                        }
                        PIJLENKOKER.set(pk.clone());
                    }
                }
            }
        }
        // nieuwe pijl wordt toegevoegd aan verzameling (PIJLENKOKER)
        match zelf.pijl_staart {
            None => {}
            Some(staart) => {
                // pijl-begin is bekend
                match zelf.pijl_punt {
                    None => {}
                    Some(punt) => {
                        // ook pijl_punt is bekend-- met de stelling van pythagoras wordt de pijllengte berekend
                        if ((punt.x - staart.x).powi(2) + (punt.y - staart.y).powi(2)).sqrt() > 6.0
                        {
                            // pijl is lang genoeg om een pijl te zijn
                            let mut pk = PIJLENKOKER.take();
                            pk.push(Pijltje {
                                rug: staart,
                                tip: punt,
                                pijltekst: format!("{}", pk.len() + 1),
                                kleur: KLEUR.get(),
                                text_rect: Rect {
                                    min: Pos2 { x: 0.0, y: 0.0 },
                                    max: Pos2 { x: 10.0, y: 10.0 },
                                }, // dummy!
                            });
                            //OPSLAAN_VRAAG.set(true);
                            PIJLENKOKER.set(pk.clone());
                            zelf.pijl_punt = None;
                        }
                    }
                }
                zelf.pijl_bewerker = PijlWijziging::default()
            }
        }
    }); // input
}

/// Pijlen, bijschriften en cursor-functies worden ingetekend.
/// Pijlen kunnen tot buiten de afbeelding worden getrokken om met name bijschriften ernaast te kunnen plaatsen.
/// Het kader rondom de afbeelding verplaatst zich dan automatisch en bij opslaan van het bewerkte beeld wordt de daarmee vergrote afbeelding geheel opgeslagen.
/// In het optievenmster kan de centrale plaats van de afbeelding wordden verschoven met sliders.
fn teken_pijlen(zelf: &mut PijlenApp, ctx: Context, beeldrect: Rect, painter: Painter) -> Rect {
    if RETOUR_NAAR_UITSNEDE.get() {
        return beeldrect;
    }
    let mut label_rect = beeldrect.clone();
    // label_rect wordt vergroot zodat eventuele naastgelegen pijl-einden en -bijschriften ook binnen de rechthoek vallen
    let mut pk = PIJLENKOKER.take().clone();
    for i in 0..pk.len() {
        //OPSLAAN_VRAAG.set(true);
        let (uiterste, text_rect) =
            pijltonen(zelf, painter.clone(), pk[i].clone(), i, beeldrect.min);
        let extreem = plaats_in_scherm(uiterste, beeldrect.min);
        pk[i].text_rect = text_rect;
        // randmarkeringen ter controle
        //painter.line_segment([Pos2{x: extreem.x-10.0, y: extreem.y-10.0}, Pos2{x: extreem.x+10.0, y: extreem.y+10.0}],Stroke {width: 1.0, color: KLEUR.get(),});
        //painter.line_segment([Pos2{x: extreem.x+10.0, y: extreem.y-10.0}, Pos2{x: extreem.x-10.0, y: extreem.y+10.0}],Stroke {width: 1.0, color: KLEUR.get(),});
        label_rect = Rect {
            min: Pos2 {
                x: f32::min(label_rect.min.x, extreem.x),
                y: f32::min(label_rect.min.y, extreem.y),
            },
            max: Pos2 {
                x: f32::max(label_rect.max.x, extreem.x),
                y: f32::max(label_rect.max.y, extreem.y),
            },
        };
    }
    PIJLENKOKER.set(pk); // terugplaatsen van pijlen, na .take() -- want .take maakt pijlenkoker leeg
    tijdelijke_markeringen(zelf, ctx.clone(), painter.clone(), beeldrect, label_rect)
}

/// Randen, pijl-markeringen e.d. zijn tijdelijk d.w.z. komen niet op het resultaat omdat voor het opslaan een 'schoon' beeld wordt gemaakt.
fn tijdelijke_markeringen(
    zelf: &mut PijlenApp,
    ctx: Context,
    painter: Painter,
    beeldrect: Rect,
    label_rect: Rect,
) -> Rect {
    let mut muis: Option<Pos2> = None;
    ctx.input(|k| {
        muis = k.pointer.hover_pos();
    });
    let wijzig_punt = zelf.pijl_bewerker;
    if SCHOONMAKEN.get() == false && PIJL_EDIT.get() || wijzig_punt.pijlnummer.is_some() {
        // markeer de pijlwijziging met een cirkel op de pijl, hetzij schacht, het zijn punt of staart
        if wijzig_punt.schacht == true {
            let pk = PIJLENKOKER.with(|v| v.clone()).into_inner();
            let pijl = pk[wijzig_punt.pijlnummer.unwrap()].clone();
            if pijl.tip != pijl.rug {
                let pijlpos = Pos2 {
                    x: (pijl.rug.x + pijl.tip.x) / 2.0,
                    y: (pijl.rug.y + pijl.tip.y) / 2.0,
                };
                painter.circle_filled(plaats_in_scherm(pijlpos, beeldrect.min), 10.0, KLEUR.get());
            }
        };
        if wijzig_punt.schacht == false {
            match muis {
                Some(pnt) => {
                    painter.circle_stroke(
                        pnt,
                        5.0,
                        Stroke {
                            width: 2.0,
                            color: KLEUR.get(),
                        },
                    );
                }
                _ => {}
            }
        };
    }
    if SCHOONMAKEN.get() == false {
        painter.rect_stroke(
            Rect {
                min: Pos2 {
                    x: label_rect.min.x - 2.0,
                    y: label_rect.min.y - 2.0,
                },
                max: Pos2 {
                    x: label_rect.max.x + 2.0,
                    y: label_rect.max.y + 2.0,
                },
            },
            0.0,
            Stroke {
                width: 3.0,
                color: KLEUR.get(),
            },
            StrokeKind::Outside,
        );
    }
    label_rect
}

/// Rechthoek van het scherm-met-pijlen dat moet worden opgeslagen
/// Als pijlen en beschriften buiten het eigenlijke beeld vallen wordt een navenant groter beeld opgeslagen, gemarkeerd door een rand die zelf niet wordt opgeslagen.
pub fn opslaan_juiste_rechthoek(rect: Rect, positie: Pos2) {
    //println!("POS {:?}  RECT={:?}", positie, rect);
    let mut rect_corr = rect;
    rect_corr.min.x += positie.x;
    rect_corr.max.x += positie.x;
    rect_corr.min.y += positie.y;
    rect_corr.max.y += positie.y;
    // bestandsnaam wordt bepaald uit LAATSTEBEELD, verwijderen van pad, en indien niet aanwezig toevoeging van -p-
    let mut bestandsnaam = LAATSTEBEELD.get_string();
    bestandsnaam = naam_zonder_pad(&bestandsnaam);
    if !bestandsnaam.contains("-p-") {
        bestandsnaam = maak_pijl_naam(&bestandsnaam);
    }
    bewaar_scherm_regio(rect_corr, bestandsnaam, RANDNABEWERKEN.get() as u32, false);
    OPSLAAN_VRAAG.set(false);
    OPSLAAN.set(false);
    SCHOONMAKEN.set(false);
}

fn plaats_menu(painter: Painter, mut knoppen: Vec<Knop>) -> Vec<Knop> {
    knoppen.push(Knop::new(
        painter.clone(),
        "Opslaan bewerkt <F1>".to_string(),
        knoppen.last(),
    ));
    knoppen.push(Knop::new(
        painter.clone(),
        (if BESTAAND_BEELD.get() == false {
            "Andere selectie"
        } else {
            " - - "
        })
        .to_string(), // als bestaand beeld is nieuwe selectie niet van toepassing
        knoppen.last(),
    ));
    knoppen.push(Knop::new(
        painter.clone(),
        "Opties AAN/UIT <F2>".to_string(),
        knoppen.last(),
    ));
    knoppen.push(Knop::new(
        painter.clone(),
        "Menu verplaatsen <F3>".to_string(),
        knoppen.last(),
    ));
    knoppen.push(Knop::new(
        painter.clone(),
        "Afsluiten <F4>".to_string(),
        knoppen.last(),
    ));
    knoppen
}

fn input_menu(zelf: &mut PijlenApp, knoppen: Vec<Knop>, muis: Option<Pos2>) -> bool {
    if knoppen.len() < 5 {
        return false;
    }
    if knoppen[0].bevat(muis) {
        OPSLAAN_VRAAG.set(false);
        SCHOONMAKEN.set(true);
        zelf.pijl_bewerker = PijlWijziging::default();
        return true;
    } else {
        if knoppen[1].bevat(muis) {
            if BESTAAND_BEELD.get() == false {
                BEELD_OPTION_TEXTURE.set(None);
                RETOUR_NAAR_UITSNEDE.set(true);
            }
        } else {
            if knoppen[2].bevat(muis) {
                OPTIES_ON.set(!OPTIES_ON.get());
                zelf.pijl_bewerker = PijlWijziging::default();
            } else {
                if knoppen[3].bevat(muis) {
                    MENU_POS.set((MENU_POS.get() + 1) % 5);
                    zelf.pijl_bewerker = PijlWijziging::default();
                } else {
                    if knoppen[4].bevat(muis) {
                        std::process::exit(0);
                    }
                }
            }
        }
    }
    false
}
