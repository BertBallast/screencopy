use eframe::{egui::{
    Align2, Color32, Context, CursorIcon, FontId, Id, Key, LayerId, Order, Painter, Pos2, Rect, Stroke, StrokeKind, ViewportBuilder
}, run_native, NativeOptions};

use crate::dochters::{
    andere_scherm::andere_functie, bediening::bediening_functie, bericht_venster::{bericht_functie, popup_bericht, popup_multi_str}, gereedschap::{
        /*struct*/  EigenTrait4LocalKey, Knop, Xpos4Monitor, Xsize4Monitor, Ypos4Monitor, Ysize4Monitor, bewaar_scherm_regio, doelscherminfo, imagebuffer_to_texture, lees_huidige_scherminhoud, line_dashed, muis_en_toets_diversen, rect_dashed
    }, globaal::{
        /*Thread_local statics*/ ANDERE_SCHERM, BEELD_OPTION, BEELD_OPTION_TEXTURE, BEELD_SELECTIE, BERICHT, BESTAAND_BEELD, BESTANDSNAAM, DASHED, DOELSCHERM, HANDLEIDING, HANDLEIDING_GEBRUIK, HANDLEIDING_UITSNEDE, HEEL_SCHERM, HandleidingGebruik, KLEUR, MAAK_UITSNEDE, MENU_POS, MENU_TYPE, MONITOR_WISSEL, MenuType, OPSLAAN, OPSLAAN_VRAAG, OPTIES_ON, PIJLENKOKER, RANDONBEWERKT, RETOUR_NAAR_UITSNEDE, TEXT_INPUT, UITBREIDEN, UITLEG, UITLEG_TWEE 
    }, optie_venster::optie_functie, pijlen_app::pijlen_app, text_input::text_inp
};

/// een 'SchermBeeld' bevat die belangrijkste data van deze functionaliteit, en  wordt als argument aan een aantal functies doorgegeven
pub struct UitsnedeApp {
    pub links_boven: Option<Pos2>,      // hoekpunt van de te maken uitsnede
    pub rechts_onder: Option<Pos2>,     // hoekpunt van de te maken uitsnede
    pub beeld_conversie_gemaakt: bool,  // is de imagebuffer al omgezet in een texturehandle? we hebben beide nodig
    pub handleiding_tonen: bool,        // de handleiding wordt getoond bij het begin van de procedure, maar daarna gestopt om ruimte te maken op het scherm
    muisplek: Option<Pos2>,             // plaats van muis op het scherm
}

impl eframe::App for UitsnedeApp {
    /// De update-functie verzorgt de inhoud van het viewport (window)
    /// de viewport is schermvullend en heeft als achtergrond de texturehandle, en viewport-vriendelijke clone van de image-buffer
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if TEXT_INPUT.get() == false {
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Focus); // zorgt dat focus altijd aan is, zo nee dan geen keyboard input - bijv functietoetsen en shortcuts
            // alleen dan kan in optievenster text_edit_single_line worden gebruikt voor bestandsnaam en in pijledit voor bijschrift
        }
        text_inp(ctx.clone());
        if MONITOR_WISSEL.get() {
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close); // terug naar loop in main om nieuw schermbeeld te maken
        }
        optie_functie(ctx.clone());                 // opties instellen
        bediening_functie(                            // een venster met knoppen 'opslaan', 'afsluiten' etc.; alternatief voor een menu langs de schermrand
            ctx.clone(),
            true,
        );
        andere_functie(ctx.clone());                // wanneer er twee monitors zijn: melding 'andere scherm'op de niet-actieve monitor
        uitsnede_functie(self, ctx.clone());  // het maken van de uitsnede
        opslaan_uitsnede(self, ctx.clone()); // opslaan van de gekozen uitsnede, vanuit de imagebuffer
        bericht_functie(ctx.clone());               // toont een eventueel bericht in een extra venster
    }
    /*
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        // de standaard implementatie van clear_color in eframe::App verzorgt slechts een gedeeltelijke transparantie
        // hier zetten we het alpha-kanaal van Color32 op nul (a= 0) en creeren daarmee volledige transparantie, rood, groen en blauw hebben dan geen effect meer
        Color32::from_rgba_unmultiplied(0, 0, 0, 0).to_normalized_gamma_f32()
    }
    */
}

