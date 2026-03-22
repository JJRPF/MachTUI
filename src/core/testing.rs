//! Testing utilities for MachTUI.
//! Provides snapshot-based terminal output verification.

use crate::core::Canvas;
use std::fs;
use std::path::PathBuf;

pub struct SnapshotTester {
    snapshot_dir: PathBuf,
}

impl SnapshotTester {
    pub fn new(dir: &str) -> Self {
        let path = PathBuf::from(dir);
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }
        Self { snapshot_dir: path }
    }

    /// Captures the current canvas state and compares it against a saved snapshot.
    pub fn verify_snapshot(&self, name: &str, canvas: &Canvas) -> bool {
        let snapshot_path = self.snapshot_dir.join(format!("{}.snap", name));
        let current_state = self.serialize_canvas(canvas);

        if !snapshot_path.exists() {
            fs::write(snapshot_path, current_state).unwrap();
            println!("Snapshot '{}' created.", name);
            return true;
        }

        let saved_state = fs::read_to_string(snapshot_path).unwrap();
        if current_state == saved_state {
            true
        } else {
            println!("Snapshot mismatch for '{}'!", name);
            false
        }
    }

    fn serialize_canvas(&self, canvas: &Canvas) -> String {
        let mut out = String::new();
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                let idx = (y as usize) * (canvas.width as usize) + (x as usize);
                out.push(canvas.cells[idx].content);
            }
            out.push('\n');
        }
        out
    }
}
