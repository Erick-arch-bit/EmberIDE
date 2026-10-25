fn main() {
    let config = ember_config::Config {
        editor: ember_config::EditorConfig {
            font_size: 16.0,
            font_family: "Custom Font".to_string(),
            tab_size: 4,
            insert_spaces: true,
            line_numbers: true,
            word_wrap: false,
        },
        theme: ember_config::ThemeConfig {
            name: "custom-theme".to_string(),
        },
        keybindings: ember_config::KeybindingsConfig {
            save: "Ctrl+S".to_string(),
            undo: "Ctrl+Z".to_string(),
            redo: "Ctrl+Y".to_string(),
            find: "Ctrl+F".to_string(),
        },
    };
    
    match ember_config::save_config(&config) {
        Ok(()) => {
            println!("Save succeeded!");
            if std::path::Path::new("ember.toml").exists() {
                let content = std::fs::read_to_string("ember.toml").unwrap();
                println!("Content:\n{}", content);
            } else {
                println!("File not found");
            }
        }
        Err(e) => println!("Save failed: {:?}", e),
    }
}