impl Default for UitsnedeApp {
    fn default() -> Self {
        HANDLEIDING.set(true);  // moet de handledding worden getoond?
        Self {
            beeld_conversie_gemaakt: false, // eenmaal wordt de imagebuffer omgezet in een texturehandle, waarna beide worden bewaard en gebruiukt
            handleiding_tonen: true,        
            links_boven: None,
            rechts_onder: None,
            muisplek: None,
        }
    }
}

impl UitsnedeApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

/// Deze functie maakt een native-viewport
/// Dit gebeurt in de laatste regels van deze functie
/// de inhoud van de viewport wordt gecreerd en bijgewerkt door de functie 'update' die een eigenschap is van 'Schermbeeld'
/// nadat eerst alle gewenst eigenschappen zijn vastgelegd in de variabele 'options'
pub fn uitsnede_app() {
    if niet_klaar_voor_uitsnede() {return};
    let doelscherm = DOELSCHERM.with(|v| v.clone()).into_inner();  // scherm waarop wordt gewerkt
    // variabelen in HOOFDLETTERS zijn gedefinieerd als globale variabelen in globaal.rs
    // als Thread_local! static. Hun waarde krijgen ze met xxxx.set(waarde)
    // Ze worden opgeroepen met xxxx.get() of xxxx.with(....) zoals hierboven
    // het doelscherm (struct Monitor, afkomstig van de crate xcap.rs) bevat alle eigenschappen van de monitor
    // zie verder de toelichting in globaal.rs
    let windowsize = [ // de viewport (egui-woord voor window) wordt even groot als het doelscherm
        doelscherm.clone().xsize() - 1.0, // er gaat iets fout tenzij '- 1.0', **** nazoeken!
        doelscherm.clone().ysize() - 1.0,
    ];
    let position = Pos2 { // als er twee of meer monitors in gebruik zijn staat het doelscherm vaak niet ops (x=0, y=0)
        x: doelscherm.clone().xpos(),
        y: doelscherm.ypos(),
    };
    // wij gebruiken hir een randloos viewport:: 'decorations(false)', maar ter info:
    // eventuele decorations (randen en kopregel; hier niet gebruikt) hebben de volgende afmetingen:
    // + 8.0 en  + 30.0 = correctie voor niet aanwezige randen, wrs alleen nodig bij os = windows
    // in windows registry under windowsmetrics you will find
    // Captionheight = 330
    // Paddedborderwidth = 60
    // beide moet je delen door 15 om het aantal pixels te krijgen + caption= 22, rand = 4 , zpoou dat het verschil zijn??

    // ctx.set_visuals(Visuals::dark()); // verplaatst
    let mut options = NativeOptions {
        // eigenschappen van het root-window worden hier vooraf bepaald: geen decoraties, volledig schermvullend
        // toont een beeld van het scherm waarvan we een uitsnede willen masken, met daaroverheen cursor-lijnen, en enkele hulpvensters
        viewport: ViewportBuilder::default()
            .with_position(position)
            .with_decorations(false)
            .with_inner_size(windowsize),
        ..Default::default() // overige eigenschappen standaard
    };
    options.run_and_return = true; // na sluiten van de viewport NIET ook de applicatie sluiten; nodig wanneer in 'opties' een andere monitor wordt gekozen
    // met bovenstaande 'options' wordt een viewport gemaakt met de structuur van struct 'Schermbeeld'
    // Schermbeeld is hierboven gedefinieerd met een aantal variabelen (zoals de hoekpunten van de te maken uitsnede) en een update functie
    // de update functie bevat de programma-stappen die de inhoud van het viewport bepalen
    // de update functie 'tekent' de viewport steeds opnieuw wanneer de inhoud wordt gewijzigd
    /* de volgende variant eframe::run_simple_native (eenvoudig, nieuwer) werkt niet; we moeten de run_native (ouder, niet deprecated) gebruiken
    let _ = eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        uitsnede_functie(&mut UitsnedeApp::default(),ctx.clone()) ;
    });
    */
     let _ = run_native(  // naam begint met '_' geeft aan dat variabele niet wordt gebruikt
        // eframe opent de root_viewport
        "naam=Uitsnede-app",
        options,
        Box::new(|cc| Ok(Box::new(UitsnedeApp::new(cc)))),
    );
    // alle functionaliteit blijft gebeuren close binnen de viewport -volgens de instructies in de update-functie- , 
    // totdat de viewport wordt gesloten met 'ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close);'
    // pas dan eindigt ook deze functie 'uitsnede_app'
}

