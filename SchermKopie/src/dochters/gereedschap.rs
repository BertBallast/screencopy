use crate::dochters::globaal::{
    BEELD_OPTION, BESTAAND_BEELD, DASHED, DESKTOP_MAP, LAATSTE_FILE_PATH, LAATSTE_PIJL_PATH, LAATSTEBEELD, OPTIES_ON, PIJL_TEKST_GROOT, PIJLENKOKER, RANDKLEUR
};
use crate::dochters::globaal::{
    BERICHT, BESTANDSNAAM, MAAK_UITSNEDE, MENU_POS, MONITOR_WISSEL,
};
use crate::dochters::pijlen_app::PijlenApp;
use crate::{DOELSCHERM, KLEUR};
//use core::time;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::path::{Path, MAIN_SEPARATOR};
use std::thread::LocalKey;
use std::time::SystemTime;

use dirs_next::desktop_dir;
use eframe::egui::color_picker::color_picker_color32;
use eframe::egui::color_picker::Alpha;
use eframe::egui::FontFamily;
use eframe::egui::Ui;
use eframe::egui::Vec2;
use eframe::egui::{self};
use eframe::egui::{FontId, TextureHandle};
use eframe::egui::{Id, LayerId, Order, Shape};
use eframe::epaint::ColorImage;
use egui::{Align2, Color32, Context, Painter, Pos2, Rect, Stroke};
use ferrishot_xcap::Monitor;
use image::flat::Error;
use image::imageops::overlay;
use image::{GenericImageView, ImageError, ImageReader, Rgba, RgbaImage};
use mouse_rs::Mouse;
use std::cmp::{max, Reverse};
use std::fs::remove_file;
use std::fs::{self, DirEntry};
use std::{env, io};

use super::globaal::{BEELD_OPTION_TEXTURE, ENKELSCHERM};
//use screenshots::display_info::DisplayInfo as Monitor;
//use screenshots::Screen;

const MAX_AANTAL_GELIJKE_FILENAMEN: i32 = 5;

/*
ColorImage is een struct van de Crate epaint, gedefinieerd in een file image.rs (in de Crate epaint)
in epaint::image.rs vind je voorbeelden om een colorimage te maken, vanuit pad naar file, vanuit memory, vanuit Rgba-unmultiplied en Rgba-premultiplied
ook een functie region om een deel-image te maken
Mijn functie load_image_from_path maakt van een file-pad een dynamicimage en daarvan een ColorImage
en die wordt in lees_zojuist_gemaakt_schermbeeld (in pijlenvenster) om gezet in een TextureHandle en met painter.image() op scherm geplaatst

De Crate image.rs kent andere images:
DynamicImage
GenericImage
ImageBuffer
dynimage.rs in Crate image bevat een goede resize functie
image kent een functie sub_image die overeenkomt met de region-functie van epaint

van schermbeeld naar ImageBuffer:
Monitor.capture_image > sub_image > Imagebuffer > to_image()  (Crate image; zie bewaar ...)
*/

// namen van traits veranderen; een heelboel onnodige clone()-s nodig, veelal 'as f32' nu overbodig
pub trait Xpos4Monitor {
    fn xpos(&self) -> f32;
}

impl Xpos4Monitor for Monitor {
    fn xpos(&self) -> f32 {
        self.x().unwrap() as f32
    }
}

pub trait Ypos4Monitor {
    fn ypos(&self) -> f32;
}

impl Ypos4Monitor for Monitor {
    fn ypos(&self) -> f32 {
        self.y().unwrap() as f32
    }
}

pub trait Xsize4Monitor {
    fn xsize(&self) -> f32;
}

impl Xsize4Monitor for Monitor {
    fn xsize(&self) -> f32 {
        self.width().unwrap() as f32
    }
}

pub trait Ysize4Monitor {
    fn ysize(&self) -> f32;
}

impl Ysize4Monitor for Monitor {
    fn ysize(&self) -> f32 {
        self.height().unwrap() as f32
    }
}

pub trait Scale4Monitor {
    fn scale(&self) -> f32;
}

impl Scale4Monitor for Monitor {
    fn scale(&self) -> f32 {
        self.scale_factor().unwrap()
    }
}

pub trait Ident4Monitor {
    fn ident(&self) -> u32;
}

impl Ident4Monitor for Monitor {
    fn ident(&self) -> u32 {
        self.id().unwrap()
    }
}

pub trait Primary4Monitor {
    fn primary(&self) -> bool;
}

impl Primary4Monitor for Monitor {
    fn primary(&self) -> bool {
        self.is_primary().unwrap()
    }
}

/// Pijl= punt-locatie, achter-locatie, bijschrift, kleur
#[derive(Clone)]
pub struct Pijltje {
    pub rug: Pos2,
    pub tip: Pos2,
    pub pijltekst: String,
    pub kleur: Color32,
    pub text_rect: Rect,
}

/// Specificatie van een pijlwijziging: volgnumnmer van de pijl, wat te wijzigen: punt-locatie, achter-locatie, overig(kleur of bijschrift)
#[derive(Clone, Copy)]
pub struct PijlWijziging {
    // vraagt om een optie<enum> !!!
    pub pijlnummer: Option<usize>,
    pub staart: bool,
    pub punt: bool,
    pub schacht: bool,
    pub start_muis: Option<Pos2>,
}

impl Default for PijlWijziging {
    fn default() -> Self {
        Self {
            pijlnummer: None,
            staart: false,
            punt: false,
            schacht: false,
            start_muis: None,
        }
    }
}

/// Nieuwe positie na verwerken van een correctie/referentie
pub fn plaats_in_rect(was: Pos2, corr: Pos2) -> Pos2 {
    // was = positie in scherm
    // resultaat = positie t.o.v. een coordinaat (linkerbovenhoek van een rechthoek)
    Pos2 {
        x: was.x - corr.x,
        y: was.y - corr.y,
    }
}

/// Nieuwe positie na verwerken van een correctie/referentie; andersom, dus een terugplaatsing
pub fn plaats_in_scherm(was: Pos2, corr: Pos2) -> Pos2 {
    // was= positie t.o.v. een coordinaat corr (linkerbovenhoek van een rechthoek)
    // resultaat= t.o.v. het hele scherm (dwz coordinaten bij elkaar opgeteld)
    Pos2 {
        x: was.x + corr.x,
        y: was.y + corr.y,
    }
}

/// bepaal een nieuwe positie, als de gegeven positie buiten de rechthoek valt.
pub fn forceer_in_rect(mut pos: Pos2, rect: Rect) -> Pos2 {
    pos.x = pos.x.max(rect.min.x).min(rect.max.x);
    pos.y = pos.y.max(rect.min.y).min(rect.max.y);
    pos
}

