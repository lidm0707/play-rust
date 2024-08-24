use eframe::egui;

#[derive(Debug, PartialEq)]
enum VatType {
    Include,
    Exclude,
}

struct VatCalculator {
    price: f64,
    vat_type: VatType,
    result: f64,
}

impl Default for VatCalculator {
    fn default() -> Self {
        Self {
            price: 0.0,
            vat_type: VatType::Exclude,
            result: 0.0,
        }
    }
}

impl VatCalculator {
    fn calculate_vat(&mut self) {
        const VAT_RATE: f64 = 0.07; // VAT rate for Thailand (7%)
        self.result = match self.vat_type {
            VatType::Include => self.price / (1.0 + VAT_RATE),
            VatType::Exclude => self.price * (1.0 + VAT_RATE),
        };
    }
}

impl eframe::App for VatCalculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("VAT Calculator");
            
            ui.horizontal(|ui| {
                ui.label("Price:");
                ui.add(egui::DragValue::new(&mut self.price).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("VAT Type:");
                ui.radio_value(&mut self.vat_type, VatType::Include, "Include VAT");
                ui.radio_value(&mut self.vat_type, VatType::Exclude, "Exclude VAT");
            });

            if ui.button("Calculate").clicked() {
                self.calculate_vat();
            }

            ui.label(format!("Result: {:.2}", self.result));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "VAT Calculator",
        options,
        Box::new(|_cc| Box::new(VatCalculator::default())),
    )
}