fn niet_klaar_voor_uitsnede() -> bool {
    if BESTAAND_BEELD.get() == false 
    && BEELD_OPTION.with(|v|{v.clone()}).into_inner().is_none() {      // geen argument gevonden op de command-line; dus er moet eerst een screenshot worden gemaakt
        lees_huidige_scherminhoud();        // opslaan van een kopie van het gehele scherm in de variabele BEELD_OPTION 
    } else {  // BESTAND_BEELD.get() == true!
        if RETOUR_NAAR_UITSNEDE.get() == false {
            MAAK_UITSNEDE.set(false);
        } else {
              
        }
    }
    if MAAK_UITSNEDE.get() == false {
        pijlen_app();
    }
    MAAK_UITSNEDE.get()== false
}

/// In het uitsnede-venster wordt een schermdeel geselecteerd, en opgeslagen in de directory 'Schermen'op de desktop
/// Het uitsnede venster is schermgroot, heeft als achtergrond een kopie van het schermbeeld waarvan we een uitsnede willen maken
/// op de voorgrond geven we met de muis aan welke uitsnede we willen
/// ook op de voorgrond zijn een paar bedieningsknoppen en -bij het begin- een korte uitleg 
pub fn uitsnede_functie(zelf: &mut UitsnedeApp, ctx: Context) {
    zet_schermkopie_in_viewport(zelf, ctx.clone());
    wijs_schermuitsnede_aan_met_muis(ctx.clone(), zelf); // tenzij al een scherm-gedeelte gekozen is
    plaats_cursor_en_toelichting(zelf, ctx.clone()); // cursorlijnen en menu's worden geplaatst, maar deze functie doet dat niet als beeld zal worden opgeslagen
}

fn toon_handleiding_of_niet(zelf: &mut UitsnedeApp) {
    if HANDLEIDING_GEBRUIK.get() == HandleidingGebruik::Never {
        zelf.handleiding_tonen = false;
    }
    if HANDLEIDING_GEBRUIK.get() == HandleidingGebruik::Always {
        zelf.handleiding_tonen = true;
    }
    if zelf.handleiding_tonen == true {  // handleiding wordt getoond, totdat het eerste hoekpunt wordt aangegeven - daarna wordt het scherm geheel zichtbaar.
        popup_multi_str(&HANDLEIDING_UITSNEDE); // de vector wordt aangeboden aan een berichtenvenster (zie de file bericht_venster.rs)
    } else {
        if BERICHT.with(|v| v.clone()).borrow().len() > 1 {
            popup_bericht(""); // het popup-bericht wordt gewist
        }
    }
}

