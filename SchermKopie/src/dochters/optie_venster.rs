use std::{env, ffi::OsString, fs::{exists, read_to_string}, io::Write, path::MAIN_SEPARATOR};

use dirs_next::home_dir;
use eframe::egui::{
    Align, CentralPanel, Color32, Context, Layout, Pos2, Rect, Slider, Stroke, StrokeKind, ViewportBuilder, Visuals
};

use crate::dochters::{
        gereedschap::{EigenTrait4LocalKey, EigenTraitTbvUi, MonitorFunctions,doelscherminfo, muis_en_toets_diversen, muis_naar_monitor_midden, os_linux, hoofdscherm}, 
        globaal::{BESTANDSNAAM, DASHED, HANDLEIDING_GEBRUIK, HEEL_SCHERM, HandleidingGebruik, KLEUR, LAATSTE_FILE_PATH, LAATSTE_PIJL_PATH, MAAK_UITSNEDE, MENU_TYPE, 
            MONITOR_WISSEL, MenuType, OPSLAAN, OPSLAAN_VRAAG, OPTIE_VIEWPORT, OPTIES_ON, POSITIE_HORZ, POSITIE_VERT, RANDKLEUR, RANDNABEWERKEN, RANDONBEWERKT, UITBREIDEN, VERKLEIN,
    }};

use crate::dochters::text_input::{Teksttype, text_opdracht};

