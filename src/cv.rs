use egui::{Context, RichText};

#[derive(Clone, Default)]
pub struct Cv {}

impl Cv {
    pub fn cv(ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.set_max_width(500.);

                ui.horizontal_wrapped(|ui| {
                    // Trick so we don't have to add spaces in the text below:
                    let width = ui.fonts(|f|f.glyph_width(&egui::TextStyle::Body.resolve(ui.style()), ' '));
                    ui.spacing_mut().item_spacing.x = width;

                    ui.label("Salut, je m'appelle");
                    ui.label(RichText::new("Thomas Campistron").strong());
                    ui.label(", ou juste");
                    ui.label(RichText::new("Tamo").strong());
                    ui.label("sur internet. J'ai");

                    ui.label("ans et je suis développeur pour");
                    ui.hyperlink_to("Meilisearch", "https://meilisearch.com");
                    ui.label("en télétravail. J'habite");
                    ui.hyperlink_to("au Vigan", "https://fr.wikipedia.org/wiki/Le_Vigan_(Gard)");

                    ui.label("et j'ai fait ce site après avoir découvert que le lycée à côté de chez moi collecte des données météorologiques depuis 2006.");
                    ui.label("Toutes les données affichées sur mon site viennent en réalité de :");
                    ui.hyperlink("http://meteo.lyc-chamson-levigan.ac-montpellier.fr/meteo/index.php?page=releve");
                    ui.label("Elles sont mises à jour tous les jours à 2h du matin.");
                });

                ui.add_space(20.);
                ui.horizontal_wrapped(|ui| {
                    ui.label("L'intégralité du code qui génère ce site web est disponible");
                    ui.hyperlink_to("ici", "https://github.com/irevoire/egui-meteo");
                    ui.label("où vous pouvez m'y faire des suggestions via les « issues ».");
                });
            });
        });
    }
}