fn zet_schermkopie_in_viewport (zelf: &mut UitsnedeApp, ctx: Context) {
    // in 'uitsnede_of_pijlen.rs' is een imagebuffer gemaakt van het schermbeeld
    // naast de imagebuffer hebben we ook een texturehandle nodig,
    // want texturehandle is het format van het image dat in een viewport kan worden getoond
    // de conversie gebeurt maar een(1) keer door 'image_buffer_to_texture'
    // de imagebuffer wordt bewaard in BEELD_OPTION, en is nodig als we een deel-image willen gaan opslaan
    // de texturehandle wordt bewaard in BEELD_OPTION_TEXTURE, en is nodig om het beeld op het scherm te tonen voor het maken van een selectie
    if zelf.beeld_conversie_gemaakt == false { // de conversie heeft nog niet plaatsgevonden
        let beeld_opt_texth = imagebuffer_to_texture(ctx.clone()); // conversie
        BEELD_OPTION_TEXTURE.set(beeld_opt_texth);
        /*
        if beeld_opt_texth.is_some() {
            zelf.beeld_conversie_gemaakt = true;            // bij een volgende update van de viewport hoeft de conversie niet opnieuw gemaakt te worden
            BEELD_OPTION_TEXTURE.set(beeld_opt_texth);      // daarom slaan we het resultaat van de conversie (TextureHandle) op
        }
        else {
            println!("BOT-none");
        }
        */
    }
    // nu is in alle gevallen de texture gereed 'Option= Some()', en kan met unwrap de texturehandle worden verkregen
    let beeld_option_texture = BEELD_OPTION_TEXTURE.with(|v| v.clone()).into_inner();
    let beeld_texturehandle = beeld_option_texture.expect("Dit kan niet gebeuren");
    
    // de image wordt op de achtergrond van het scherm gezet; die 'achtergrond' maken we nu beschrijfbaar
    let achtergrond = ctx.layer_painter(LayerId::new(Order::Background, Id::new("monitorbeeld")));

    let doelscherm = DOELSCHERM.with(|v| v.clone()).into_inner();
    achtergrond.image(   // zet de texturehandle op het scherm
        beeld_texturehandle.id(),
        Rect {
            min: Pos2 { x: 0.0, y: 0.0 },
            max: Pos2 {
                x: doelscherm.clone().xsize(),
                y: doelscherm.ysize(),
            },
        },
        Rect::from_min_max(Pos2 { x: 0.0, y: 0.0 }, Pos2 { x: 1.0, y: 1.0 }),
        Color32::WHITE,
    );
}

