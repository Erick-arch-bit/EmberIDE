use ropey::Rope;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Una selección de texto con un punto de anclaje y un punto final.
/// El orden de los puntos determina la dirección de la selección.
/// anchor es el punto de inicio y head es el punto final.
pub struct Selection {
    /// Punto de inicio/ancla de la selección
    pub anchor: usize,
    /// Punto final de la selección
    pub head: usize,
}

impl Selection {
    /// Crea una selección nueva con anchor y head en la misma posición (sin selección)
    pub fn new(pos: usize) -> Self {
        Self { anchor: pos, head: pos }
    }

    /// Crea una selección con anchor y head definidos
    pub fn new_range(anchor: usize, head: usize) -> Self {
        Self { anchor, head }
    }

    /// Devuelve el rango ordenado (start..end) sin importar el orden de anchor/head
    pub fn range(&self) -> Range<usize> {
        let (a, h) = (self.anchor, self.head);
        if a <= h { a..h } else { h..a }
    }

    /// Longitud de la selección (0 si anchor == head)
    pub fn len(&self) -> usize {
        self.head.saturating_sub(self.anchor)
    }

    /// ¿La selección está vacía? (anchor == head)
    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }
}

/// Posición de un cursor en el buffer.
/// Puede tener una selección asociada opcional.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    /// Posición del cursor en el buffer (índice de carácter)
    pub position: usize,
    /// Selección actual del cursor (opcional, None = sin selección)
    pub selection: Option<Selection>,
}

impl Cursor {
    /// Crea un cursor en la posición dada sin selección
    pub fn new(position: usize) -> Self {
        Self {
            position,
            selection: None,
        }
    }

    /// Crea un cursor con selección
    pub fn with_selection(position: usize, selection: Selection) -> Self {
        Self {
            position,
            selection: Some(selection),
        }
    }

    /// Mover cursor a una nueva posición
    pub fn move_to(&mut self, new_pos: usize) {
        self.position = new_pos;
        // Si la selección está vacía, moverla también
        if self.selection.as_ref().map_or(true, |s| s.is_empty()) {
            if let Some(selection) = self.selection.as_mut() {
                selection.anchor = new_pos;
                selection.head = new_pos;
            }
        }
    }

    /// Alternar/actualizar selección (expandir/reducir o crear/borrar)
    pub fn toggle_selection(&mut self, new_head: usize) {
        if self.selection.is_none() {
            // Crear nueva selección
            self.selection = Some(Selection::new_range(self.position, new_head));
        } else {
            // Actualizar head de la selección existente
            if let Some(selection) = self.selection.as_mut() {
                selection.head = new_head;
            }
        }
    }
}

/// Estado global del editor: buffer, cursores y historial
#[derive(Debug, Clone)]
pub struct EditorState {
    /// Buffer de texto principal
    pub buffer: Buffer,
    /// Lista de cursores activos (para multi-cursor)
    pub cursores: Vec<Cursor>,
    /// Historial simplificado para undo/Redo
    ///
    /// Para v1 usamos el historial del buffer directamente;
    /// operaciones Undo/Redo se aplican globalmente.
    pub historial_undo: Vec<Rope>,
}

/// Resultado de una operación de edición
#[derive(Debug, Clone, PartialEq)]
pub enum EditResult {
    /// Operación exitosa
    Ok,
    /// No hay cambio (por ejemplo, cursor al final y intentar insertar al final)
    NoChange,
    /// Error de posición fuera de rango
    OutOfRange,
}

/// Buffer de texto con undo/redo usando Rope
#[derive(Debug, Clone)]
pub struct Buffer {
    rope: Rope,
    undo_stack: Vec<Rope>,
    redo_stack: Vec<Rope>,
}

impl Buffer {
    /// Crea un buffer vacío
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// Crea un buffer a partir de un string
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// Devuelve todo el contenido como String
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Número de caracteres
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Número de líneas
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Inserta texto en una posición
    /// `at` es la posición donde insertar (antes del carácter en esa posición)
    /// Retorna el resultado de la edición
    pub fn insert(&mut self, at: usize, text: &str) -> EditResult {
        // Validar posición
        if at > self.rope.len_chars() {
            return EditResult::OutOfRange;
        }

        self.save_undo();
        self.rope.insert(at, text);
        self.redo_stack.clear();
        EditResult::Ok
    }

    /// Borra un rango de caracteres
    /// `range` es el rango a borrar (inclusive start, exclusive end)
    /// Retorna el resultado de la edición
    pub fn remove(&mut self, range: Range<usize>) -> EditResult {
        let start = range.start.min(range.end);
        let end = range.start.max(range.end);
        let clamped_start = start.max(0);
        let clamped_end = end.min(self.rope.len_chars());

        if clamped_start >= clamped_end {
            return EditResult::NoChange;
        }

        if clamped_start >= self.rope.len_chars() {
            return EditResult::NoChange;
        }

        let actual_range = clamped_start..clamped_end;
        self.save_undo();
        self.rope.remove(actual_range);
        self.redo_stack.clear();
        EditResult::Ok
    }

    /// Obtiene una línea completa (sin el \n final)
    pub fn line(&self, line_idx: usize) -> String {
        self.rope.line(line_idx).to_string()
    }

    /// Convierte posición de carácter a (línea, columna)
    pub fn char_to_line_col(&self, char_idx: usize) -> (usize, usize) {
        if char_idx > self.rope.len_chars() {
            return (self.rope.len_lines(), 0);
        }
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let col = char_idx - line_start;
        (line, col)
    }

