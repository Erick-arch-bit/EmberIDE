use eframe::egui::{self, Color32, FontId, RichText, Visuals};
use std::path::PathBuf;

// Diálogos nativos de archivo (Sección 3.2 SDD v1.0)
use rfd::FileDialog;

/// Estructura de documento con path y estado dirty
#[derive(Clone)]
struct Documento {
    nombre: String,
    texto: String,
    ruta: Option<PathBuf>,
    sucio: bool,
}

/// Aplicación principal de EmberIDE con sistema de pestañas y file I/O
pub struct EmberApp {
    documentos: Vec<Documento>,
    activo: usize,
}

impl EmberApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = Visuals::dark();
        visuals.window_fill = Color32::from_rgb(18, 18, 22);
        visuals.panel_fill = Color32::from_rgb(18, 18, 22);
        visuals.extreme_bg_color = Color32::from_rgb(12, 12, 16);
        visuals.override_text_color = Some(Color32::from_rgb(220, 220, 230));
        cc.egui_ctx.set_visuals(visuals);

        let inicial = Documento {
            nombre: "untitled.ember".to_string(),
            texto: "# Bienvenido a EmberIDE\n// Editor modular escrito en Rust\n\nfn main() {\n    println!(\"Hola desde EmberIDE\");\n}".to_string(),
            ruta: None,
            sucio: false,
        };

        Self {
            documentos: vec![inicial],
            activo: 0,
        }
    }
}

impl eframe::App for EmberApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let ctx_input = ctx.input(|_| egui::InputState::default());

        // === 1. Manejo de atajos de teclado ===
        // Ctrl + S: Guardar documento activo
        if ctx_input.key_pressed(egui::Key::S) && ctx_input.modifiers.ctrl {
            self.guardar_documento_activo();
        }

        // Ctrl + S + Shift: Guardar como
        if ctx_input.key_pressed(egui::Key::S) && ctx_input.modifiers.ctrl && ctx_input.modifiers.shift {
            self.guardar_documento_como();
        }

        // Ctrl + Z: Marcar documento como modificado
        if ctx_input.key_pressed(egui::Key::Z) && ctx_input.modifiers.ctrl {
            self.documentos[self.activo].sucio = true;
        }

        // Ctrl + Y: Rehacer (placeholder)
        if ctx_input.key_pressed(egui::Key::Y) && ctx_input.modifiers.ctrl {
            // Placeholder
        }

        // Ctrl + O: Abrir archivo
        if ctx_input.key_pressed(egui::Key::O) && ctx_input.modifiers.ctrl {
            self.abrir_archivo();
        }

        // === 2. Barra superior con tabs ===
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Mostrar cada documento como botón (simulando tabs)
                for (i, doc) in self.documentos.iter().enumerate() {
                    let _is_active = self.activo == i;
                    let nombre_display = if doc.sucio {
                        format!("{}###doc{}", doc.nombre, i)
                    } else {
                        format!("{}###doc{}", doc.nombre, i)
                    };
                    if ui.button(nombre_display.as_str()).clicked {
                        self.activo = i;
                    }
                }

                // Botón Nuevo documento
                if ui.button("+ Nuevo").clicked {
                    let nuevo = Documento {
                        nombre: format!("untitled_{}.ember", self.documentos.len()),
                        texto: "# Nuevo documento".to_string(),
                        ruta: None,
                        sucio: false,
                    };
                    self.documentos.push(nuevo);
                    self.activo = self.documentos.len() - 1;
                }

                // Botón Abrir
                if ui.button("📁").clicked {
                    self.abrir_archivo();
                }

                // Botón Guardar
                if ui.button("💾").clicked {
                    self.guardar_documento_activo();
                }

                // Botón Guardar Como
                if ui.button("💾⇧").clicked {
                    self.guardar_documento_como();
                }

                // Botón Cerrar pestaña activa (solo si hay más de 1)
                if self.documentos.len() > 1 {
                    if ui.button("✕").clicked {
                        self.cerrar_pestaña_activa(ctx);
                    }
                }
            });
        });

        // === 3. Editor central ===
        egui::CentralPanel::default().show(ctx, |ui| {
            let idx = self.activo;
            if self.documentos.len() > 0 && idx < self.documentos.len() {
                let doc = &self.documentos[idx];
                // TextEdit - removido mut problematic, usando clone pattern
                let mut texto = doc.texto.clone();
                let text_edit = egui::TextEdit::multiline(&mut texto)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(20)
                    .font(FontId::monospace(14.0))
                    .frame(false);

                let response = ui.add(text_edit);

                // Actualizar documento solo si changed y no es el primer render
                if response.changed {
                    // Actualizar el texto en el documento
                    self.documentos[idx].texto = texto;
                    self.documentos[idx].sucio = true;
                }
            }
        });

        // === 4. Status bar ===
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.documentos.len() > 0 && self.activo < self.documentos.len() {
                    let doc = &self.documentos[self.activo];
                    // Contar líneas y columna simples
                    let mut linea = 1;
                    let mut col = 0;
                    for ch in doc.texto.chars() {
                        if ch == '\n' {
                            linea += 1;
                            col = 0;
                        } else {
                            col += 1;
                            if col > 100 { break; } // límite para performance
                        }
                    }
                    ui.label(format!("Ln {}  |  Col {}", linea, col + 1));
                    if doc.sucio {
                        ui.label(RichText::new("*").color(Color32::RED).small());
                    } else {
                        ui.label(RichText::new("").small());
                    }
                    ui.label(RichText::new(&doc.nombre).small());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        ui.label("EmberIDE");
                    });
                } else {
                    ui.label("No documents");
                }
            });
        });
    }
}

