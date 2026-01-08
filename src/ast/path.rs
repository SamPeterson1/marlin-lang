use std::hash::Hash;

use serde::Serialize;

use crate::lexer::token::{Located, PositionRange};

#[derive(Serialize, Clone)]
pub struct Path {
    // Store segments and their locations separately for easier access
    pub segments: Vec<String>,
    pub locations: Vec<PositionRange>,
}

impl Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for segment in &self.segments {
            segment.hash(state);
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        if self.segments.len() != other.segments.len() {
            return false;
        }

        for (a, b) in self.segments.iter().zip(other.segments.iter()) {
            if a != b {
                return false;
            }
        }

        true
    }
}

impl Eq for Path {}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.segments.join("::")
    }
}

impl Path {
    pub fn new(segments: Vec<Located<String>>) -> Self {
        let (segment_strings, segment_positions): (Vec<_>, Vec<_>) = segments
            .into_iter()
            .map(|s| s.into_parts())
            .unzip();

        Self {
            segments: segment_strings,
            locations: segment_positions,
        }
    }

    pub fn extend(&mut self, other: &Path) {
        self.segments.extend_from_slice(&other.segments);
        self.locations.extend_from_slice(&other.locations);        
    }
}