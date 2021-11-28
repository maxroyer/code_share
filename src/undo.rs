use std::ops::Range;

use eframe::egui::TextBuffer;

pub trait TextCommand {
    fn execute(&self, buffer: &mut String) -> usize;
}

pub struct TextInsert {
    text: String,
    ch_index: usize
}

impl TextCommand for TextInsert {
    fn execute(&self, buffer: &mut String) -> usize {
         buffer.insert_text(&self.text, self.ch_index)
    }
}

pub struct TextDelete {
    ch_range: Range<usize>
}

impl TextCommand for TextDelete {
    fn execute(&self, buffer: &mut String) -> usize {
        let range = self.ch_range.clone();
        buffer.delete_char_range(range);
        0
    }
}

pub struct UndoStack<T> {
    undo_stack: Vec<T>,
    redo_stack: Vec<T>,
}

impl <T: TextCommand> UndoStack<T> {

}