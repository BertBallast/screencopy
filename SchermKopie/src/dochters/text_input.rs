use std::collections::HashMap;

use crate::dochters::{
    globaal::{BESTANDSNAAM, TEKSTEN, TEXT_INPUT},
    optie_venster::schrijf_ini_file,
};
use eframe::egui::{CentralPanel, Context, Pos2, TopBottomPanel, ViewportBuilder, ViewportId};

#[derive(Clone, Debug)]
pub struct TekstEnType {
    teksttype: Teksttype,
    tekst: String,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Teksttype {
    Bestandsnaam,
    // Bijschrift,
}

pub fn text_inp(ctx: Context) {
    if TEXT_INPUT.get() == false {
        return;
    }
    ctx.request_repaint();
    let mut tekstkeys: HashMap<Teksttype, String> = HashMap::new();
    tekstkeys.insert(Teksttype::Bestandsnaam, "Bestandsnaam".to_string());
    // voor elk Teksttype is er nu ruimte voor een tekst-string in teklstkeys

    // Om teksten te kunnen invoeren via het keyboard mag de focus niet op uitsnede_app of pijlen_app gefxeerd zijn
    // daarom wordt in de update-functie van die beide functies de focus NIET opgeeist
    // anders is dat wel nodig om in die functies gebruik te kunnem maken van keyboard shortcuts en functietoetsen
    ctx.show_viewport_deferred(
        //deze functie heeft 3 argumenten nodig: (Viewport_id, ViewportBuilder, vieuwport_ui_cb)
        ViewportId::from_hash_of("text alternatief"),
        ViewportBuilder::default()
            .with_position(Pos2 { x: 0.0, y: 50.0 })
            .with_title("Text-input")
            .with_visible(true)
            .with_always_on_top()
            .with_decorations(true)
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_close_button(true)
            .with_inner_size([400.0, 200.0]),
        move |ctx, _class| {
            TopBottomPanel::bottom("mybotton").show(ctx, |ui| {
                ui.label("Tijdens deze actie werken functietoetsen en shortcuts niet");
                // want als TEXT_INPUT==true
            });
            CentralPanel::default().show(ctx, |ui| {
                let mut teksten = TEKSTEN.with(|v| v.clone()).into_inner();
                let mut naam_stringen: Vec<String> = Vec::new();
                for _i in 0..teksten.len() {
                    naam_stringen.push("--".to_string());
                }
                for i in 0..teksten.len() {
                    let naam_en_type = teksten[i].clone(); //= Tekst{teksttype: Teksttype::Bestandsnaam, tekst: "--".to_string()};
                                                           //let mut mutpijl: [&str;10]= [""; 10];
                    naam_stringen[i] = naam_en_type.tekst;
                    ui.horizontal(|ui| {
                        ui.label(tekstkeys.get(&naam_en_type.teksttype).unwrap());
                        ui.text_edit_singleline(&mut naam_stringen[i]);
                    });
                    teksten[i].tekst = naam_stringen[i].clone();
                    TEKSTEN.set(teksten.clone());
                }
                //if type_bijschrift==false {
                if ui.button("Opslaan").clicked() {
                    for bewerkt in teksten {
                        //println!("OP {:?}", naam_en_type);
                        match bewerkt.teksttype {
                            Teksttype::Bestandsnaam => {
                                BESTANDSNAAM.set(bewerkt.tekst);
                            }
                        }
                    }
                    TEXT_INPUT.set(false);
                    TEKSTEN.set(vec![]);
                    schrijf_ini_file();
                }
                //}
                if ctx.input(|i| i.viewport().close_requested()) {
                    TEXT_INPUT.set(false);
                }
            });
        },
    );
    if TEXT_INPUT.get() == false {
        TEKSTEN.set(vec![]);
    }
}

pub fn text_opdracht(soort: Teksttype, tekst: String) {
    let tekst = TekstEnType {
        teksttype: soort,
        tekst: tekst,
    };
    let mut teksten = TEKSTEN.with(|v| v.clone()).into_inner();
    // Van elk teksttypoe kan er maar een behandeld worden, de laatste, de anderen worden getoond maar bewerking gebeurt niet
    // je moet dus een teksttype dat al bestaat overschrijven ipv toevoegen
    //let mut teksten = Vec::new();
    teksten.push(tekst);
    TEKSTEN.set(teksten);
    TEXT_INPUT.set(true);
}
