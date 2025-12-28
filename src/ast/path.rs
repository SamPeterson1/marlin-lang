use serde::Serialize;

use crate::{impl_positioned, lexer::token::{Located, PositionRange, Positioned}};

#[derive(Serialize, Clone)]
pub struct Path {
    pub segments: Vec<Located<String>>,
    position: PositionRange,
}

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

    pub fn extend(&mut self, other: &Path) {
        self.segments.extend_from_slice(&other.segments);
        self.position = PositionRange::concat(&self.position, &other.position);
    }
}

impl_positioned!(Path);