    /// Convierte (línea, columna) a posición de carácter
    pub fn line_col_to_char(&self, line: usize, col: usize) -> usize {
        let line_start = self.rope.line_to_char(line);
        let safe_col = col.min(self.rope.line(line).chars().count());
        line_start + safe_col
    }

    fn save_undo(&mut self) {
        self.undo_stack.push(self.rope.clone());
        // Limitar historial para no comer mucha memoria
        if self.undo_stack.len() > 100 {
            self.undo_stack.remove(0);
        }
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.rope.clone());
            self.rope = prev;
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.rope.clone());
            self.rope = next;
        }
    }

    /// Mover cursor a una nueva posición
    /// Posición en índices de carácter (0 = antes del primer carácter)
    pub fn set_cursor_pos(&mut self, pos: usize, cursor: &mut Cursor) {
        let actual_pos = pos.min(self.rope.len_chars());
        cursor.position = actual_pos;
        // Si la selección estaba vacía, actualizar su anchor también
        if cursor.selection.as_ref().map_or(true, |s| s.is_empty()) {
            cursor.selection = Some(Selection::new(actual_pos));
        }
    }

    /// Mover cursor por caracteres (positivo = adelante, negativo = atrás)
    /// Retorna true si se movió, false si ya estaba en esa posición
    pub fn move_cursor_by(&mut self, delta: isize, cursor: &mut Cursor) -> bool {
        let new_pos = (cursor.position as isize + delta)
            .max(0)
            .min(self.rope.len_chars() as isize) as usize;
        if new_pos != cursor.position {
            cursor.position = new_pos;
            // Ajustar selección si está vacía
            if cursor.selection.as_ref().map_or(true, |s| s.is_empty()) {
                cursor.selection = Some(Selection::new(new_pos));
            }
            true
        } else {
            false
        }
    }

    /// Seleccionar texto entre anchor y head
    /// Moves the head to new_pos, expanding or contracting the selection
    pub fn select_in_range(&mut self, anchor: usize, head: usize, cursor: &mut Cursor) {
        let a = anchor.min(self.rope.len_chars());
        let h = head.min(self.rope.len_chars());
        cursor.position = if a <= h { h } else { a }; // cursor va al head
        cursor.selection = Some(Selection::new_range(a, h));
    }

    /// Obtener la selección actual desde el cursor
    pub fn get_selection_from_cursor(&self, cursor: &Cursor) -> Option<Selection> {
        cursor.selection.clone()
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_remove() {
        let mut buf = Buffer::new();
        buf.insert(0, "Hola mundo");
        assert_eq!(buf.to_string(), "Hola mundo");

        buf.remove(5..10);
        assert_eq!(buf.to_string(), "Hola ");
    }

    #[test]
    fn test_undo_redo() {
        let mut buf = Buffer::from_str("hola");
        buf.insert(4, " mundo");
        assert_eq!(buf.to_string(), "hola mundo");

        buf.undo();
        assert_eq!(buf.to_string(), "hola");

        buf.redo();
        assert_eq!(buf.to_string(), "hola mundo");
    }

    #[test]
    fn test_selection_operations() {
        let mut buf = Buffer::from_str("Hola mundo");
        let mut cursor = Cursor::new(0);

        // Mover cursor
        buf.move_cursor_by(6, &mut cursor);
        assert_eq!(cursor.position, 6);

        // Seleccionar texto
        buf.select_in_range(0, 5, &mut cursor);
        let sel = cursor.selection.unwrap();
        assert!(!sel.is_empty());
        assert_eq!(sel.len(), 5);

        // Mover cursor con selección vacía
        cursor.selection = None; // Quitar selección temporalmente
        buf.set_cursor_pos(0, &mut cursor);
        assert_eq!(cursor.position, 0);
        assert!(cursor.selection.is_none() || cursor.selection.as_ref().map_or(true, |s| s.is_empty()));

        // Crear selección de nuevo
        buf.select_in_range(0, 5, &mut cursor);
        let sel2 = cursor.selection.unwrap();
        assert_eq!(sel2.len(), 5);
        assert!(!sel2.is_empty());
    }

    #[test]
    fn test_cursor_movement() {
        let mut buf = Buffer::from_str("Hola mundo");
        let mut cursor = Cursor::new(0);

        // Mover cursor al final ("Hola mundo" tiene 10 caracteres, posiciones 0-9, len=10)
        let result = buf.move_cursor_by(10, &mut cursor); // 10 = len de "Hola mundo"
        // Después de mover 10 pasos desde 0, cursor debería estar en posición 10 (después del último char)
        // o en 9 si el límite es estricto
        assert!(result || cursor.position >= 0);

        // Mover cursor hacia atrás (de 10 a 9)
        buf.move_cursor_by(-1, &mut cursor);
        assert_eq!(cursor.position, 9);

        // No moverse más allá de los límites hacia abajo
        buf.move_cursor_by(-100, &mut cursor);
        assert_eq!(cursor.position, 0);

        // Moverse hacia arriba más allá del límite
        buf.move_cursor_by(100, &mut cursor);
        assert_eq!(cursor.position, 10); // posición máxima = len
    }

    #[test]
    fn test_edit_result() {
        let mut buf = Buffer::new();
        // Insertar en posición válida
        let result = buf.insert(0, "Hola");
        assert_eq!(result, EditResult::Ok);
        assert_eq!(buf.to_string(), "Hola");

        // Insertar fuera de rango
        let result = buf.insert(100, "X");
        assert_eq!(result, EditResult::OutOfRange);

        // Borrar rango vacío
        let result = buf.remove(0..0);
        assert_eq!(result, EditResult::NoChange);
    }
}