/// Input-functies die zowel in het optie_venster als in het uitsnede_venster zijn geactiveerd;
/// Muisknop rechts => programma beeindigen, F4 => heropenen gesloten optie_venster
pub fn muis_en_toets_diversen(ctx: Context) {
    ctx.input(|k| {
        if k.pointer.secondary_pressed() {
            std::process::exit(0);
        }
    });
}

#[derive(Clone)]
pub struct Knop {
    pub rect: Rect,
    volgnr: usize,
}

impl Knop {
    pub fn new(painter: Painter, label: String, last_knop: Option<&Knop>) -> Knop {
        let mut volgnr = 0;
        const MARGE: f32 = 5.0;
        const KNOPFONT: FontId = FontId::monospace(18.0);
        const YSIZE: f32 = 22.0; //KNOPFONT.size + 4.0;
        const SPATIE: f32 = 10.0;
        const STROKE_WIDTH: f32 = 2.0;
        let monitor_width = doelscherminfo().width().unwrap_or(200);
        let monitor_height = doelscherminfo().height().unwrap_or(200);
        let text_width = if KNOPFONT.family == FontFamily::Monospace {
            KNOPFONT.size * label.len() as f32 * 0.6
        } else {
            let label_galley =
                painter.layout(label.clone(), KNOPFONT.clone(), KLEUR.get(), f32::INFINITY);
            label_galley.rect.max.x
        };
        let mut y_pos: f32 = 0.0;
        let mut x_pos: f32 = 0.0; // Menu boven links
        if MENU_POS.get() == 1 {
            x_pos = monitor_width as f32 - text_width - MARGE * 2.0 - STROKE_WIDTH
        } // Menu rechts boven
        if MENU_POS.get() == 2 {
            y_pos = monitor_height as f32 - YSIZE;
        } // Menu onder links
        if MENU_POS.get() == 3 {} // Menu links boven

        let positie: Pos2;
        //last_knop last_knop.rect.max.x;
        match last_knop {
            None => positie = Pos2 { x: x_pos, y: y_pos },
            Some(lk) => {
                volgnr = lk.volgnr + 1;
                positie = Pos2 {
                    x: if MENU_POS.get() % 2 != 0
                    /* 1 of 3 */
                    {
                        x_pos
                    } else {
                        lk.rect.max.x + SPATIE
                    },
                    y: if MENU_POS.get() % 2 == 0
                    /* 0 of 2 */
                    {
                        y_pos
                    } else {
                        volgnr as f32 * (YSIZE + 5.0)
                    },
                };
            }
        };
        let rect = Rect {
            min: positie,
            max: Pos2 {
                x: positie.x + text_width + MARGE * 2.0,
                y: positie.y + YSIZE,
            },
        };
        painter.rect(
            rect,
            0.0,
            contrastkleur(KLEUR.get(), false),
            Stroke {
                width: STROKE_WIDTH,
                color: KLEUR.get(),
            },
            egui::StrokeKind::Inside,
        );
        painter.text(
            Pos2 {
                x: positie.x + MARGE,
                y: positie.y,
            },
            Align2::LEFT_TOP,
            label.clone(),
            KNOPFONT.clone(),
            KLEUR.get(),
        );
        Knop { rect, volgnr }
    }

    pub fn bevat(&self, punt: Option<Pos2>) -> bool {
        match punt {
            None => {
                return false;
            }
            Some(pnt) => {
                if pnt.x > self.rect.min.x
                    && pnt.x < self.rect.max.x
                    && pnt.y > self.rect.min.y
                    && pnt.y < self.rect.max.y
                {
                    return true;
                }
            }
        }
        false
    }
}

///Denkbeeldige rechthoek die om alle beschikbare schermen heen getekend kan worden; niet meer ingebruik
pub fn _alle_monitors() -> Rect {
    // niet meer in gebruik
    // functie heeft '-> Rect' (rechthoek) als resultaat
    // 'Rect' wordt een denkbeeldige rechthoek die om alle beschikbare schermen heen getekend kan worden
    let screens = Monitor::all().unwrap(); // maak variabele met 'let ='
    let mut boven_links: Pos2 = Pos2::ZERO; // als later mutatie nodig is met 'let mut naam= '
    let mut onder_rechts: Pos2 = Pos2::ZERO;
    for screen in screens {
        // doorloop de Vector 'screens'
        if (screen.xpos()) < boven_links.x {
            // if <conditie ZONDER haakjes())>
            boven_links.x = screen.xpos() // de {expressie} staat altijd tussen {}
        }
        if (screen.ypos()) < boven_links.y {
            boven_links.y = screen.ypos()
        }
        if (onder_rechts.x) < screen.xpos() + screen.xsize() {
            onder_rechts.x = screen.xpos() + screen.xsize()
        }
        if (onder_rechts.y as f32) < screen.ypos() + screen.ysize() {
            onder_rechts.y = screen.ypos() + screen.ysize()
        }
    }
    //    println!("AS  {:?}  {:?}", links_boven, rechts_onder );
    Rect {
        // Rect is een 'struct' van twee posities, resp. min (=Li-boven) en max (=Re-onder)
        min: Pos2 { x: 0.0, y: 0.0 }, // Pos2 is een 'struct' van twee coordinaten, resp x en y
        max: Pos2 {
            // je kunt ook max samenstellen uit de x en de y van rechts_onder
            x: onder_rechts.x,
            y: onder_rechts.y,
        },
    } // de laatste expressie heeft geen ';'. Daarmee wordt deze 'Rect' het resultaat van de functie
}

/// Verwijder path-gedeelte van een full-path, zodat de kale filenaam overblijft
pub fn naam_zonder_pad(naam: &str) -> String {
    let separator_string = MAIN_SEPARATOR.to_string();
    let i = naam.rfind(&separator_string);
    let ii: usize;
    match i {
        None => {
            ii = 0;
        }
        Some(r) => {
            ii = r + 1;
        }
    }
    naam[ii..].to_string()
}