///Selectie van schermdeel met behulp van cursor en muis
fn wijs_schermuitsnede_aan_met_muis(ctx: Context, zelf: &mut UitsnedeApp) {
    if zelf.rechts_onder.is_none() {
        ctx.set_cursor_icon(CursorIcon::Crosshair);
    }
    let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("dubbellaag")));
    // met de painter kunnen lijnen, rechthoeken, teksten etc worden in het scherm/venster worden geplaatst 

    let mut knoppen: Vec<Knop> = vec![]; // rijtje menu-knoppen, standaard langs bovenramd
    knoppen = plaats_menu_knoppen(painter.clone(), knoppen.clone());

    ctx.input(|k| {
        if k.key_pressed(Key::F1) {   // Opslaan
            if zelf.links_boven.is_some() && zelf.rechts_onder.is_some() {
                OPSLAAN_VRAAG.set(false);
                OPSLAAN.set(true);
            }
            return;
        }
        if k.key_pressed(Key::F2) {    // Opties Aan/Uit
            OPTIES_ON.set(!OPTIES_ON.get());
            return;
        }
        if k.key_pressed(Key::F3) {     // Menu wisselend Boven, Rechts, Onder, Links, Geen
            MENU_POS.set((MENU_POS.get() + 1) % 5);
        }
        if k.key_pressed(Key::F4) {     // Afsluiten
            std::process::exit(0);
        }
        //        }
        if k.key_pressed(Key::Escape) {  // Escape: Menu Aan/Uit - om zicht op hele scherm te krijgen
            if MENU_POS.get() == 4 {
                MENU_POS.set(0)    // knoppen tonen boven links, standaard positie
            } else {
                MENU_POS.set(4)    // knoppen niet tonen
            };
        }
        ANDERE_SCHERM.set(zelf.muisplek.is_none());     // Waarschuwing op in actieve scherm: 'gebruik andere scherm' 
        // muis_li_indrukken => eerste hoekpunt
        zelf.muisplek= k.pointer.latest_pos();
        match zelf.muisplek {
            Some(muisplaats) => {
                // muis_li_loslaten => tweede hoekpunt
                if k.pointer.primary_pressed() {
                    zelf.handleiding_tonen = false;
                    if input_menu_knoppen(knoppen.clone(), zelf) == true {   // lees ingedrukte memu-knop, en return
                        return;
                    } else {
                        zelf.links_boven = Some(muisplaats);    // muis indrukken: muisplek= links-boven-hoek van uitsnede
                        zelf.rechts_onder = None;
                    }
                }
                if k.pointer.primary_released() {               // muis loslaten: muisplek= rechts-onder-hoek van uitsnede
                    if zelf.links_boven.is_some() {
                        zelf.rechts_onder = Some(muisplaats);
                        OPSLAAN_VRAAG.set(true);
                    }
                }
            }
            _ => {}
        }
    });

    if UITBREIDEN.get() {
    // als selectie is bijna volledig scherm: maak er dan volledig scherm van
    // dit voorkomt onbedoeld een HEEL_SCHERM-kopie die onvolledig blijkt
    // volledig scherm kan ook worden gekozen in het optie-venster
        match zelf.links_boven {        // alleen als hoekpunt linksboven bekend
        None => {},
        Some(li_bo) => 
            match zelf.rechts_onder {   // alleen als rechtsonder belend bekend
                None => {}
                Some(re_on) => {
                    let doelinfo = DOELSCHERM.with(|v| v.clone()).into_inner();
                    if re_on.x - li_bo.x > doelinfo.clone().xsize() - 50.0   // vrijwel volledige schermbreedte
                    && re_on.y - li_bo.y > doelinfo.clone().ysize() - 50.0   // vrijwel volledige schermhoogte 
                    {
                        //OPSLAAN_VRAAG.set(true);
                        popup_bericht("HEEL SCHERM\nwordt opgeslagen\nDat duurt even!");
                        OPSLAAN.set(true);
                        HEEL_SCHERM.set(true);
                        //zelf.links_boven = Some(Pos2 { x: 0.0, y: 0.0 });
                        //zelf.rechts_onder = Some(Pos2 {
                        //    x: doelinfo.clone().xsize() - 0.0,
                        //    y: doelinfo.ysize() - 0.0,
                        //});
                        return;
                    }
                }
            },
        }    
    }

    if zelf.links_boven.is_some()
        && zelf.rechts_onder.is_some()
        && (zelf.links_boven.unwrap().x + 10.0 > zelf.rechts_onder.unwrap().x
            || zelf.links_boven.unwrap().y + 10.0 > zelf.rechts_onder.unwrap().y)
    {
        // heel kleine uitsnede, bedieningsfout!?, daarom wissen:
        zelf.links_boven = None;
        zelf.rechts_onder = None;
    }
    muis_en_toets_diversen(ctx.clone()); // toets-acties (nu alleen 'Muis-Re'= afsluiten) die gebruikt worden in uitsnede-venster en ook in optie-venster
}

