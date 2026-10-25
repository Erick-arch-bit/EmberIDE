use serde::Deserialize;
use std::fs;
use std::path::Path;
use anyhow::{Result, anyhow};

// Color simplificado para serialización TOML
#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// Configuración completa del editor EmberIDE
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub editor: EditorConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
    #[serde(default)]
    pub keybindings: KeybindingsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: EditorConfig {
                font_size: default_font_size(),
                font_family: default_font_family(),
                tab_size: default_tab_size(),
                insert_spaces: default_insert_spaces(),
                line_numbers: default_line_numbers(),
                word_wrap: default_word_wrap(),
            },
            theme: ThemeConfig {
                name: default_theme_name(),
            },
            keybindings: KeybindingsConfig {
                save: default_save_key(),
                undo: default_undo_key(),
                redo: default_redo_key(),
                find: default_find_key(),
            },
        }
    }
}

/// Configuración del editor
#[derive(Debug, Clone, Default, Deserialize)]
pub struct EditorConfig {
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default = "default_tab_size")]
    pub tab_size: usize,
    #[serde(default = "default_insert_spaces")]
    pub insert_spaces: bool,
    #[serde(default = "default_line_numbers")]
    pub line_numbers: bool,
    #[serde(default = "default_word_wrap")]
    pub word_wrap: bool,
}

/// Configuración del tema
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_theme_name")]
    pub name: String,
}

/// Configuración de atajos de teclado
#[derive(Debug, Clone, Default, Deserialize)]
pub struct KeybindingsConfig {
    #[serde(default = "default_save_key")]
    pub save: String,
    #[serde(default = "default_undo_key")]
    pub undo: String,
    #[serde(default = "default_redo_key")]
    pub redo: String,
    #[serde(default = "default_find_key")]
    pub find: String,
}

/// Rutas de configuración
const EMBER_CONFIG_PATH: &str = "ember.toml";
const USER_CONFIG_PATH: &str = ".ember.toml";

// Valores por defecte
fn default_font_size() -> f32 { 14.0 }
fn default_font_family() -> String { "JetBrains Mono".to_string() }
fn default_tab_size() -> usize { 4 }
fn default_insert_spaces() -> bool { true }
fn default_line_numbers() -> bool { true }
fn default_word_wrap() -> bool { false }
fn default_theme_name() -> String { "ember-dark".to_string() }
fn default_save_key() -> String { "Ctrl+S".to_string() }
fn default_undo_key() -> String { "Ctrl+Z".to_string() }
fn default_redo_key() -> String { "Ctrl+Y".to_string() }
fn default_find_key() -> String { "Ctrl+F".to_string() }

/// Cargar configuración (proyecto > usuario)
pub fn load_config() -> Result<Config> {
    let mut config = match load_project_config() {
        Ok(c) => c,
        Err(_) => Config::default(),
    };

    if let Ok(user_config) = load_user_config() {
        if user_config.editor.font_size > 0.0 {
            config.editor.font_size = user_config.editor.font_size;
        }
        if !user_config.theme.name.is_empty() {
            config.theme.name = user_config.theme.name;
        }
        if !user_config.keybindings.save.is_empty() {
            config.keybindings.save = user_config.keybindings.save;
        }
        if !user_config.keybindings.undo.is_empty() {
            config.keybindings.undo = user_config.keybindings.undo;
        }
        if !user_config.keybindings.redo.is_empty() {
            config.keybindings.redo = user_config.keybindings.redo;
        }
        if !user_config.keybindings.find.is_empty() {
            config.keybindings.find = user_config.keybindings.find;
        }
    }

    Ok(config)
}

/// Cargar configuración del proyecto (ember.toml en raíz)
fn load_project_config() -> Result<Config> {
    let path = Path::new(EMBER_CONFIG_PATH);
    if path.exists() {
        let content = fs::read_to_string(path)
            .map_err(|e| anyhow!("Error leyendo {}: {}", EMBER_CONFIG_PATH, e))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| anyhow!("Error parseando {}: {}", EMBER_CONFIG_PATH, e))?;
        Ok(config)
    } else {
        Err(anyhow!("No se encontró {} en la raíz del proyecto", EMBER_CONFIG_PATH))
    }
}