pub fn optie_functie(ctx: Context) {
    if OPTIES_ON.get()==false {  // met de thread_local! OPTIES_ON wordt het optie_venster zichtbaar of niet-zichtbaar gemaakt.
        return;
    }
    if OPSLAAN.get() {return;}
    let windowsize= [400.0, 600.0];
    let position= Pos2{ x: hoofdscherm().xpos()/hoofdscherm().scale(),
                              y: (hoofdscherm().ypos() + hoofdscherm().ysize()- windowsize[1])/hoofdscherm().scale() - 30.0};

    ctx.set_visuals(Visuals::dark());
    ctx.show_viewport_deferred(
        //deze functie heeft 3 argumenten nodig: (Viewport_id, ViewportBuilder, vieuwport_ui_cb)
        OPTIE_VIEWPORT.with(|v|{v.clone()}).into_inner(),
        ViewportBuilder::default()
            .with_title("Optie venster")
            .with_position(position)
            .with_always_on_top()
            .with_inner_size(windowsize),
        move |ctx, _class| {
            //let style = style::Style::default();
            //let mut framestyle = containers::Frame::central_panel(&style);
            //framestyle.fill = Color32::TRANSPARENT;
            //CentralPanel::default().frame(framestyle).show(ctx, |ui| {
            CentralPanel::default().show(ctx, |ui| {
                let mut save_opties= false;
                let mut kleur_was = KLEUR.get();
                let dashed= DASHED.get();
                ui.markering_kleur(&mut kleur_was);      // markering_kleur in gereedschap.rs bevaat o.a. een 'color_picker_color32'
                if kleur_was!=KLEUR.get() || dashed!=DASHED.get() {
                    KLEUR.set(kleur_was);
                    save_opties= true;
                }    
                if os_linux()== false {
                    if ui.button("Andere Scherm").clicked() {      // je wilt een kopie maken van het andere scherm, niet van deze (schermwissel functie)
                        if MAAK_UITSNEDE.get() == true {
                            MONITOR_WISSEL.set(true);
                            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close);
                        }
                    };
                }    
                if ui.button("Verklein").clicked() {            // beeld wordt verkleind met factor 2/3, moet worden verbeterd of verwijderd
                    VERKLEIN.set(true);
                };
                if ui.button("Uitsnede uitbreiden als vrijwel volledig  ".to_owned()    // bijna volledig scherm geselecteerd: uitbreiden tot volledig??
                                                                                               // alternatief: knop 'Kopieer heel scherm: NU'     
                    + if UITBREIDEN.get() {"AAN"} else {"UIT"}
                ).clicked() {
                    UITBREIDEN.set(UITBREIDEN.get()==false);
                };

                let mut gebruik_handleiding = HANDLEIDING_GEBRUIK.get();
                // radioknoppen lunnen niet rechtstreels een thread_local! zoals HANDLEIDING_GEBRUIK wijzigen, duas wijzigen we gebruik_handleiding
                ui.horizontal(|ui| {                 // radio-knoppen
                    ui.label("Handleiding");
                    if ui
                        .radio_value(&mut gebruik_handleiding, HandleidingGebruik::Always, "Altijd")
                        .clicked()         // toon handleiding altijd
                    {};
                    if ui
                        .radio_value(&mut gebruik_handleiding, HandleidingGebruik::Protocol, "Standaard")
                        .clicked()         // toon handleiding tot de eerste muisklik, daarna is het scherm schoon
                    {};
                    if ui
                        .radio_value(&mut gebruik_handleiding, HandleidingGebruik::Never, "Nooit")
                        .clicked()         // toon handleiding niet
                    {};
                });
                // gewijzigde gebruik_handleiding terugzetten naar HANDLEIDING_GEBRUIK
                HANDLEIDING_GEBRUIK.set(gebruik_handleiding);

                let mut menu_type = MENU_TYPE.get();
                ui.horizontal(|ui| {    // radioknoppen naast epkaar ipv boven elkaar
                    ui.label("Menu:");
                    if ui
                        .radio_value(&mut menu_type, MenuType::Rand, "Rand")
                        .clicked()       // toon menu langs beeldrand
                    {};
                    if ui
                        .radio_value(&mut menu_type, MenuType::Popup, "Popup")
                        .clicked()        // toon bediening in popup-window
                    {};
                    if ui
                        .radio_value(&mut menu_type, MenuType::Beide, "Beide")
                        .clicked()         // toon beeldrand-menu EN bedienings-window
                    {};
                });
                if menu_type!= MENU_TYPE.get() {
                    MENU_TYPE.set(menu_type);
                    save_opties= true;
                }

                let mut huidige_randkleur = RANDKLEUR.get();
                ui.horizontal(|ui| {
                    ui.label("Randkleur");
                    if ui
                        .radio_value(&mut huidige_randkleur, Color32::WHITE, "Wit")
                        .clicked()
                    {};
                    if ui
                        .radio_value(&mut huidige_randkleur, Color32::GRAY, "Grijs")
                        .clicked()
                    {};
                    if ui
                        .radio_value(&mut huidige_randkleur, Color32::BLACK, "Zwart")
                        .clicked()
                    {};
                    if ui
                        .radio_value(&mut huidige_randkleur, KLEUR.get(), "Vrij")
                        .clicked()
                    {};
                });
                if huidige_randkleur!= RANDKLEUR.get() {
                    RANDKLEUR.set(huidige_randkleur);
                    save_opties= true;
                }
                   
                ui.horizontal(|ui| {   // aan de onbewerkte schermkopie wordt een randje toegevoegd; soms nuttig bij deels witte afbeeldingen 
                    ui.label("Rand niet bewerkt: ");
                    let mut dikte = RANDONBEWERKT.get();
                    if ui.button("<").clicked() {
                        if dikte > 0 {
                            dikte -= 1;
                        }
                    };
                    ui.label(format!(" {} ", dikte));
                    if ui.button(">").clicked() {
                        dikte += 1;
                    };
                    if dikte!= RANDONBEWERKT.get() {
                        RANDONBEWERKT.set(dikte);
                        save_opties= true;
                    }    
                    if dikte <= 0 {
                        ui.label("  Geen rand");
                    }
                });
                ui.horizontal(|ui| {   // idem bij het bewerkte scherm, wanneer pijlen tot buiten de afbeelding zijn getrokken, omvat de rand ook die pijlen geheel
                    ui.label("Rand na bewerken: ");
                    let mut dikte = RANDNABEWERKEN.get();
                    if ui.button("<").clicked() {
                        if dikte > 0 {
                            dikte -= 1;
                        }
                    };
                    ui.label(format!(" {} ", dikte));
                    if ui.button(">").clicked() {
                        dikte += 1;
                    };
                    if dikte!= RANDNABEWERKEN.get() {
                        RANDNABEWERKEN.set(dikte);
                        save_opties= true;
                    }    
                    if dikte <= 0 {
                        ui.label("  Geen rand");
                    }
                });
                //let buttontekst= "Kopieer het hele Scherm: NU".to_string();
                if ui.button("Kopieer het hele Scherm: NU".to_string()).clicked() {
                    muis_naar_monitor_midden(doelscherminfo());  // verplaatst muis naar doelscherm, en activeert daaarmee het effect
                    OPSLAAN.set(true);
                    OPSLAAN_VRAAG.set(false);
                    HEEL_SCHERM.set(true);        // deze variabele zorgt dat het hele scherm wordt verwerkt, en niet (met de muis) een schermdeel
                };
                if MAAK_UITSNEDE.get() == false {   // alleen bij het pijlen setten van toepassing
                    let mut horizontaal = POSITIE_HORZ.get();
                    let mut vertikaal = POSITIE_VERT.get();
                    ui.add(Slider::new(&mut horizontaal, -1.0..=1.0).text("Positie Horiz"));   // sliders die in pijlen_app ruimte rond beeld verdelen
                    ui.add(Slider::new(&mut vertikaal, -1.0..=1.0).text("Positie Vert"));      // voor grote bijschriften buiten het beeld bijv
                    //The default Slider size is set by crate::style::Spacing::slider_width.
                    POSITIE_HORZ.set(horizontaal);
                    POSITIE_VERT.set(vertikaal);
                }
                ui.label("");
                let toonlf = LAATSTE_FILE_PATH.get_string();
                ui.label("SchermSchot: ".to_string() + toonlf.to_string().as_str());   // toont de filenaam van de laatste schermkopie 
                let toonlp = LAATSTE_PIJL_PATH.get_string();
                ui.label("Pijlenbeeld: ".to_string() + toonlp.to_string().as_str());   // toont de filenaam van een bewerkte schermkopie (met pijlen en bijschriften)
                ui.label("");
                muis_en_toets_diversen(ctx.clone());   // behandeling van Window-sluiten
                let naamstr = BESTANDSNAAM.get_string();
                ui.horizontal(|ui| {
                    ui.label("Bestandsnaam= ".to_string()+&naamstr)
                });
                if ui.button("Verander Bestandsnaam").clicked() {
                    text_opdracht(Teksttype::Bestandsnaam, naamstr.clone());  // aan deze bestandsnaam worden later nog unieke volgnrs toegevoegd
                    save_opties= true;
                }
                ui.horizontal(|ui|
                    (
                        if ui.button("Standaard opties").clicked() {        // terug naar standaardopties
                            standaard_opties();
                            save_opties= true;
                        },
                        ui.with_layout(Layout::top_down(Align::Max), |ui| {if ui.button("Programma sluiten").clicked() {
                            std::process::exit(0);
                        }}),
                    )
                );
                if save_opties { schrijf_ini_file();}
                if ctx.input(|i| i.viewport().close_requested()) { // window-sluiten geklikt
                    OPTIES_ON.set(false);
                }
                let painter = ui.painter();
                painter.rect_stroke(
                    Rect {
                        min: Pos2 { x: 0.0, y: 0.0 },
                        max: Pos2 {
                            x: ctx.content_rect().max.x,
                            y: ctx.content_rect().max.y,
                        },
                    },
                    0.0,
                    Stroke {width: 2.0, color: huidige_randkleur},
                    StrokeKind::Inside   
                );
            });
        },
    );
    wayland_rand();
}

