use eframe::egui;
use ember_core::Buffer;

pub struct EmberApp {
    buffer: Buffer,
    /// Posición del cursor (en caracteres)
    cursor: usize,
}

impl EmberApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Texto de ejemplo para que se vea algo al abrir
        let initial = r#"// Bienvenido a EmberIDE
fn main() {
    println!("Hola desde EmberIDE");
}
"#;

        Self {
            buffer: Buffer::from_str(initial),
            cursor: 0,
        }
    }
}

impl eframe::App for EmberApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("EmberIDE — v0.1");
            ui.separator();

            // Área de edición simple (por ahora solo lectura + inserción básica)
            let mut text = self.buffer.to_string();

            let response = ui.add(
                egui::TextEdit::multiline(&mut text)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(30)
                    .font(egui::TextStyle::Monospace),
            );

            // Si el usuario escribió algo, actualizamos el buffer
            if response.changed() {
                // Por ahora reemplazamos todo el contenido (más adelante haremos edición precisa)
                self.buffer = Buffer::from_str(&text);
            }

            ui.separator();
            ui.label(format!(
                "Caracteres: {} | Líneas: {} | Cursor: {}",
                self.buffer.len_chars(),
                self.buffer.len_lines(),
                self.cursor
            ));
        });
    }
}