use eframe::{egui, epi};

// Struktur zur Darstellung der Projektdaten
struct Projekt {
    budget: f64,
    zeitplan: f64,
    ressourcen: f64,
    qualitaet: f64,
}

// Implementierung der Methoden für die Projektstruktur
impl Projekt {
    // Methode zur Berechnung des Risikoscores
    fn berechne_risikoscore(&self) -> f64 {
        // Hier verwenden wir eine einfache Gewichtung für die Risikofaktoren
        let gewicht_budget = 0.3;
        let gewicht_zeitplan = 0.3;
        let gewicht_ressourcen = 0.2;
        let gewicht_qualitaet = 0.2;

        (self.budget * gewicht_budget) +
        (self.zeitplan * gewicht_zeitplan) +
        (self.ressourcen * gewicht_ressourcen) +
        (self.qualitaet * gewicht_qualitaet)
    }
}

struct MyApp {
    budget: f64,
    zeitplan: f64,
    ressourcen: f64,
    qualitaet: f64,
    risikoscore: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            budget: 0.0,
            zeitplan: 0.0,
            ressourcen: 0.0,
            qualitaet: 0.0,
            risikoscore: 0.0,
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Risikoscore Berechnung"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Risikoscore Berechnung");

            ui.add(egui::Slider::new(&mut self.budget, 0.0..=10.0).text("Budget-Risiko"));
            ui.add(egui::Slider::new(&mut self.zeitplan, 0.0..=10.0).text("Zeitplan-Risiko"));
            ui.add(egui::Slider::new(&mut self.ressourcen, 0.0..=10.0).text("Ressourcen-Risiko"));
            ui.add(egui::Slider::new(&mut self.qualitaet, 0.0..=10.0).text("Qualitäts-Risiko"));

            if ui.button("Berechne Risikoscore").clicked() {
                let projekt = Projekt {
                    budget: self.budget,
                    zeitplan: self.zeitplan,
                    ressourcen: self.ressourcen,
                    qualitaet: self.qualitaet,
                };
                self.risikoscore = projekt.berechne_risikoscore();
            }

            ui.label(format!("Der berechnete Risikoscore beträgt: {:.2}", self.risikoscore));
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(MyApp::default()),
        options,
    );
}