///get_string: hulp-functie voor opvragen van de waarde van een `LocalKey<RefCell<String>>` - zie globaal.rs
pub trait EigenTrait4LocalKey {
    fn get_string(&'static self) -> String;
}

impl EigenTrait4LocalKey for LocalKey<RefCell<String>> {
    fn get_string(&'static self) -> String {
        self.with(|v| v.clone()).into_inner()
    }
}

///get_pijlwz: hulp-functie voor opvragen van de waarde van een `LocalKey<RefCell<PijlWijziging>>`  - zie globaal.rs
pub trait Pijlwz4LocalKey {
    fn _get_pijlwz(&'static self) -> PijlWijziging;
}

impl Pijlwz4LocalKey for LocalKey<RefCell<PijlWijziging>> {
    fn _get_pijlwz(&'static self) -> PijlWijziging {
        self.with(|v| v.clone()).into_inner()
    }
}

fn read_dir_sorted_reversed<P: AsRef<Path>>(path: P) -> Result<Vec<DirEntry>, io::Error> {
    let mut paths = fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;
    paths.sort_by_key(|de| Reverse(de.metadata().unwrap().modified().unwrap()));
    Ok(paths)
}

/// Bepaal de volgende filenaam, door toevoegen van een volgnr;
/// tevens worden files ouder dan x dagen verwijderd;
/// Het gaat hierbij om resultaat-files die zijn opgeslagen in de directory 'Desktop/Schermen'
fn volgende_filenaam(naam_gezocht: &str) -> String {
    let mut map_pad = desktop_dir().unwrap_or("".into()).into_os_string(); // als een ander map_pad gewenst is
    let separator_string = MAIN_SEPARATOR.to_string();
    let lees_string = DESKTOP_MAP.get_string();
    map_pad.push(separator_string.clone());
    map_pad.push(lees_string.as_str());
    let mut resultaat: String;
    let _ = fs::create_dir(map_pad.clone()); // als bestaat: ook goed!
                                             // let _ = omdat resultaat van fs... niet wordt gebruikt; compiler waarschuwt dat dit zo hoort; _ betekent: ongebruikt
    let mut oudste = "".to_string();
    let mut jongst = "".to_string();
    let mut volgnr = 0;
    let f = fs::read_dir(map_pad.clone());
    let mut bewaardagen = 10;
    if f.is_ok() {
        let t = f.unwrap().count();
        if t > 30 {
            bewaardagen = 1
        };
        // als in desktop/schermen meer dan 30 files staan wordt de bewaartermijn gereduceerd tot 1 dag ipv 10 dagen
    };
    let mut bewerkte_beelden: Vec<String> = vec![];
    let mut delete_first = "".to_string();
    match read_dir_sorted_reversed(map_pad.clone()) {
        // jongste files eerst, na volgnr= 30 worden ze verwijderd
        // lees de directory 'desktop/Schermen'
        Err(waarom) => println!("Waarom: {:?}", waarom.kind()),
        Ok(pad) => {
            for mr in pad {
                let map_regel: Result<DirEntry, io::Error> = Ok(mr);
                match map_regel {
                    // lees alle files(regels) in de directory(map)
                    Err(_) => {}
                    Ok(regel) => {
                        let regel_meta = regel.metadata();
                        match regel_meta {
                            Err(_) => {}
                            Ok(meta_data) => {
                                if meta_data.is_file() {
                                    // onderzoek bestaande files in doel-directory
                                    let naam_bestaand = regel
                                        .file_name() // de directory-regel bevat een filenaam van het type OsString
                                        .into_string() // OsString omzetten in (String of Error)
                                        .expect("Geen naamstring"); //ontmantelen tot String
                                    if naam_bestaand.contains(&(BESTANDSNAAM.get_string() + "-")) {
                                        // bestanden die beginnen met BESTANDSNAAM
                                        if naam_bestaand.contains("-p-") == false {
                                            // als onbewerkte file: tellen
                                            volgnr += 1; // tel alleen de niet-bewerkte beelden
                                        } else {
                                            // als bewerkte file:
                                            bewerkte_beelden.push(naam_bestaand.clone())
                                            // verzamel alle -p- files
                                        }
                                    }
                                    let file_datum = if let Ok(dd) = meta_data.modified() {
                                        dd
                                    } else {
                                        SystemTime::now()
                                    };
                                    let file_ouderdom = SystemTime::now()
                                        .duration_since(file_datum)
                                        .unwrap()
                                        .as_secs(); // 3600;
                                    if file_ouderdom > 24 * 3600 * bewaardagen  // verwijder te oude files ; // 24 uur is een dag; 3600 sec is een uur
                                    || volgnr > MAX_AANTAL_GELIJKE_FILENAMEN
                                    // door sorteren op jongste, verwijderen de oudste files
                                    {
                                        // dit zou niet moeten voorkomen!
                                        remove_old_file(
                                            map_pad.clone().into_string().unwrap()
                                                + &separator_string.clone()
                                                + &naam_bestaand,
                                        );
                                    } else {
                                        if naam_gezocht.contains("-p-")
                                            == naam_bestaand.contains("-p")
                                        {
                                            // afhankelijk van of naam_gezocht -p- bevat of niet wordt vergeleken met soortgelijke files
                                            oudste = naam_bestaand.clone(); // dit wordt de oudste van de files
                                            if volgnr == MAX_AANTAL_GELIJKE_FILENAMEN {
                                                delete_first = naam_bestaand.clone();
                                            }
                                            // mogelijk zijn oudste en delete_first eigenlijk gelijk!
                                        }
                                        // is er een bestaande file met dezelfde naam en hetzelfde nummer en {bevat -p- als 'gezocht' bevat -p- en omgekeerd} ??
                                        if naam_bestaand.starts_with(naam_gezocht) // begint de String met 'naam'?
                                        && naam_bestaand.ends_with(".png") // eindigt de string op .png
                                            // zoek naar files-met-p- als (naam_gezocht bevat -p-) en files-zonder-p- als (naam bevat geen -p-)
                                        && naam_gezocht.contains("-p-") == naam_bestaand.contains("-p-")
                                        // omgekeerd
                                        {
                                            if jongst.len() == 0 {
                                                jongst = naam_bestaand;
                                            }
                                        }
                                    }
                                } // is_file;  'else' zijn meta_data.is_dir(), meta_data.is_symlink(), en die zijn niet belangrijk
                            }
                        }
                    }
                }
            }
            for bewerkt in bewerkte_beelden {
                let naam_zonder_type = zonder_type(&delete_first); //
                if naam_zonder_type.len() > 0 && bewerkt.starts_with(&naam_zonder_type) {
                    remove_old_file(
                        map_pad.clone().into_string().unwrap()
                            + &separator_string.clone()
                            + &bewerkt,
                    );
                }
            }
        }
    }
    // situatie 1: geen oude files => resultaat= jongst= ""
    // situatie 2: weinig oude files => jongst = "Scherm-001.png"
    // situatie 3: teveel files => file_removed = true;
    // situatie 4: precies genoeg files => file_removed = true, oudste file-naam gebruiken, nummer niet verhogen
    if volgnr>=MAX_AANTAL_GELIJKE_FILENAMEN     // dit geldt voor onbewerkte beelden, daarvan mogen er een max aantal zijn
    && naam_gezocht.contains("-p-")==false
    {
        // dit geldt voor bewerkte beelden; die worden gewist als het bijbehorende onbewerkte wordt gedelete
        resultaat = oudste.clone(); // situatie 4
    } else {
        resultaat = jongst.clone(); // situatie 1 en 2
    }
    if resultaat.contains(naam_gezocht) == false {
        // nog geen eerdere filenaam
        resultaat = naam_gezocht.to_string()
    };
    if resultaat.len() == 0 {
        resultaat = naam_gezocht.to_string(); // maak filenaam
        if !resultaat.ends_with("-") {
            // voeg '-' toe als dat er nog niet is
            resultaat = resultaat + "-";
        }
    }
    resultaat = map_pad.into_string().unwrap() + &separator_string + &resultaat; // naam met path
    resultaat = bepaal_volgnr(
        // bepaal volgnr
        &resultaat,
        volgnr < MAX_AANTAL_GELIJKE_FILENAMEN || resultaat.contains("-p-"), //verhoog volgnr als het max aantal nog niet is bereikt; -p- heeft geen max
    );
    resultaat
}

/// aan de filenaam wordt de extensie .png toegevoegd
fn met_type(naam: &str) -> String {
    if !naam.contains(".") {
        return naam.to_owned() + ".png";
    }
    naam.to_string()
}

/// Verwijderen van een file
fn remove_old_file<P: AsRef<Path>>(pad: P) {
    match remove_file(pad) {
        Ok(_) => {}
        Err(e) => {
            println!("ERROR remove file {:?}", e);
        }
    };
}

/// de extensie van een filenaam wordt verwijderd
fn zonder_type(naam: &str) -> String {
    let resultaat;
    let i = naam.find(".");
    match i {
        None => resultaat = naam.to_string(),
        Some(ii) => resultaat = naam[..ii].to_string(),
    }
    resultaat
}

/// resultaat = volgnr uit een filenaam
fn volgnr_uit_filenaam(naam: &str) -> Result<usize, std::num::ParseIntError> {
    if naam == "" {
        return naam.to_string().parse::<usize>();
    };
    let romp = zonder_type(naam);
    let mut i = romp.len() - 1;
    while i > 0 && romp.chars().nth(i).unwrap_or('q').is_digit(10) {
        // ****** die 'q'is niet zo sterk
        i = i - 1;
    }
    i += 1;
    let getal_string = romp[i..].to_string();
    getal_string.parse::<usize>()
}

/// filenaam na vertwijderen van extensie en volgnummer   
pub fn neem_romp(naam: &str) -> String {
    let romp = zonder_type(naam);
    let mut i = romp.len();
    while i > 0 && romp.chars().nth(i - 1).unwrap_or('q').is_digit(10) {
        // ****** die 'q'is niet zo sterk
        i = i - 1;
    }
    let romp_string = romp[..i].to_string();
    romp_string
}

/// bepaal voor een nieuwe file een opvolgend volgnummer, na 1000 terug naar 1,
fn bepaal_volgnr(naam: &str, verhogen: bool) -> String {
    let volgnr: usize;
    match volgnr_uit_filenaam(naam) {
        Ok(getal) => {
            if verhogen {
                volgnr = (getal + 1) % 1000; // nooit hoger dan 100
            } else {
                volgnr = getal;
            }
        }
        _ => {
            volgnr = 1;
        }
    }
    let romp = neem_romp(naam);
    met_type(&(romp + format!("{:03}", volgnr).as_str()))
}

/// Maak een naam voor een bewerkte file (met toegevoegde pijlen en randen) door toevoegen van -p- aan de naam
pub fn maak_pijl_naam(naam: &str) -> String {
    // een file waaraan pijlen of rand zijn toegevoegd krijgt '-p-' toegevoegd aan de naam
    zonder_type(naam).to_string() + "-p-"
}

/// grootste van de beschikbare monitor-schermen
pub fn grootstescherm() -> Monitor {
    let monitors = Monitor::all().unwrap(); // maak variabele met 'let ='
    let mut result = hoofdscherm();
    let mut largesize = result.xsize() * result.ysize();
    for i in 0..monitors.len() {
        //println!("MON {:?} {:?} {:?} {:?} {:?} {:?}", monitors[i].xsize(), monitors[i].ysize(),monitors[i].xpos(), monitors[i].ypos(),
        //monitors[i].is_builtin(),monitors[i].is_primary());
        let ls = monitors[i].xsize() * monitors[i].ysize();
        if ls > largesize {
            largesize = ls;
            result = monitors[i].clone();
        };
    }
    result
}

/// Hoofdscherm van de PC ('primary')
pub fn hoofdscherm() -> Monitor {
    let monitors = Monitor::all().unwrap(); // maak variabele met 'let ='
    let mut hoofd = monitors[0].clone();
    for monitor in &monitors[1..] {
        if monitor.primary() {
            hoofd = monitor.clone();
        }
    }
    hoofd
}

/// De andere Monitor, alleen wanner met twee monitoren wordt gewerkt
pub fn ander_scherm(scherm: Monitor) -> Monitor {
    let monitors = Monitor::all().unwrap(); // maak variabele met 'let ='
    let mut gewijzigd = scherm.clone();
    for monitor in monitors {
        if monitor.ident() != scherm.ident() {
            gewijzigd = monitor
        }
    }
    if ENKELSCHERM.get() {
        gewijzigd = DOELSCHERM.with(|v| v.clone()).into_inner();
    }
    gewijzigd
}

///Opvragen eigenschappen vannde andere Monitor
pub fn anderscherminfo() -> Monitor {
    ander_scherm(DOELSCHERM.with(|v| v.clone()).into_inner())
}

///Opvragen eigenschappen van de monitor waarop gewerkt wordt
pub fn doelscherminfo() -> Monitor {
    DOELSCHERM.with(|v| v.clone()).into_inner()
}

pub fn muis_naar_monitor_midden(scherm: Monitor) {
    let mouse = Mouse::new();
    let _ = mouse.move_to(
        scherm.x().unwrap() + scherm.width().unwrap() as i32 / 2,
        scherm.y().unwrap() + scherm.height().unwrap() as i32 / 2,
    );
}

/*
pub fn twee_monitors() -> bool {
    let monitors = Monitor::all().unwrap(); // maak variabele met 'let ='
    monitors.len() > 1
}
*/

/// bewaar een rechthoek van het monitorscherm in een file met de aangegeven naam
pub fn bewaar_scherm_regio(rect: Rect, naam: String, rand: u32, uit_geheugen: bool) -> bool {
    // resultaat= succes
    //if HEEL_SCHERM.get() {rand= 0;}
    let punt_1 = Pos2 {
        x: rect.min.x,
        y: rect.min.y,
    };
    let punt_2 = Pos2 {
        x: rect.max.x,
        y: rect.max.y,
    };
    match monitor_bevat_beide_punten(punt_1, punt_2) {
        Some(mon) => {
            let monitorimage = if uit_geheugen {
                // is dit nodig of is er een eenvoudiger oplossing?|
                match BEELD_OPTION.with(|v| v.clone()).into_inner() {
                    Some(bop) => bop,
                    None => mon.capture_image().unwrap(),
                }
            } else {
                mon.capture_image().unwrap()
            };
            let capture_area = monitorimage
                .view(
                    (punt_1.x - mon.xpos()) as u32,
                    (punt_1.y - mon.ypos()) as u32,
                    (punt_2.x - punt_1.x) as u32,
                    (punt_2.y - punt_1.y) as u32,
                )
                .to_image();
            let mut naam_string = volgende_filenaam(&naam);
            if naam_string.len() == 0 {
                naam_string = naam.to_string();
            }
            if !naam_string.contains("-p-") {
                LAATSTEBEELD.set(naam_string.to_string());
            }
            if !naam_string.contains(".png") {
                naam_string = naam_string + ".png";
            }
            // een imagebuffer in randkleur, iets groter dan capture_area wordt gemaakt
            let mut met_beeldrand = RgbaImage::from_pixel(
                capture_area.width() + rand * 2,
                capture_area.height() + rand * 2,
                Rgba(RANDKLEUR.get().to_array()),
            );
            // capture_area wordt over de met_beeld_rand heen gelegd; zodat met_beeldrand wordt [capture_area met de gewenste rand ]
            overlay(&mut met_beeldrand, &capture_area, rand as i64, rand as i64);
            if met_beeldrand.save(naam_string.as_str()).is_ok() {
                // .save slaat de imagebuffer op, op het gegeven pad, in het formt dat in de extensie van het path is opgegeven 
                let start = naam_string.find(&DESKTOP_MAP.get_string());
                let korte_naam = naam_string[start.unwrap_or(0)..].to_string();
                println!("NAAM {:?}", naam_string.as_str());
                if naam_string.contains("-p-") {
                    LAATSTE_PIJL_PATH.set(korte_naam);
                } else {
                    LAATSTE_FILE_PATH.set(korte_naam);
                }
            }
        }
        None => {
            println!("punten niet in zelfde scherm");
            return false;
        }
    }
    true
}

/// Monitor::omvat: in welke `Option<Monitor>` ligt het punt; None= niet in een Monitor
pub trait EigenTraitTbvMonitor {
    fn omvat(&self, pos: Pos2) -> Option<Monitor>;
}

impl EigenTraitTbvMonitor for Monitor {
    fn omvat(&self, pos: Pos2) -> Option<Monitor> {
        let monitors: Vec<Monitor> = Monitor::all().unwrap();
        let mut found = false;
        for monitor in monitors {
            if monitor.ident() == self.ident() {
                found = true;
                break;
            }
        }
        if found
            && pos.x >= self.xpos()
            && pos.x < (self.xpos() as i32 + self.xsize().round() as i32) as f32
            && pos.y >= (self.ypos()) as f32
            && pos.y < (self.ypos() as i32 + self.ysize().round() as i32) as f32
        {
            return Some(self.clone());
        } else {
            return None;
        }
    }
}

///Welke `option<monitor>` bevat deze beide punten; None= niet in een Monitor
pub fn monitor_bevat_beide_punten(pos_libo: Pos2, pos_reon: Pos2) -> Option<Monitor> {
    let mut result: Option<Monitor> = None;
    let monitors = Monitor::all().unwrap();
    for monitor in monitors {
        if monitor.omvat(pos_libo).is_some() && monitor.omvat(pos_reon).is_some() {
            result = Some(monitor.clone());
        }
    }
    result
}

///Een kleur die zeker contrasteert met een andere kleur
pub fn contrastkleur(kleur: Color32, zwart_wit: bool) -> Color32 {
    //kun je testen met fn contrastkleurtest()
    let mut result = Color32::from_rgba_unmultiplied(kleur.b(), kleur.r(), kleur.g(), 255);
    if zwart_wit {
        result = if kleur.r() as i32 + kleur.g() as i32 + kleur.b() as i32 > 127 {
            Color32::BLACK
        } else {
            Color32::WHITE
        };
        return result;
    }
    if (kleur.r() as i32 - kleur.g() as i32).abs()
        + (kleur.g() as i32 - kleur.b() as i32).abs()
        + (kleur.b() as i32 - kleur.r() as i32).abs()
        < 255
    {
        result = Color32::from_rgba_unmultiplied(!kleur.a(), !kleur.g(), !kleur.b(), 255);
    }
    if (kleur.r() as i32 - kleur.g() as i32).abs()
        + (kleur.g() as i32 - kleur.b() as i32).abs()
        + (kleur.b() as i32 - kleur.r() as i32).abs()
        < 255
    {
        // veel te weinig verschil tussen r, g en b -> verschil aanbrengen
        result = Color32::from_rgba_unmultiplied(
            if kleur.r() < kleur.g() && kleur.r() < kleur.b() {
                255
            } else {
                128
            },
            if kleur.g() < kleur.r() && kleur.g() < kleur.b() {
                255
            } else {
                64
            },
            if kleur.b() < kleur.g() && kleur.b() < kleur.r() {
                255
            } else {
                0
            },
            255,
        ); // grijs wordt bruin en contrasteert duidelijk
           //ik zou nog eens een blok moeten maken met veel kleuren en daarin een contrasterend accent
    }
    result
}

pub fn _contrastkleurtest(ctx: Context) {
    //* contraskleurtest - alle kleuren
    let painter = ctx.layer_painter(LayerId::new(Order::Background, Id::new("contrasten")));
    for a in 0..24 {
        for b in 0..16 {
            let mut rd = (16 - a * 2) * 16 - 1;
            if a > 16 {
                rd = ((a * 32) as i32).abs();
            };
            if rd < 0 {
                rd = 0
            };
            let mut bl = (a * 2 - 16) * 16 - 1;
            if a > 16 {
                bl = (32 - a) * 32;
            };
            if bl < 0 {
                bl = 0;
            };
            let mut gr = 255 - rd - bl;
            if a > 15 {
                gr = 0;
            }
            let maxrgb = max(rd, max(gr, bl));
            let mut rdx = rd * 255 / maxrgb;
            let mut grx = gr * 255 / maxrgb;
            let mut blx = bl * 255 / maxrgb;
            rdx = rdx * (16 - b) / 16 + 128 * b / 16;
            grx = grx * (16 - b) / 16 + 128 * b / 16;
            blx = blx * (16 - b) / 16 + 128 * b / 16;
            painter.rect_filled(
                Rect {
                    min: Pos2 {
                        x: a as f32 * 60.0,
                        y: b as f32 * 60.0,
                    },
                    max: Pos2 {
                        x: a as f32 * 60.0 + 50.0,
                        y: b as f32 * 60.0 + 50.0,
                    },
                },
                0.0,
                Color32::from_rgba_unmultiplied(rdx as u8, grx as u8, blx as u8, 255),
            );
            painter.rect_filled(
                Rect {
                    min: Pos2 {
                        x: a as f32 * 60.0 + 20.0,
                        y: b as f32 * 60.0 + 10.0,
                    },
                    max: Pos2 {
                        x: a as f32 * 60.0 + 40.0,
                        y: b as f32 * 60.0 + 40.0,
                    },
                },
                0.0,
                contrastkleur(
                    Color32::from_rgba_unmultiplied(rdx as u8, grx as u8, blx as u8, 255),
                    true,
                ),
            );
            //if b==0 {println!("KL {} {} {} {}", rd as u8, gr as u8, bl as u8, maxrgb);}
            //if b==0 {println!("KX {} {} {} {} {:?}", rdx as u8, grx as u8, blx as u8, minrgb, contrastkleur(Color32::from_rgba_unmultiplied(rdx as u8, grx as u8, blx as u8,255),true) );}
        }
    }
    return;
    // */
}

pub fn rect_dashed(painter: Painter, rect: Rect) {
    painter.add(Shape::dashed_line( //Shape::Path(eframe::egui::epaint::PathShape {
        &vec![
            rect.min,
            Pos2 {x: rect.max.x-1.0, y: rect.min.y},
            rect.max,
            Pos2 {x: rect.min.x, y: rect.max.y-1.0},
            rect.min,
        ],
        Stroke{width: 1.0, color: Color32::WHITE},
        5.0,
        5.0
    ));
}

pub fn line_dashed(painter: Painter, start: Pos2, end: Pos2 ) {
    painter.add(Shape::dashed_line( //Shape::Path(eframe::egui::epaint::PathShape {
        &vec![start, end],
        Stroke{width: 1.0, color: Color32::WHITE},
        5.0,
        5.0
    ));
}

/// Pijl tonen in venster, met label bij begin en punt aan einde
pub fn pijltonen(zelf: &mut PijlenApp, painter: Painter, pijl: Pijltje, index: usize, rect_min: Pos2) -> (Pos2, Rect) {
    //let wijzig_punt = PIJL_WIJZIGING
    //    .with(|v| v.clone())
    //    .into_inner()
    //    .unwrap_or(PijlWijziging::default());
    let wijzig_punt = zelf.pijl_bewerker;
    let pijltip = plaats_in_scherm(pijl.tip, rect_min);
    let pijlrug = plaats_in_scherm(pijl.rug, rect_min);
    // pijl.tip en pijl.rug zijn t.o.v. het kleine beeldrect ( de plaats waar het te bewerken beeld staat op het scherm)
    // pijltip en pijlrug zijn t.o.v. het hele scherm
    let mut pijlkleur = pijl.kleur;
    if wijzig_punt.pijlnummer.is_some() && wijzig_punt.pijlnummer.unwrap() == index {
        pijlkleur = pijl.kleur;
    }
    let puntgrootte = 7.0;
    let lijn_type = Stroke {
        width: 3.0,
        color: pijlkleur,
    };
    let hoek = Vec2 {
        x: pijltip.x - pijlrug.x,
        y: pijltip.y - pijlrug.y,
    }
    .angle(); // pijl naar rechts = 0.0 pijl linksom wordt negatief tot -3.14 als pijl naar links wijst; pijl rechtsom positief tot +3.14 als pijl naar links wijst
    let achter_punt = Pos2 {
        x: pijltip.x - puntgrootte * hoek.cos(),
        y: pijltip.y - puntgrootte * hoek.sin(),
    };
    if pijltip!=pijlrug {      
    painter.line_segment([pijlrug, achter_punt], lijn_type);
    painter.add(Shape::Path(eframe::egui::epaint::PathShape {
        points: vec![
            pijltip,
            Pos2 {
                x: achter_punt.x + puntgrootte * 0.7 * hoek.sin(),
                y: achter_punt.y - puntgrootte * 0.7 * hoek.cos(),
            },
            Pos2 {
                x: achter_punt.x - puntgrootte * 0.7 * hoek.sin(),
                y: achter_punt.y + puntgrootte * 0.7 * hoek.cos(),
            },
        ],
        closed: true,
        fill: pijlkleur, // Color32::TRANSPARENT => open pijlpunt
        stroke: lijn_type.into(),
    }));
    }
    //let (uiterste, rechthoek) = bijschrift_uiterste(painter, pijl, rect_min);
    //uiterste
    teken_bijschrift(painter, pijl, rect_min)
}

fn teken_bijschrift(
    painter: Painter,
    pijl: Pijltje,
    rect_min: Pos2,
) -> (Pos2, Rect) {
    let pijltip = plaats_in_scherm(pijl.tip, rect_min);
    let pijlrug = plaats_in_scherm(pijl.rug, rect_min);
    let hoek = if pijltip==pijlrug {PI} else{
        Vec2 {
        x: pijltip.x - pijlrug.x,
        y: pijltip.y - pijlrug.y,
    }.angle()}; 
    // pijl naar rechts = 0.0 pijl linksom wordt negatief tot -3.14 als pijl naar links wijst; pijl rechtsom positief tot +3.14 als pijl naar links wijst
    let font = FontId::monospace(PIJL_TEKST_GROOT.get() as f32);
    let tekst_anker = 
        if hoek >= 0.0 {
            // pijl naar beneden
            if hoek < PI / 2.0 {  // pijl naar links
                Align2::RIGHT_BOTTOM
            }
            else {                // pijl naar rechts
                Align2::LEFT_BOTTOM
            }
        } else {
            // pijl naar boven
            if hoek > -PI / 2.0 {  //pijl naar links 
                Align2::RIGHT_TOP
            }
            else {                 // pijl naar rechts
                Align2::LEFT_TOP
            }
        };
    let tekst_omvang =
        Painter::layout_no_wrap(&painter, pijl.pijltekst.clone(), font.clone(), KLEUR.get()).size();
    let uiterste;
    let mut rechthoek = Rect {
        min: Pos2 { x: 10.0, y: 10.0 },
        max: Pos2 { x: 100.0, y: 100.0 },
    };
    match tekst_anker {
        Align2::RIGHT_TOP => {
            //uiterste = Pos2 {
            //    x: pijl.rug.x - tekst_omvang.x,
            //    y: pijl.rug.y + tekst_omvang.y,
            //};
            rechthoek = Rect {
                min: plaats_in_scherm(
                    Pos2 {
                        x: pijl.rug.x - tekst_omvang.x,
                        y: pijl.rug.y,
                    },
                    rect_min,
                ),
                max: plaats_in_scherm(
                    Pos2 {
                        x: pijl.rug.x,
                        y: pijl.rug.y + tekst_omvang.y,
                    },
                    rect_min,
                ),
            };
            uiterste = plaats_in_rect(
                Pos2 {
                    x: rechthoek.min.x,
                    y: rechthoek.max.y,
                },
                rect_min,
            );
        }
        Align2::LEFT_TOP => {
            //uiterste = Pos2 {
            //    x: pijl.rug.x + tekst_omvang.x,
            //    y: pijl.rug.y + tekst_omvang.y,
            //};
            rechthoek = Rect {
                min: plaats_in_scherm(
                    Pos2 {
                        x: pijl.rug.x,
                        y: pijl.rug.y,
                    },
                    rect_min,
                ),
                max: plaats_in_scherm(
                    Pos2 {
                        x: pijl.rug.x + tekst_omvang.x,
                        y: pijl.rug.y + tekst_omvang.y,
                    },
                    rect_min,
                ),
            };
            uiterste = plaats_in_rect(rechthoek.max, rect_min);
        }
        Align2::RIGHT_BOTTOM => {
            //uiterste = Pos2 {
            //    x: pijl.rug.x - tekst_omvang.x,
            //   y: pijl.rug.y - tekst_omvang.y,
            //};
            rechthoek = Rect {
                min: plaats_in_scherm(
                    Pos2 {
                        x: pijl.rug.x - tekst_omvang.x,
                        y: pijl.rug.y - tekst_omvang.y,
                    },
                    rect_min,
                ),
                max: plaats_in_scherm(
                    Pos2 {
                        x: pijl.rug.x,
                        y: pijl.rug.y,
                    },
                    rect_min,
                ),
            };
            uiterste = plaats_in_rect(rechthoek.min, rect_min);
        }
        Align2::LEFT_BOTTOM => {
            //uiterste = Pos2 {
            //    x: pijl.rug.x + tekst_omvang.x,
            //    y: pijl.rug.y - tekst_omvang.y,
            //};
            rechthoek = Rect {
                min: plaats_in_scherm(
                    Pos2 {
                        x: pijl.rug.x,
                        y: pijl.rug.y - tekst_omvang.y,
                    },
                    rect_min,
                ),
                max: plaats_in_scherm(
                    Pos2 {
                        x: pijl.rug.x + tekst_omvang.x,
                        y: pijl.rug.y,
                    },
                    rect_min,
                ),
            };
            uiterste = plaats_in_rect(
                Pos2 {
                    x: rechthoek.max.x,
                    y: rechthoek.min.y,
                },
                rect_min,
            );
        }
        _ => uiterste = pijl.rug,
    }
    if pijl.kleur==Color32::BLACK || pijl.kleur==Color32::WHITE {
        painter.rect_filled(
            rechthoek,
            0.0,
            if pijl.kleur==Color32::BLACK {Color32::WHITE} else {Color32::BLACK}
        );
    }
    painter.text(
        plaats_in_scherm(pijl.rug, rect_min),
        tekst_anker,
        pijl.pijltekst,
        font,
        pijl.kleur,
    );
    (uiterste, rechthoek)
}

/// Laad het te bewerken scherm-beeld in het pijlen_venster, om pijlen, randen en bijschrift toe te voegen
pub fn load_image_from_path(path: &std::path::Path) -> Result<ColorImage, ImageError> {
    let image = ImageReader::open(path)?.decode()?;
    //let image= image.resize(image.width()*2/3, image.height()*2/3, FilterType::Lanczos3);
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}

///Conversie xcap::image::ImageBuffer naar eframe::epaint::ColorImage
pub fn image_buffer_to_color_image(
    imbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
) -> eframe::epaint::ColorImage {
    ColorImage::from_rgba_unmultiplied(
        [imbuf.width() as usize, imbuf.height() as usize],
        &imbuf.into_vec(),
    )
}

///Conversie van eframe::epaint::ColorImage naar xcap::image::ImageBuffer
pub fn color_image_to_image_buffer(
    colim: eframe::epaint::image::ColorImage,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let wh = colim.size;
    let result: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::from_raw(wh[0] as u32, wh[1] as u32, colim.as_raw().to_owned())
            .unwrap();
    result
}

pub fn lees_huidige_scherminhoud() {
    // scherminhoud wordt als imagebuffer verkregen mbv capture_image
    // en als zodanig opgeslagen in BEELD_OPTION
    // we gebruiken hem later meestal als TEXTURE
    // imagebuffer_to_texture zorgt voor de conversie en opslag in BEELD_OPTION_TEXTURE
    BEELD_OPTION_TEXTURE.set(None); // BEELD_OPTION_TEXTURE leeg maken
    let doelscherm = DOELSCHERM.with(|v| v.clone()).into_inner();
    let imagebuffer = doelscherm.capture_image();
    match imagebuffer {
        Err(_) => {
            println!("SCHERM NIET GEZIEN");
            return;
        }
        Ok(imgb) => {
            BEELD_OPTION.set(Some(imgb));
        }
    }
}

pub fn imagebuffer_to_texture(ctx: Context) -> Option<TextureHandle> {
    let mut option_texture = BEELD_OPTION_TEXTURE.with(|v| v.clone()).into_inner();
    if false && option_texture.is_some() {
        println!("option_tecture yes");
        return option_texture;
    } else {
        let imagebuffer = BEELD_OPTION.with(|v| v.clone()).into_inner();
        match imagebuffer {
            None => {
                println!("NO-image");
                BEELD_OPTION.set(None);
                BEELD_OPTION_TEXTURE.set(None);
                return None;
            }
            Some(imgb) => {
                let monitorimage = image_buffer_to_color_image(imgb);
                option_texture = Some(ctx.load_texture("schot", monitorimage, Default::default()));
                if option_texture.is_none() {
                    return None;
                }
                BEELD_OPTION_TEXTURE.set(option_texture.clone());
                return option_texture;
            }
        }
    }
}

/// Ui::markering_kleur: standaardkleur voor markeringen zoals pijlen en randen etc
pub trait EigenTraitTbvUi {
    /// bepaal (in het optie-venster) een kleur voor pijlen etc
    fn markering_kleur(&mut self, kleur: &mut Color32);
}

impl EigenTraitTbvUi for Ui {
    fn markering_kleur(&mut self, kleur: &mut Color32) {
        self.horizontal(|ui| {
            ui.label("Kies een kleur voor pijlen etc.");
                let dashed= DASHED.get(); 
                let dashed= "DASHED".to_string() + {if dashed {"-Y"} else {"-N"}};
                if ui.button(dashed.as_str()).clicked() {
                    DASHED.set(DASHED.get()==false);
                }
        });   
        self.horizontal(|ui| {
            color_picker_color32(ui, kleur, Alpha::Opaque);
            ui.vertical(|ui| {
                ui.label("");
                if ui.button("ROOD").clicked() {
                    *kleur = Color32::RED;
                }
                if ui.button("ORANJE").clicked() {
                    *kleur = Color32::from_rgb(255, 128, 0);
                }
                if ui.button("GEEL").clicked() {
                    *kleur = Color32::YELLOW;
                }
                if ui.button("GROEN").clicked() {
                    *kleur = Color32::GREEN;
                }
                if ui.button("BLAUW").clicked() {
                    *kleur = Color32::BLUE;
                }
                if ui.button("ZWART").clicked() {
                    *kleur = Color32::BLACK;
                }
                if ui.button("WIT").clicked() {
                    *kleur = Color32::WHITE;
                }
            })
        });
    }
}

/// Met een commandline argument wordt een bestaande filenaam aangegeven die een beeld bevat waaraan randen of pijlen meoten worden teogevoegd;
/// Het bepalen van een scherm-selectie wordt dan overgeslagen.
pub fn cmd_line_argumenten() {
    let args: Vec<String> = env::args().collect();
    println!("My path is {}.", args[0]);
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[0..]);
    //regedit: OpenWithList is in MS-windows de lijst van progrs die een file van een bepaalde extensie mogen openen (vereist meer onderzoek!)
    //gebruik om dit te wijzigen: defaultprogrameditor
    if args.len() > 1 {
        // wanneer op de commandline aan de programmanaam een filenaam (bijv tekening.png) is toegevoegd
        // wordt deze file geopend om van pijlen-met-bijschrift te voorzien
        // de functie om screenshots te maken wordt dan overgeslagen
        {
            //let mut bewaker_naam_schrijf = LAATSTEBEELD.write().unwrap();
            //*bewaker_naam_schrijf = args[1].to_string(); // LAATSTEBEELD wordt overschreven met arg[1]
            LAATSTEBEELD.set(args[1].to_string());
            BESTAAND_BEELD.set(true);
        }
    }
}

pub fn monitor_wissel_functie() {
    if MONITOR_WISSEL.get()== false {return;}
    BEELD_OPTION.set(None);
    DOELSCHERM.set(ander_scherm(DOELSCHERM.with(|v| v.clone()).into_inner()));
    MONITOR_WISSEL.set(false);
    MAAK_UITSNEDE.set(true);
    OPTIES_ON.set(true);
    BERICHT.set(vec![]);
}

pub fn pos_in_rect(pos: Pos2, rect: Rect) -> bool {
    //println!("PIR {} {} {}", rect.min, rect.max, pos);
    pos.x>rect.min.x && pos.x<rect.max.x && pos.y>rect.min.y && pos.y<rect.max.y 
}

fn sqr(x: f32) -> f32 { x * x}

/// Is het punt dichtbij een reeds getekende pijl?
/// resultaat= (0=ja/nee, 2=volgnr van de pijl, 3=dichtbij_pijlstart, 4=dichtbij_pijleinde, 5=dichtbij_pijlschacht);
/// 3,4 wordt gebruikt voor verplaatsen van start of einde, 5 wordt gebruikt voor verwijderen of kleurverandering
//pub fn dichtbij(punt: Pos2) -> (bool, usize, bool, bool, bool) {
pub fn dichtbij(punt: Pos2, rect_min: Pos2) -> PijlWijziging {
    // dit zou eenvoudiger een Option<Pijlwijziging> kunnen worden}
    let mut resultaat = PijlWijziging::default();
    // (cursor_op_pijl, volgnr_van_de_pijl, start, einde, schacht)
    let pijlen = PIJLENKOKER.take();
    if pijlen.len() == 0 {
        return PijlWijziging::default();
    }
    let mut het_is = 9999;
    let mut is_start = false;
    let mut is_einde = false;
    let mut is_schacht = false;
    // vind de betreffende pijl in PIJLENKOKER
    for volgnr in 0..pijlen.len() {
        let pijl = &pijlen[volgnr];
        //let pijltip = herplaats(pijl.tip, rect_min);
        //let pijlrug = herplaats(pijl.rug, rect_min);
        let pijltip = pijl.tip;
        let pijlrug = pijl.rug;
        let prx = pijlrug.x;
        let pry = pijlrug.y;
        let ptx = pijltip.x;
        let pty = pijltip.y;
        let mut xkans = false;
        let mut ykans = false;
        if pos_in_rect(plaats_in_scherm(punt, rect_min), pijl.text_rect) {
            if pijltip==pijlrug {
                het_is= volgnr;
                is_schacht= true;
            } else {
                het_is = volgnr;
                is_start = true;
            }
        } else {
            if sqr(punt.x - prx) + sqr(punt.y - pry) < 100.0 {
                het_is = volgnr;
                is_start = true;
            } else if sqr(punt.x - ptx) + sqr(punt.y - pty) < 100.0 {
                het_is = volgnr;
                is_einde = true;
            }else 
                if prx < ptx {
                    // alleen als de x-waarde van de cursor binnen de x-waarden van de pijl liggen kan hij de pijl raken
                    if punt.x >= prx - 10.0 && punt.x <= ptx + 10.0 {
                        xkans = true;
                    }
                } else {
                    // idem voor als de pijl van links naar rechts wijst
                    if punt.x <= prx + 10.0 && punt.x >= ptx - 10.0 {
                        xkans = true;
                    }
                }
                if pry < pty {
                    // alleen als de x-waarde van de cursor binnen de y-waarden van de pijl liggen kan hij de pijl raken
                    if punt.y >= pry - 10.0 && punt.y <= pty + 10.0 {
                        ykans = true;
                    }
                } else {
                    // idem voor als de pijl van onder naar boven wijst
                    if punt.y <= pry + 10.0 && punt.y >= pty - 10.0 {
                        ykans = true;
                    }
                }
                if xkans && ykans {
                    let mut a = (pry - pty) / (prx - ptx); // tangens van de hoek tussen x-as en pijl
                    if a.abs() > 100.0 {
                        a = 100.0
                    }; // voorkom delen door nul
                       // pijl ligt op de lijn y= a*x + b  ; b wordt berekend uit het punt (pxs,pys) op deze lijn
                    let b = pry - a * prx;
                    if a.abs() < 0.01 {
                        a = 0.01;
                    } // voorkom delen door nul
                    let c = punt.y + punt.x / a; // loodljn vanuit punt: y= -1/a.x + c
                    let sx = a / (a * a + 1.0) * (c - b);
                    let sy = a * sx + b;
                    let snijpunt = Pos2 { x: sx, y: sy };
                    let naast_pijl_kwadr =
                        ((snijpunt.x - punt.x).powi(2)) + ((snijpunt.y - punt.y).powi(2));
                    if naast_pijl_kwadr < 100.0 {
                        is_schacht = true;
                        het_is = volgnr;
                    }
                }
            }
        }
        
    PIJLENKOKER.set(pijlen);
    //(het_is < 100, het_is, is_start, is_einde, is_schacht)

    if het_is < 9999 {
        resultaat.pijlnummer = Some(het_is)
    }    
    resultaat.staart = is_start;
    resultaat.punt = is_einde;
    resultaat.schacht = is_schacht;
    resultaat
}