pub fn schrijf_ini_file() {
    let homedir= path_ini_file();
    let schrijf_file= std::fs::File::create(homedir.clone());
    match schrijf_file  {
        Ok(mut sf) => {
            let _ = writeln!(sf, "Accentkleur {} {} {} {}", KLEUR.get().r(),KLEUR.get().g(),KLEUR.get().b(),KLEUR.get().a());
            let _ = writeln!(sf, "Randkleur {} {} {} {}", RANDKLEUR.get().r(),RANDKLEUR.get().g(),RANDKLEUR.get().b(),RANDKLEUR.get().a());
            let _ = writeln!(sf, "Rand-onbewerkt {}", RANDONBEWERKT.get());
            let _ = writeln!(sf, "Rand-na-bewerken {}", RANDNABEWERKEN.get());
            let _ = writeln!(sf, "Bestandsnaam {}", BESTANDSNAAM.get_string());
            let _ = writeln!(sf, "Menutype {:?}", MENU_TYPE.get());
            let _ = writeln!(sf, "Dashed-Y/N {:?}", DASHED.get());
            let _ = sf.flush();
        },
        Err(_err) =>{},
    }
}

pub fn lees_ini_file() {
    let homedir= path_ini_file();
    let exist_ini=  exists(homedir.as_os_str()); 
    if exist_ini.is_ok() && exist_ini.unwrap()==false { println!("geen ini {:?}", homedir); return();}  // hier is een standaard voor!
    else {println!("ini gelezen {:?}", homedir);}
    //if exist_ini.is_ok()?.unwrap()==false { println!("geen ini {:?}", homedir); return();}  // hier is een standaard voor!
    //else {println!("ini gelezen {:?}", homedir);}
    let mut result = Vec::new();
    for line in read_to_string(homedir).unwrap().lines() {
        let woorden= line.split_whitespace();
        let mut optie= "";
        let mut kleur: Vec<i32> = vec![];
        for woord in woorden {
            if optie=="" {optie= woord; continue;}
            if optie== "Bestandsnaam" {BESTANDSNAAM.set(woord.to_string());}
            let byte = woord.parse::<i32>();
            if byte.is_ok() {kleur.push(byte.unwrap());}
            if optie== "Rand-onbewerkt"   && kleur.len()>0 {RANDONBEWERKT.set(kleur[0] as usize);}
            if optie== "Rand-na-bewerken" && kleur.len()>0 {RANDNABEWERKEN.set(kleur[0] as usize);}
            if optie== "Menutype" {
                MENU_TYPE.set(
                if woord=="Rand" {MenuType::Rand} 
                    else if woord=="Popup" {MenuType::Popup}
                        else {MenuType::Beide});
            }
            if optie== "Accentkleur"      && kleur.len()==4 {
                KLEUR.set(Color32::from_rgba_unmultiplied(kleur[0] as u8, kleur[1] as u8, kleur[2] as u8, kleur[3] as u8));
            }
            if optie== "Randkleur" && kleur.len()==4 {
                RANDKLEUR.set(Color32::from_rgba_unmultiplied(kleur[0] as u8, kleur[1] as u8, kleur[2] as u8, kleur[3] as u8));
            }
            if optie== "Dashed-Y/N" {
                DASHED.set(woord.parse::<bool>().unwrap_or(true));
            }
        }
        result.push(line.to_string())
    }
     //if os_linux() {MENU_TYPE.set(MenuType::Rand);}
     wayland_rand();   // wayland zet vensters zoals het bedieningsvenster in het centrum van het beeld, en dat stoort bijna altijd --> alleen RAND-menu gebruiken
}

fn path_ini_file() -> OsString {                      // rust werkt voor padnamen etc met niet-normale strings (alleen ASCI, geen unicode characters)
    let args: Vec<String> = env::args().collect();
    let mut prognaam= args[0].to_string();
    prognaam.truncate(prognaam.rfind("//").unwrap_or(usize::MAX));            // programma-naam zonder .exe
    prognaam= prognaam.split_off(prognaam.rfind(MAIN_SEPARATOR).unwrap_or(0));   // programma-naam zonder directorynaam
    let mut homedir= home_dir().unwrap_or("".to_string().into()).into_os_string();  // home-directory van gebruiker
    homedir.push(prognaam.clone());
    homedir.push(".ini");
    homedir
}

fn standaard_opties() {
    KLEUR.set(Color32::RED);
    RANDKLEUR.set(Color32::GRAY);
    RANDONBEWERKT.set(1);
    RANDNABEWERKEN.set(0);
    DASHED.set(true);
    MENU_TYPE.set(MenuType::Rand);
}

pub fn wayland_rand() {
    let session= env::var("XDG_SESSION_TYPE");
    if session.is_ok() && session.unwrap()=="wayland" {MENU_TYPE.set(MenuType::Rand);}
}