///De cursor-lijnen worden getekend, met enkele woorden als toelichting
///Dit wordt niet gedaan als de slectie gemaakt is en opgeslagen moet worden om een 'schone' afbeelding te krijgen
fn plaats_cursor_en_toelichting(zelf: &mut UitsnedeApp, ctx: Context) {
    // laat dit weg vlak voor maken van het screenshot
    if OPSLAAN.get() { // tijdens opslaan zijn deze aingen overbodig
        return;
    }
    let font = FontId::monospace(14.0);
    let mut uitleg = UITLEG.get_string(); // passend bijschrift voor bepalen links-boven
    let painter = ctx.layer_painter(LayerId::new(Order::Background, Id::new("dubbellaag")));
    let stroke = Stroke {   // te gebruiken lijndikte en kleur
        width: 1.0,
        color: if DASHED.get() {Color32::BLACK} else {KLEUR.get()}
    };

    match zelf.muisplek {     // muis is op een ander scherm of wordt anderszins niet gevonden
        None => match zelf.links_boven {
            None => {}
            Some(b) => match zelf.rechts_onder {
                None => {}
                Some(o) => {
                    // hoekpunten uitsnede bekend: teken rechthoek (geen vulling, alleen lijnen) 
                    painter.rect_stroke(
                        Rect { min: b, max: o },   // rechthoek - hoekpunten
                        0.0,        // hoeken niet afronden
                        stroke,                    // lijndikte en kleur                    
                        StrokeKind::Inside         // rand binnen of buiten de hoekpunten
                    );
                    if DASHED.get() {rect_dashed(painter.clone(), Rect{min: b, max: o});    }
                }
            },
        },
        Some(muisplaats) => {
            if zelf.links_boven.is_none() {
                // cursor voor LINKS_BOVEN wordt getekend met twee lijnen: van de muis naar rechts en van de muis naar onder
                painter.hline(muisplaats.x..=ctx.content_rect().max.x, muisplaats.y, stroke);
                painter.vline(muisplaats.x, muisplaats.y..=ctx.content_rect().max.y, stroke);
                if DASHED.get() {
                    line_dashed(painter.clone(), Pos2{x:muisplaats.x, y: muisplaats.y} , Pos2{x: ctx.content_rect().max.x, y: muisplaats.y});
                    line_dashed(painter.clone(), Pos2{x:muisplaats.x, y: muisplaats.y} , Pos2{x: muisplaats.x, y: ctx.content_rect().max.y});
                }
            } else {
                uitleg = UITLEG_TWEE.get_string(); // passend bijschrift voor bepalen rechts-onder
                if zelf.rechts_onder.is_none()                              // rechtsonder moet gemaakt worden
                    && muisplaats.x < zelf.links_boven.unwrap().x + 50.0 
                    && muisplaats.y < zelf.links_boven.unwrap().y + 50.0    // muis dichtbij links-boven het eerdere hoekpunt of er dihtbij
                {
                    // cursor voor RECHTS_ONDER wordt getekend met twee lijnen: van muis naar links en van muis naar boven
                    painter.hline(ctx.content_rect().min.x..=muisplaats.x, muisplaats.y, stroke);
                    painter.vline(muisplaats.x, ctx.content_rect().min.y..=muisplaats.y, stroke);
                    if DASHED.get() {
                        line_dashed(painter.clone(), Pos2{x:muisplaats.x, y: muisplaats.y} , Pos2{x: ctx.content_rect().min.x, y: muisplaats.y});
                        line_dashed(painter.clone(), Pos2{x:muisplaats.x, y: muisplaats.y} , Pos2{x: muisplaats.x, y: ctx.content_rect().min.y});
                    }
                } else {
                    let rect= Rect {
                        min: zelf.links_boven.unwrap(),
                        max: if zelf.rechts_onder.is_some() {
                            zelf.rechts_onder.unwrap()
                        } else {
                            muisplaats
                        },
                    }; 
                    // muis rechts-onder het eerste hoekpunt: een (voorlopige) rechthoek wordt getekend
                    painter.rect_stroke(
                        rect,
                        0.0,
                        stroke,
                        StrokeKind::Inside,
                    );
                    if DASHED.get() {rect_dashed(painter.clone(), rect);}
                }
            }
            if zelf.rechts_onder.is_some() {
                uitleg = "".to_string()   // geen uitleg meer nodig
            };
            painter.text(
                // plaats het bijschrift 'uitleg'
                muisplaats,
                if zelf.links_boven.is_none() {
                    Align2::LEFT_BOTTOM
                } else {
                    Align2::LEFT_TOP
                },
                uitleg,
                font,
                KLEUR.get(),
            );
        }
    };
    toon_handleiding_of_niet(zelf);  // wordt 1x getoond, en verdwijnt na de eerste druk op de linker muisknop
}

