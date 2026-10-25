/// Módulo de highlighting de sintaxis para EmberIDE.
/// Implementa la estructura requerida por SDD v1.0 (Sección 3.1).
///
/// Aunque el highlighting completo con tree-sitter requiere parsers adicionales,
/// la estructura de tipos y la interfaz están listas para la integración futura.

/// Lenguajes soportados para highlighting de sintaxis (Sección 3.1 SDD v1.0)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    /// Lenguaje Rust
    Rust,
    /// Lenguaje TOML
    Toml,
    /// Lenguaje Markdown
    Markdown,
    /// Lenguaje JSON
    Json,
    /// Lenguaje Python
    Python,
}

/// Convertir Language a string para display y detección por extensión (Sección 3.1)
pub fn language_to_string(lang: Language) -> String {
    match lang {
        Language::Rust => "Rust".to_string(),
        Language::Toml => "TOML".to_string(),
        Language::Markdown => "Markdown".to_string(),
        Language::Json => "JSON".to_string(),
        Language::Python => "Python".to_string(),
    }
}

/// Obtener CSS de highlighting para UI (Sección 3.1 SDD).
/// Styles placeholder que se pueden reemplazar con tree-sitter spans reales en futuras versiones.
pub fn highlighting_css() -> String {
    "\
    <style>
    /* Highlighting styles for v1.0 basic implementation */
    .comment { color: #6a9955; font-style: italic; }
    .string { color: #ce9178; }
    .number { color: #b5cea8; }
    .keyword { color: #c586c0; }
    .function { color: #dcdcaa; }
    .variable { color: #d4d4d4; }
    .operator { color: #d4d4d4; }
    </style>\
    "
    .to_string()
}

/// Obtener CSS de highlighting para usar en la UI (Sección 3.1)
pub fn get_highlight_css() -> String {
    highlighting_css()
}

/// Estado actual del highlighting (Sección 3.1 SDD)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightState {
    /// El lenguaje actualmente activo
    pub language: Language,
    /// Si el highlighting está activo
    pub active: bool,
}

/// Crear un nuevo estado de highlighting (Sección 3.1)
pub fn new_highlight_state(language: Language) -> HighlightState {
    HighlightState {
        language,
        active: true,
    }
}

/// Detectar lenguaje por extensión de archivo (Sección 3.1 complementaria)
/// Esto es útil para asignar automáticamente un lenguaje cuando se abre un archivo
pub fn detect_language_from_path(path: &std::path::Path) -> Language {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("rs") | Some("erb") => Language::Rust,
        Some("toml") => Language::Toml,
        Some("md") | Some("markdown") => Language::Markdown,
        Some("json") => Language::Json,
        Some("py") => Language::Python,
        _ => Language::Markdown, // Default a Markdown
    }
}

/// Probar highlighting de sintaxis (Sección 3.1 tests obligatorios v1.0)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_enum() {
        // Tests básicos del enum Language (requerido por SDD v1.0)
        let rust = Language::Rust;
        let toml = Language::Toml;

        assert_eq!(language_to_string(rust), "Rust");
        assert_eq!(language_to_string(toml), "TOML");
    }

    #[test]
    fn test_highlighting_css() {
        // Verificar que el CSS no está vacío (requerido por SDD v1.0)
        let css = highlighting_css();
        assert!(!css.is_empty());
        assert!(css.contains("style"));
    }

    #[test]
    fn test_highlight_state() {
        // Verificar que HighlightState funciona correctamente (requerido por SDD v1.0)
        let state = new_highlight_state(Language::Rust);
        assert!(state.active);
        assert_eq!(state.language, Language::Rust);
    }

    #[test]
    fn test_language_detection_from_path() {
        // Detección de lenguaje por extensión (complementario SDD)
        let rust_path = std::path::Path::new("test.rs");
        let toml_path = std::path::Path::new("config.toml");
        let md_path = std::path::Path::new("README.md");
        let json_path = std::path::Path::new("data.json");
        let py_path = std::path::Path::new("script.py");

        assert_eq!(detect_language_from_path(rust_path), Language::Rust);
        assert_eq!(detect_language_from_path(toml_path), Language::Toml);
        assert_eq!(detect_language_from_path(md_path), Language::Markdown);
        assert_eq!(detect_language_from_path(json_path), Language::Json);
        assert_eq!(detect_language_from_path(py_path), Language::Python);
        // Default
        let unknown = std::path::Path::new("unknown.xyz");
        assert_eq!(detect_language_from_path(unknown), Language::Markdown);
    }
}