/// Cargar configuración de usuario (.ember.toml)
fn load_user_config() -> Result<Config> {
    let path = Path::new(USER_CONFIG_PATH);
    if path.exists() {
        let content = fs::read_to_string(path)
            .map_err(|e| anyhow!("Error leyendo {}: {}", USER_CONFIG_PATH, e))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| anyhow!("Error parseando {}: {}", USER_CONFIG_PATH, e))?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

/// Guardar configuración en ember.toml
pub fn save_config(config: &Config) -> Result<()> {
    let mut toml_string = String::new();

    toml_string.push_str("[editor]\n");
    toml_string.push_str(&format!("font_size = {:.1}\n", config.editor.font_size));
    toml_string.push_str(&format!("font_family = \"{}\"\n", config.editor.font_family));
    toml_string.push_str(&format!("tab_size = {}\n", config.editor.tab_size));
    toml_string.push_str(&format!("insert_spaces = {}\n", config.editor.insert_spaces));
    toml_string.push_str(&format!("line_numbers = {}\n", config.editor.line_numbers));
    toml_string.push_str(&format!("word_wrap = {}\n", config.editor.word_wrap));

    toml_string.push_str("\n[theme]\n");
    toml_string.push_str(&format!("name = \"{}\"\n", config.theme.name));

    toml_string.push_str("\n[keybindings]\n");
    toml_string.push_str(&format!("save = \"{}\"\n", config.keybindings.save));
    toml_string.push_str(&format!("undo = \"{}\"\n", config.keybindings.undo));
    toml_string.push_str(&format!("redo = \"{}\"\n", config.keybindings.redo));
    toml_string.push_str(&format!("find = \"{}\"\n", config.keybindings.find));

    fs::write(EMBER_CONFIG_PATH, &toml_string)
        .map_err(|e| anyhow!("Error guardando {}: {}", EMBER_CONFIG_PATH, e))?;
    Ok(())
}

/// Aplicar configuración de tema (llamar desde ember-app)
pub fn apply_theme_name(theme_name: &str) -> &str {
    if theme_name == "ember-dark" {
        "dark"
    } else if theme_name == "light" {
        "light"
    } else {
        "default"
    }
}

/// Obtener ruta de configuración de usuario
pub fn user_config_path() -> std::path::PathBuf {
    std::path::PathBuf::from(USER_CONFIG_PATH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.editor.font_size, 14.0);
        assert_eq!(config.editor.font_family, "JetBrains Mono");
        assert_eq!(config.editor.tab_size, 4);
        assert_eq!(config.editor.insert_spaces, true);
        assert_eq!(config.editor.line_numbers, true);
        assert_eq!(config.editor.word_wrap, false);
        assert_eq!(config.theme.name, "ember-dark");
    }

    #[test]
    fn test_load_save_config_toml() {
        let config = Config {
            editor: EditorConfig {
                font_size: 16.0,
                font_family: "Custom Font".to_string(),
                tab_size: 4,
                insert_spaces: true,
                line_numbers: true,
                word_wrap: false,
            },
            theme: ThemeConfig {
                name: "custom-theme".to_string(),
            },
            keybindings: KeybindingsConfig {
                save: "Ctrl+S".to_string(),
                undo: "Ctrl+Z".to_string(),
                redo: "Ctrl+Y".to_string(),
                find: "Ctrl+F".to_string(),
            },
        };

        let result = save_config(&config);
        assert!(result.is_ok());

        let path = std::path::Path::new(EMBER_CONFIG_PATH);
        assert!(path.exists());

        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("font_size = 16.0"));
        assert!(content.contains("name = \"custom-theme\""));
        assert!(content.contains("save = \"Ctrl+S\""));

        let _ = fs::remove_file(path);
    }
}