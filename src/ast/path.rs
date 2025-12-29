use std::hash::Hash;

use serde::Serialize;

use crate::{impl_positioned, lexer::token::{Located, PositionRange, Positioned}};

#[derive(Serialize, Clone)]
pub struct Path {
    pub segments: Vec<Located<String>>,
    position: PositionRange,
}

impl Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for segment in &self.segments {
            segment.data.hash(state);
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        if self.segments.len() != other.segments.len() {
            return false;
        }

        for (a, b) in self.segments.iter().zip(other.segments.iter()) {
            if a.data != b.data {
                return false;
            }
        }

        true
    }
}

impl Eq for Path {}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.segments.iter().map(|s| s.data.clone()).collect::<Vec<_>>().join("::")
    }
}

impl Path {
    pub fn new(segments: Vec<Located<String>>) -> Self {
        let position = if segments.is_empty() {
            PositionRange::zero()
        } else {
            PositionRange::concat(
                segments.first().unwrap().get_position(),
                segments.last().unwrap().get_position(),
            )
        };

        Self {
            segments,
            position,
        }
    }

    pub fn with_separator(&self, separator: &str) -> String {
        self.segments.iter().map(|s| s.data.clone()).collect::<Vec<_>>().join(separator)
    }

    pub fn extend(&mut self, other: &Path) {
        self.segments.extend_from_slice(&other.segments);
        self.position = PositionRange::concat(&self.position, &other.position);
    }
}

impl_positioned!(Path);