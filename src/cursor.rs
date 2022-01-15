// Copyright (c) 2022 Sam Blenny
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
use crate::cliprect::ClipRect;
use crate::pt::Pt;

/// Cursor specifies a drawing position along a line of text. Lines of text can
/// be different heights. Line_height is for keeping track of the tallest
/// character that has been drawn so far on the current line.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cursor {
    pub pt: Pt,
    pub line_height: usize,
}

impl Cursor {
    // Make a new Cursor. When in doubt, set line_height = 0.
    pub fn new(x: usize, y: usize, line_height: usize) -> Cursor {
        Cursor {
            pt: Pt { x, y },
            line_height,
        }
    }

    // Make a Cursor aligned at the top left corner of a ClipRect
    pub fn from_top_left_of(r: ClipRect) -> Cursor {
        Cursor {
            pt: r.min,
            line_height: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_equivalence() {
        let c1 = Cursor {
            pt: Pt { x: 1, y: 2 },
            line_height: 0,
        };
        let c2 = Cursor::new(1, 2, 0);
        assert_eq!(c1, c2);
        let clip = ClipRect::new(1, 2, 3, 4);
        let c3 = Cursor::from_top_left_of(clip);
        assert_eq!(c1, c3);
    }

    #[test]
    fn test_cursor_from_clip_rect() {
        let cr = ClipRect::new(1, 2, 8, 9);
        let c = Cursor::from_top_left_of(cr);
        assert_eq!(c.pt, cr.min);
    }
}