impl EmberApp {
    /// Abrir archivo con diálogo nativo (Sección 3.2 SDD)
    fn abrir_archivo(&mut self) {
        let dialog = FileDialog::new()
            .set_title("Abrir Archivo")
            .add_filter("Source files", &["*.rs", "*.toml", "*.md", "*.json", "*.ember"])
            .add_filter("All files", &["*.*"]);

        if let Some(path) = dialog.pick_file() {
            if let Ok(contenido) = std::fs::read_to_string(&path) {
                // Verificar si ya está abierto (mismo path)
                let ya_abierto = self.documentos.iter().any(|d| d.ruta.as_ref().map_or(false, |r| r == &path));

                if ya_abierto {
                    // Cambiar a ese documento
                    for (i, doc) in self.documentos.iter().enumerate() {
                        if doc.ruta.as_ref().map_or(false, |r| r == &path) {
                            self.activo = i;
                            return;
                        }
                    }
                } else {
                    // Nuevo documento desde archivo
                    let nombre = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let doc = Documento {
                        nombre,
                        texto: contenido,
                        ruta: Some(path),
                        sucio: false,
                    };
                    self.documentos.push(doc);
                    self.activo = self.documentos.len() - 1;
                }
            }
        }
    }

    /// Guardar documento activo (Sección 3.2 SDD)
    fn guardar_documento_activo(&mut self) {
        let doc = &mut self.documentos[self.activo];

        if let Some(ref path) = doc.ruta {
            // Ya tiene ruta → guardar directamente
            if let Ok(_) = std::fs::write(path, &doc.texto) {
                doc.sucio = false;
                // Asegurar que el nombre no termine con caracteres extraños
                doc.nombre = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            }
        } else {
            // No tiene ruta → guardar como
            self.guardar_documento_como();
        }
    }

    /// Guardar documento como (Sección 3.2 SDD)
    fn guardar_documento_como(&mut self) {
        let doc = &mut self.documentos[self.activo];

        if let Some(path) = rfd::FileDialog::new()
            .set_title("Guardar Como")
            .save_file()
        {
            // Escribir contenido
            if let Ok(_) = std::fs::write(&path, &doc.texto) {
                doc.ruta = Some(path.clone());
                doc.nombre = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                doc.sucio = false;
            }
        }
    }

    /// Cerrar pestaña activa con confirmación si hay cambios (Sección 3.3 SDD)
    fn cerrar_pestaña_activa(&mut self, ctx: &egui::Context) {
        let doc = &self.documentos[self.activo];
        let nombre = doc.nombre.clone(); // Clone para evitar préstamo en closure

        if doc.sucio {
            // Mostrar ventana de confirmación
            egui::Window::new("¿Guardar cambios?")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(format!("¿Desea guardar los cambios en {}", nombre));
                    ui.horizontal(|ui| {
                        if ui.button("Sí, guardar").clicked {
                            self.guardar_documento_activo();
                            self.limitar_pestaña();
                        }
                        if ui.button("No, descartar").clicked {
                            self.limitar_pestaña();
                        }
                        if ui.button("Cancelar").clicked {
                            // No hacer nada, mantener pestaña
                        }
                    });
                });
        } else {
            self.limitar_pestaña();
        }
    }

    /// Limitar (remover) la pestaña actual
    fn limitar_pestaña(&mut self) {
        self.documentos.remove(self.activo);

        // Ajustar índice activo
        if self.activo >= self.documentos.len() && self.documentos.len() > 0 {
            self.activo = self.documentos.len() - 1;
        }
        if self.documentos.is_empty() {
            // Reañadir documento inicial
            let inicial = Documento {
                nombre: "untitled.ember".to_string(),
                texto: "# Bienvenido a EmberIDE\n// Editor modular escrito en Rust\n\nfn main() {\n    println!(\"Hola desde EmberIDE\");\n}".to_string(),
                ruta: None,
                sucio: false,
            };
            self.documentos.push(inicial);
            self.activo = 0;
        }
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title("EmberIDE").with_inner_size((1000.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "EmberIDE",
        options,
        Box::new(|cc| Ok(Box::new(EmberApp::new(cc)))),
    )
    .unwrap();
}