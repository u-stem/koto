//! Koto Undo - Undo/redo system

use std::collections::VecDeque;

/// A command that can be undone and redone
pub trait UndoCommand: Send {
    /// Execute the command
    fn execute(&mut self);
    /// Undo the command
    fn undo(&mut self);
    /// Get a description of the command
    fn description(&self) -> &str;
}

/// Undo/redo history
pub struct UndoHistory {
    /// Commands that can be undone
    undo_stack: VecDeque<Box<dyn UndoCommand>>,
    /// Commands that can be redone
    redo_stack: VecDeque<Box<dyn UndoCommand>>,
    /// Maximum history size
    max_size: usize,
}

impl UndoHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_size,
        }
    }

    /// Execute a command and add it to the history
    pub fn execute(&mut self, mut command: Box<dyn UndoCommand>) {
        command.execute();
        self.undo_stack.push_back(command);
        self.redo_stack.clear();

        // Limit history size
        while self.undo_stack.len() > self.max_size {
            self.undo_stack.pop_front();
        }
    }

    /// Undo the last command
    pub fn undo(&mut self) -> Option<&str> {
        if let Some(mut command) = self.undo_stack.pop_back() {
            command.undo();
            let desc = command.description().to_string();
            self.redo_stack.push_back(command);
            Some(Box::leak(desc.into_boxed_str()))
        } else {
            None
        }
    }

    /// Redo the last undone command
    pub fn redo(&mut self) -> Option<&str> {
        if let Some(mut command) = self.redo_stack.pop_back() {
            command.execute();
            let desc = command.description().to_string();
            self.undo_stack.push_back(command);
            Some(Box::leak(desc.into_boxed_str()))
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the description of the next undo action
    pub fn undo_description(&self) -> Option<&str> {
        self.undo_stack.back().map(|c| c.description())
    }

    /// Get the description of the next redo action
    pub fn redo_description(&self) -> Option<&str> {
        self.redo_stack.back().map(|c| c.description())
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

impl Default for UndoHistory {
    fn default() -> Self {
        Self::new(100)
    }
}
