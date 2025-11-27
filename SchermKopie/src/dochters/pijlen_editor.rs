use crate::dochters::{
    gereedschap::{EigenTraitTbvUi, PijlWijziging}, globaal::{
        KLEUR, OPSLAAN_VRAAG, PIJLENKOKER, PIJL_EDIT, PIJL_NUMMER, PIJL_TEKST_GROOT
    }, pijlen_app::PijlenApp,
};
use eframe::egui::{
    Align, CentralPanel, Context, CursorIcon, Layout, Pos2, ViewportBuilder, ViewportClass, ViewportId,
};

pub fn edit_pijl(ctx: Context, zelf: &mut PijlenApp) {
    if PIJL_EDIT.get() == false || PIJL_NUMMER.get()==None || OPSLAAN_VRAAG.get() {
        return;
    }
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
            .with_position(Pos2 { x: 0.0, y: 300.0 })
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
                {// >> muteer PIJLENKOKER
                    let mut pk = PIJLENKOKER.take();
                    if pk.len() == 0 {
                        niet_tonen = true;
                        PIJL_EDIT.set(false);
                    } else {
                        ui.horizontal(|ui| {
                            ui.label("Bijschrift");
                            ui.text_edit_singleline(&mut (pk[pijlnr].pijltekst));
                        });
                        let mut kleur_was = KLEUR.get();
                        ui.markering_kleur(&mut kleur_was);
                        pk[pijlnr].kleur = kleur_was;
                        KLEUR.set(pk[pijlnr].kleur);
                        pk[pijlnr].kleur = KLEUR.get();
                        ui.horizontal(|ui| {
                            ui.label("Letter grootte: ");
                            let mut dikte = PIJL_TEKST_GROOT.get();
                            if ui.button("<").clicked() {
                                if dikte > 4 {
                                    dikte -= 1;
                                }
                            };
                            ui.label(format!(" {} ", dikte));
                            if ui.button(">").clicked() {
                                dikte += 1;
                            };
                            PIJL_TEKST_GROOT.set(dikte);
                        });
                        //                            WIJZIG_PUNT.set(PijlWijziging::default());
                        //                            ctx.request_repaint_of(PIJL_VENSTER.get());
                    }
                    PIJLENKOKER.set(pk);
                } // << muteer PIJLENKOKER
                ui.separator();
                if niet_tonen==false {
                    if ui.button("Verwijder pijl").clicked() {
                        let mut pk = PIJLENKOKER.take();
                        pk.remove(PIJL_NUMMER.get().unwrap());
                        PIJLENKOKER.set(pk);
                        //WIJZIG_PUNT.set(PijlWijziging::default());
                        //PIJL_WIJZIGING.set(Some(PijlWijziging::default()));
                        zelf.pijl_bewerker= PijlWijziging::default();
                        PIJL_EDIT.set(false); // anders panic de laatste pijl is verwijderd
                        edit_end(zelf);
                        //ctx.request_repaint_of(PIJL_VENSTER.get());
                    };
                    if ui.button("Toggle bijschrift met of zonder pijl").clicked() {
                        let mut pk = PIJLENKOKER.take();
                        let pijlpos= pk[pijlnr].rug;
                        //let beeld_option = BEELD_SELECTIE.with(|v| v.clone()).into_inner();
                        if pijlpos== pk[pijlnr].tip {
                            pk[pijlnr].tip= Pos2{x: pijlpos.x /2.0, y: pijlpos.y / 2.0};
                        } else {
                            pk[pijlnr].tip= pijlpos;
                        }
                        PIJLENKOKER.set(pk);
                        //WIJZIG_PUNT.set(PijlWijziging::default());
                        //PIJL_WIJZIGING.set(Some(PijlWijziging::default()));
                        edit_end(zelf);
                    }
                }
                ui.with_layout(Layout::bottom_up(Align::Max), |ui| {
                    if ui.button("ACCEPTEER").clicked() {
                        edit_end(zelf);
                    };
                });
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent viewport that we should not show next frame:
                //zelf.immediate_viewport.store(false, Ordering::Relaxed);
                PIJL_EDIT.set(false);
                //WIJZIG_PUNT.set(PijlWijziging::default());
                zelf.pijl_bewerker= PijlWijziging::default();
//                PIJL_WIJZIGING.set(Some(PijlWijziging::default()));
                edit_end(zelf);
                //ctx.request_repaint_of(PIJL_VENSTER.get());
            }
        },
    );
}

fn edit_end(zelf: &mut PijlenApp) {
    zelf.pijl_bewerker= PijlWijziging::default();
    PIJL_EDIT.set(false); // anders panic de laatste pijl is verwijderd
}