fn plaats_menu_knoppen(painter: Painter, mut knoppen: Vec<Knop>) -> Vec<Knop> {
    // menu-knoppen worden aan de rand van het scherm getoond, met zelf geprogrammeerde knoppen
    // alternatief, te kiezen in 'opties': menu wordt getoond in een eigen venster dat verplaatst kan worden
    // in linux wordt zo'n venster standaard centraal in het scherm geplaatst; het kan wel worden verplaatst maar zit te vaak in de weg
    if MENU_TYPE.get() == MenuType::Popup || MENU_POS.get()>=4 {return Vec::new()};
    knoppen.push(Knop::new(     // Knop is gedefinieerd in 'gereedschap.rs'
        painter.clone(),
        "Opslaan; naar bewerken (F1)".to_string(),
        knoppen.last(),  // geen laatste knop, dus start-positie
    ));
    knoppen.push(Knop::new(     // Knop is gedefinieerd in 'gereedschap.rs'
        painter.clone(),
        "Opslaan; nog een uitsnede".to_string(),
        knoppen.last(),  // geen laatste knop, dus start-positie
    ));
    knoppen.push(Knop::new(
        painter.clone(),
        "Opties AAN/UIT (F2)".to_string(),
        knoppen.last(),     // grootte laatst opgegeven knop bepaalt positie van de nieuwe knop
    ));
    knoppen.push(Knop::new(
        painter.clone(),
        "Menu verplaatsen (F3, Esc)".to_string(),
        knoppen.last(),
    ));
    knoppen.push(Knop::new(
        painter.clone(),
        "Afsluiten (F4)".to_string(),
        knoppen.last(),
    ));
    knoppen
}

fn input_menu_knoppen(knoppen: Vec<Knop>, zelf: &mut UitsnedeApp) -> bool {
    if knoppen.len() >= 4 {
        // identiek aan menutype Rand actief
        let muis = zelf.muisplek;
        if knoppen[0].bevat(muis) {
            //ctx.request_repaint_of(OPSLAAN_VENSTER.get());  // is dit nodig??
            if zelf.links_boven.is_some() && zelf.rechts_onder.is_some() {
                OPSLAAN_VRAAG.set(false);
                OPSLAAN.set(true);
            }
            return true;
        } else {
            if knoppen[1].bevat(muis) {
            if zelf.links_boven.is_some() && zelf.rechts_onder.is_some() {
                OPSLAAN_VRAAG.set(false);
                OPSLAAN.set(true);
                zelf.beeld_conversie_gemaakt= false;
                println!("NU OPSLAAN");
//                BEELD_OPTION_TEXTURE.set(None);
                RETOUR_NAAR_UITSNEDE.set(true);
                return true;
            }
            } else {
                if knoppen[2].bevat(muis) {
                    OPTIES_ON.set(!OPTIES_ON.get());
                    return true;
                } else {
                    if knoppen[3].bevat(muis) {
                        MENU_POS.set((MENU_POS.get() + 1) % 5);
                        return true;
                    } else {
                        if knoppen[4].bevat(muis) {
                            std::process::exit(0);
                        } else {
                            zelf.links_boven = muis;
                            zelf.rechts_onder = None;
                        }
                    }
                }
            }
        }
    }    
    false
}

