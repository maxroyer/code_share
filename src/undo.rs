#![allow(dead_code)]
use std::collections::VecDeque;
use std::ops::Range;
use eframe::egui::TextBuffer;

enum TxtCom {
    Insert {
        text: String,
        ch_index: usize,
    },
    Delete {
        ch_range: Range<usize>
    }
}

impl TxtCom {
    fn execute(&self, buffer: &mut String) -> usize {
        match self {
            Self::Insert{ text, ch_index } => {
                buffer.insert_text(text, *ch_index)
            },
            Self::Delete {ch_range } => {
                buffer.delete_char_range(ch_range.clone());
                0
            }
        }
        
    }
}


pub struct UndoStack {
    undo_stack: VecDeque<TxtCom>,
    redo_stack: VecDeque<TxtCom>,
}

