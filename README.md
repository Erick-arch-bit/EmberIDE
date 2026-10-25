# EmberIDE

**Editor de código modular, nativo y rápido escrito en Rust.**

EmberIDE prioriza rendimiento real, configuración limpia y una experiencia visual propia.  
No es un clon de VS Code. Es una herramienta ligera construida desde cero con Rust + egui.

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0-blue" alt="version">
  <img src="https://img.shields.io/badge/rust-1.80%2B-orange" alt="rust">
  <img src="https://img.shields.io/badge/license-MIT-green" alt="license">
</p>

---

## ✨ Características v1.0

| Característica | Descripción |
|---------------|-------------|
| **Editor nativo** | Renderizado rápido con Rust + egui + ropey |
| **Systema de pestañas** | Múltiples archivos con estado *dirty* |
| **Syntax highlighting** | Rust, TOML, Markdown, JSON, Python |
| **Diálogos nativos** | Abrir/Guardar con `rfd` (multiplataforma) |
| **Configuración** | `ember.toml` con hot-reload (proyecto > usuario) |
| **Atajos de teclado** | Ctrl+S (guardar), Ctrl+Z (deshacer), Ctrl+Y (rehacer), Ctrl+O (abrir) |
| **Tema oscuro** | Identidad propia con Visuals personalizados |
| **Arquitectura modular** | 7 crates en Cargo workspace |

---

## ⚙️ Requisitos

- [Rust 1.80+](https://www.rust-lang.org/)
- Sistema operativo: Linux, macOS o Windows

---

## 🚀 Instalación y ejecución

```bash
# Clonar el repositorio
git clone https://github.com/tu-usuario/ember-ide.git
cd ember-ide

# Ejecutar en modo debug
cargo run -p ember-app

# Ejecutar en modo release (recomendado)
cargo run -p ember-app --release
```

---

## ⌨️ Uso básico - Atajos de teclado

| Acción | Atajo |
|--------|-------|
| Abrir archivo | `Ctrl + O` |
| Guardar | `Ctrl + S` |
| Guardar como | `Ctrl + Shift + S` |
| Deshacer | `Ctrl + Z` |
| Rehacer | `Ctrl + Y` |
| Cerrar pestaña | `Ctrl + W` |
| Nueva pestaña | `Ctrl + N` |

---

## 📁 Configuración (`ember.toml`)

EmberIDE busca el archivo de configuración en este orden:

1. `ember.toml` en la raíz del proyecto actual
2. `~/.config/ember/ember.toml` (configuración de usuario)

### Ejemplo de `ember.toml`

```toml
[editor]
font_size = 14.5
tab_size = 4
insert_spaces = true
line_numbers = true
word_wrap = false

[theme]
name = "ember-dark"
```

Los cambios en el archivo de configuración se aplican automáticamente (hot-reload).

---

## 🏗️ Arquitectura

```
textember-ide/
├── crates/
│   ├── ember-core      # Buffer, selección, cursor, undo/redo
│   ├── ember-syntax    # Detección de lenguaje + highlighting
│   ├── ember-config    # Sistema de configuración
│   ├── ember-ui        # Interfaz (egui)
│   ├── ember-lsp       # Preparado para futuro LSP
│   ├── ember-plugins   # Preparado para plugins
│   └── ember-app       # Binario principal
└── target/             # Compilados
```

---

## 🛣️ Roadmap v1.1 (próximo)

- Highlighting completo con tree-sitter (tokens reales)
- Mejor multi-cursor
- Búsqueda y reemplazo mejorados
- Más lenguajes

### Futuro

- Cliente LSP
- Terminal integrada
- Integración básica con Git
- Sistema de plugins nativos

---

## 🤝 Contribuir

Las contribuciones son bienvenidas.

1. Haz fork del repositorio
2. Crea una rama: `git checkout -b feature/nueva-funcion`
3. Commit de tus cambios: `git commit -m 'Añadir nueva funcionalidad'`
4. Push a la rama: `git push origin feature/nueva-funcion`
5. Abre un Pull Request

---

## 📄 Licencia

MIT License. Ver el archivo [LICENSE](LICENSE) para más detalles.

---

## 📦 Estructura del repositorio

```text
.
├── crates/              # 7 crates del workspace
│   ├── ember-core/
│   ├── ember-app/
│   ├── ember-syntax/
│   ├── ember-config/
│   ├── ember-ui/
│   ├── ember-lsp/
│   └── ember-plugins/
├── .gitignore
├── LICENSE
├── README.md            # Este archivo
├── Cargo.toml           # Configuración del workspace
└── target/              # Compilados
```

---

## 🙏 Reconocimientos

- [eframe](https://github.com/emrsbyr/egui) - GUI framework
- [egui](https://github.com/emersya/egui) - Immediate mode GUI
- [rfd](https://github.com/arafatih/rfd) - Diálogos nativos de archivo

---

## ¿Quieres ayudar?

Puedes:
- Reportar bugs
- Sugerir nuevas funcionalidades
- Mejorar la documentación
- Traducir el README a otros idiomas
- Mejorar los tests y cobertura

---

<p align="center">
  Hecho con ❤️ en Rust | EmberIDE v1.0.0
</p>
</details>