///gekozen schermdeel wordt opgeslagen op schijf; het uitsnedevenster wordt onzichtbaar en het pijlenvenster wordt geopend. om met pijlen en bijschriften uitleg te geven over de afbeelding.
pub fn opslaan_uitsnede(zelf: &mut UitsnedeApp, ctx: Context) {
    //: &mut SchermBeeld) {
    // parent, title, events, native_pixels_per_point(Option), monitor_size(Option), inner_rect(Option), outer_rect(Option),
    // minimized(Option), maximized(Option), fullscreen(Option), focused)Option)
    if OPSLAAN.get() {
        let naam = BESTANDSNAAM.get_string()+"-";
        if HEEL_SCHERM.get() {
            zelf.links_boven= Some(Pos2{x: 0.0, y: 0.0});
            zelf.rechts_onder= Some(Pos2{x: doelscherminfo().xsize() - 0.0, y: doelscherminfo().ysize() - 0.0});
        }
        match zelf.links_boven {
            None => {}
            Some(mut lb) => match zelf.rechts_onder {
                None => {}
                Some(mut ro) => {
                    // we willen het venster binnen de cursorlijnen, dus de cursorlijnen niet meegerekend -1, +1
                    let doelscherm = DOELSCHERM.with(|v| v.clone()).into_inner();
                    lb = Pos2 {   // Pos2{links-boven} krijgt de juiste waarden voor x en y
                        x: lb.x
                            + doelscherm.clone().xpos()
                            + if HEEL_SCHERM.get() { 0.0 } else { 1.0 },
                        y: lb.y
                            + doelscherm.clone().ypos()
                            + if HEEL_SCHERM.get() { 0.0 } else { 1.0 },
                    };
                    ro = Pos2 {   // Pos2{rechts-onder} krijgt de juiste waarden voor x en y
                        x: ro.x  - 1.0
                            + doelscherm.clone().xpos(),
                        y: ro.y  - 1.0
                            + doelscherm.ypos(),
                    };
                    //NB: de screenshot functie van eframe::egui tekent het huidige venster op, maar niet de achtergrond, dat is de afbeelding die we kopieren willen.
                    //daarom wordt de Crate xcap.rs gebruikt om het screenshot te maken. Misschien is er ook een manier om het uitsnede venster echt onzichtbaar te maken.
                    if bewaar_scherm_regio(
                        Rect { min: lb, max: ro },
                        naam.clone(),
                        RANDONBEWERKT.get() as u32,
                        true,
                    ) == false                       // als het niet lukt om op te slaan
                    {
                        naar_uitsnede(ctx.clone());  // bedoeld als reset van uitsnede
                        return;
                    };
                }
            },
        }
        OPSLAAN.set(false);
        if RETOUR_NAAR_UITSNEDE.get()== false {
            naar_pijlen(ctx); // naar_pijlen vangt zelf een evtl 'retour_naar_uitsnede' op!
        } else {
            BEELD_OPTION_TEXTURE.set(None);
            naar_uitsnede(ctx);
        }
    }
}

/// switch van uitsnede_venster naar pijlenvenster om toelichting bij de afbeelding te maken met pijlen en bijschriften
pub fn naar_pijlen(ctx: Context) {    // uitsnede is klaar
    if RETOUR_NAAR_UITSNEDE.get() {   // maar er zijn situaties ...
        naar_uitsnede(ctx.clone());
        return;
    } else {
        PIJLENKOKER.take();         // het pijlengeheugen wordt leeg gemaakt
        OPSLAAN_VRAAG.set(false);   // nieuw
        MAAK_UITSNEDE.set(false);   // de switch wordt ingezet
        ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close);  // opdracht om uitsnede_app te sluiten
        // de update-functie van SchermBeeld  wordt niet meer uitgevoerd; de uitsnede_app wordt afgelsoten
        // MAAK_UITSNEDE zorgt dat in uitsnede_of_pijlen voor 'pijlen' wordt gekozen
    }
}

/// switch van pijlen-venster terug naar uitsnede-venster, om een nieuwe selectie te maken of andere pijlen te zetten
pub fn naar_uitsnede(ctx: Context) {
    if BESTAAND_BEELD.get() {
        return;
    }
    BEELD_SELECTIE.set(None);   // in geval pijlen_app nog eens wordt gebruikt moet de oude selectie vervallen zijn
    BEELD_OPTION_TEXTURE.set(None);
    HEEL_SCHERM.set(false);
    if MAAK_UITSNEDE.get()==false             // als afkomstig van pijlen_app moet die worden afgesloten
    // || if RETOUR_NAAR_UITSNEDE.get() == false
    {    
        ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close);
    }
    MAAK_UITSNEDE.set(true);
    RETOUR_NAAR_UITSNEDE.set(false);
    // pijlen_app (of ingeval van RETOUR_NAAR_UITSNEDE uitsnede_app) wordt afgesloten, en uitsnede wordt opnieuw opgestart 
}



