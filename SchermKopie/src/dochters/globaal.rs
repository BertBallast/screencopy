// zie: https://www.sitepoint.com/rust-global-variables/

use crate::dochters::{gereedschap::Pijltje, text_input::TekstEnType};
use eframe::egui::{Color32, ColorImage, Pos2, Rect, TextureHandle, ViewportId};
use ferrishot_xcap::Monitor;
use image::{ImageBuffer, Rgba};
use std::cell::{Cell, RefCell};
//use screenshots::display_info::DisplayInfo as Monitor;

#[derive(Copy, Clone, PartialEq)]
pub enum HandleidingGebruik {
    Always,
    Protocol,
    Never,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MenuType {
    Popup,
    Rand,
    Beide,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Gestart {
    Eerste,
    Tweede,
    Running
}

thread_local! {   // benaderen met NAAM.get()  of NAAM.set(value) - heeft in elke thread een eigen waarde
// RefCell ipv get() converteren met .with(|v|{v.clone()}).into_inner();
// meer uitleg nodig *****
// RefCell<String> converteren met .getstr()
// zie ook 'std::thread::LocalKey'
pub static ANDERE_SCHERM: Cell<bool> = Cell::new(false);
pub static TIKKER: Cell<bool> = Cell::new(false);
pub static BAGGER: Cell<bool> = Cell::new(false);
pub static BEELD_SELECTIE: RefCell<Option<TextureHandle>> = RefCell::new(None);
pub static BEELD_OPTION_TEXTURE: RefCell<Option<TextureHandle>> = RefCell::new(None);
pub static BEELD_OPTION: RefCell<Option<ImageBuffer<Rgba<u8>, Vec<u8>>>>= RefCell::new(None);
pub static BERICHT: RefCell<Vec<String>> = RefCell::new(vec![]);
pub static BERICHTJE: RefCell<Vec<String>> = RefCell::new(vec![]);
pub static BESTAAND_BEELD: Cell<bool> = Cell::new(false);
pub static BESTANDSNAAM: RefCell<String> = RefCell::new("Scherm".to_string());
pub static DESKTOP_MAP: RefCell<String> = RefCell::new("Schermen".to_string());
pub static DASHED: Cell<bool> = Cell::new(true);
pub static DOELSCHERM: RefCell<Monitor> = RefCell::new(Monitor::all().unwrap()[0].clone());  // eerstgenoemde monitor
pub static ENKELSCHERM: Cell<bool>= Cell::new(false);
// zie ook doelscherminfo en anderscherminfo in gereedschap.rs
pub static HANDLEIDING: Cell<bool> = Cell::new(true);
pub static HANDLEIDING_GEBRUIK: Cell<HandleidingGebruik> = Cell::new(HandleidingGebruik::Protocol);
pub static HEEL_SCHERM: Cell<bool> = Cell::new(false);
pub static KLEUR: Cell<Color32> = Cell::new(Color32::RED);
pub static LAATSTEBEELD: RefCell<String> = RefCell::new("".to_string());
pub static LAATSTE_FILE_PATH: RefCell<String> = RefCell::new("".to_string());
pub static LAATSTE_PIJL_PATH: RefCell<String> = RefCell::new("".to_string());
pub static MAAK_UITSNEDE: Cell<bool> = Cell::new(true);
pub static MENU_POS: Cell<u8> = Cell::new(0);
pub static MENU_TYPE: Cell<MenuType> = Cell::new(MenuType::Popup);
pub static MONITOR_WISSEL: Cell<bool> = Cell::new(false);
pub static MONITOR_IMAGE: RefCell<Option<ColorImage>> = RefCell::new(None);
pub static OPSLAAN: Cell<bool> = Cell::new(false);
pub static OPSLAAN_VRAAG: Cell<bool> = Cell::new(false);
pub static OPTIE_VIEWPORT: RefCell<ViewportId>= RefCell::new(ViewportId::from_hash_of("optie_viewport"));
pub static RECHTHOEK_OPSLAAN: Cell<Rect>= Cell::new(Rect{max: Pos2{x: 0.0, y: 0.0}, min: Pos2{x: 100.0, y: 100.0}});
pub static OPTIES_ON: Cell<bool> = Cell::new(false);
pub static OPSLAAN_VENSTER: Cell<ViewportId> = Cell::new(ViewportId::from_hash_of("opslaan_viewport"));
// OPSLAAN_VENSTER wordt niet meer actief gebruikt, waarschijnlijk overbodig; blijft om event te kunnen terugzetten; 5nov2025
pub static PIJLENKOKER: RefCell<Vec<Pijltje>>= RefCell::new(vec![]);
pub static PIJL_EDIT: Cell<bool> = Cell::new(false);
pub static PIJL_NUMMER: Cell<Option<usize>> = Cell::new(None);
pub static PIJL_TEKST_GROOT: Cell<usize> = Cell::new(16);
pub static PIJL_EDIT_VENSTER: Cell<ViewportId> = Cell::new(ViewportId::from_hash_of("pijl_edit_viewport"));
pub static PIJL_NAAM: RefCell<Option<String>> = RefCell::new(None);
pub static POSITIE_HORZ: Cell<f32>= Cell::new(0.0);
pub static POSITIE_VERT: Cell<f32>= Cell::new(0.0);
pub static RANDKLEUR: Cell<Color32> = Cell::new(Color32::GRAY);
pub static RANDONBEWERKT: Cell<usize> = Cell::new(1);      // lichtgrijs randje rond uitsnede; niet meer in gebruik; wel i nengelse versie
pub static RANDNABEWERKEN: Cell<usize> = Cell::new(0);     // lihtgrijs randje om bewerkt beeld; niet meer in gebruik; wel in engelse versie
pub static RETOUR_NAAR_UITSNEDE: Cell<bool> = Cell::new(false);
pub static SCHOONMAKEN: Cell<bool> = Cell::new(false);
pub static TEXT_INPUT: Cell<bool> = Cell::new(false);
pub static TEKSTEN: RefCell<Vec<TekstEnType>> = RefCell::new(Vec::new());
pub static UITBREIDEN: Cell<bool> = Cell::new(true);
pub static UITLEG: RefCell<String> = RefCell::new(" Kies hoek linksboven\n Afsluiten= muis-Rechts; Opties= F4".to_string());
pub static UITLEG_TWEE: RefCell<String> = RefCell::new(" Links boven gekozen; Kies nu hoek rechtsonder".to_string());
pub static VERKLEIN: Cell<bool> = Cell::new(false);
pub static WAYLAND: Cell<bool> = Cell::new(false);
pub static GESTART: Cell<Gestart> = Cell::new(Gestart::Eerste);
}

pub const MAX_AANTAL_GELIJKE_FILENAMEN: i32 = 30;
// beperking max aantal files met dezelfde naam, om de schermen directory niet vol te laten lopen

pub const HANDLEIDING_UITSNEDE: [&str; 7] = [
    "Handleiding: Uitsnede-functie\n",
    "Linksboven aanwijzen met muis,",
    "vasthouden, naar Rechtsonder, loslaten\n",
    "Rechter muisknop= Programma sluiten.\n",
    "Sluit zonodig deze Handleiding en het Optie-venster",
    "met de knop in het venster 'Opslaan of Stoppen'\n",
    "Menu-aan/uit met <Esc>",
];
pub const HANDLEIDING_PIJLEN: [&str; 7] = [
    "Handleiding: Pijlen-functie\n",
    "Breng pijlen aan met muis,",
    "muis-neer = punt, muis-op = staart\n",
    "klik op pijl om kleur of tekst te veranderen",
    "klik op punt of staart om te verplaatsen\n",
    "verplaats zonodig het beeld met Positie-schuivers in Optiescherm\n",
    "verwijder deze handleiding met een muis-klik",
];
