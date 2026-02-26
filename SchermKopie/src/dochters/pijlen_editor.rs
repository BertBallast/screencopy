use crate::dochters::{
    gereedschap::{EigenTraitTbvUi, PijlWijziging, hoofdscherm, MonitorFunctions}, globaal::{
        KLEUR, OPSLAAN_VRAAG, PIJL_EDIT, PIJL_NUMMER, PIJL_TEKST_GROOT, PIJLENKOKER
    }, pijlen_app::PijlenApp,
};
use eframe::egui::{
    Align, CentralPanel, Context, CursorIcon, Layout, Pos2, ViewportBuilder, ViewportClass, ViewportId,
};

pub fn edit_pijl(ctx: Context, zelf: &mut PijlenApp) {

    if PIJL_EDIT.get() == false || PIJL_NUMMER.get()==None || OPSLAAN_VRAAG.get() {
        return;
    }
    let positie= Pos2{
        x: hoofdscherm().xpos()/ hoofdscherm().scale(), 
        y: hoofdscherm().ypos()/ hoofdscherm().scale() + 50.0 // + 50.0 om niet op het menu te zitten
    };
    ctx.show_viewport_immediate(
        //deze functie heeft 3 argumenten nodig: (Viewport_id, ViewportBuilder, vieuwport_ui_cb)
        // Immediate viewports are shown immediately, so passing state to/from them is easy.
        // The downside is that their painting is linked with the parent viewport:
        // if either needs repainting, they are both repainted.
        ViewportId::from_hash_of("pijl_edit_viewport"),
        ViewportBuilder::default()
            .with_title("Wijzig geselecteerde pijl")
            .with_always_on_top()
            .with_close_button(true)
            //.with_position(Pos2 { x: 0.0, y: 300.0 })
            .with_position(positie )
            .with_inner_size([400.0, 350.0]),
        |ctx, class| {
            assert!(
                class == ViewportClass::Immediate,
                "This egui backend doesn't support multiple viewports"
            );
            ctx.set_cursor_icon(CursorIcon::default());
            CentralPanel::default().show(ctx, |ui| {
                let pijlnr = PIJL_NUMMER.get().unwrap();
                let mut niet_tonen: bool = false;
                // >> muteer PIJLENKOKER
                let mut pk = PIJLENKOKER.take();   // pijlenkoker kan niet worden gewijzigd, wordt 'ingenomen' en na wijziging teruggeplaatst
                if pk.len() == 0 {                               // er zijn geen pijlen (meer)
                    niet_tonen = true;
                    PIJL_EDIT.set(false);
                } else {
                    ui.horizontal(|ui| {                    
                        ui.label("Bijschrift".to_string());                     // bijschrift
                        ui.text_edit_singleline(&mut (pk[pijlnr].pijltekst));
                    });
                    let mut kleur_was = KLEUR.get();
                    ui.markering_kleur(&mut kleur_was);                               // pijlkleur
                    pk[pijlnr].kleur = kleur_was;
                    KLEUR.set(pk[pijlnr].kleur);
                    pk[pijlnr].kleur = KLEUR.get();
                    ui.horizontal(|ui| {
                        ui.label("Letter grootte: ");                            // lettergrootte
                        let mut grootte = PIJL_TEKST_GROOT.get();
                        if ui.button("<").clicked() {
                            if grootte > 4 {
                                grootte -= 1;
                            }
                        };
                        ui.label(format!(" {} ", grootte));
                        if ui.button(">").clicked() {
                            grootte += 1;
                        };
                        PIJL_TEKST_GROOT.set(grootte);
                    });
                }
                PIJLENKOKER.set(pk);    // gewijzigde pijlenset terug in koker
                // << muteer PIJLENKOKER
                ui.separator();
                if niet_tonen==false {
                    if ui.button("Verwijder pijl").clicked() {                    // pijl verwijderen
                        let mut pk = PIJLENKOKER.take();
                        pk.remove(PIJL_NUMMER.get().unwrap());
                        PIJLENKOKER.set(pk);
                        zelf.pijl_bewerker= PijlWijziging::default();
                        PIJL_EDIT.set(false); // anders panic de laatste pijl is verwijderd
                        edit_end(zelf);
                    };
                    if ui.button("Toggle bijschrift met of zonder pijl").clicked() {    // alleen bijschrift, zonder pijl
                        let mut pk = PIJLENKOKER.take();
                        let pijlpos= pk[pijlnr].rug;
                        if pijlpos== pk[pijlnr].tip {
                            pk[pijlnr].tip= Pos2{x: pijlpos.x /2.0, y: pijlpos.y / 2.0};
                        } else {
                            pk[pijlnr].tip= pijlpos;
                        }
                        PIJLENKOKER.set(pk);
                        edit_end(zelf);
                    }
                }
                ui.with_layout(Layout::bottom_up(Align::Max), |ui| {    // verlaat pijleditor
                    if ui.button("ACCEPTEER").clicked() {
                        edit_end(zelf);
                    };
                });
            });

            if ctx.input(|i| i.viewport().close_requested()) {                 // sluit editvenster zonder lopende wijzigingen
                // Tell parent viewport that we should not show next frame:
                 edit_end(zelf);
            }
        },
    );
}

fn edit_end(zelf: &mut PijlenApp) {
    zelf.pijl_bewerker= PijlWijziging::default();
    PIJL_EDIT.set(false); // anders panic de laatste pijl is verwijderd
}
