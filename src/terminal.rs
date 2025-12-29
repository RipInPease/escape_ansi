use crate::Command;


use std::io::Write;
use std::io::Result as IOResult;


const ESC: &[u8; 1] = b"\x1b";






/// Erase functions
/// 
/// Note: Erasing the line won't move the cursor, 
/// meaning that the cursor will stay at the last 
/// position it was at before the line was erased. 
/// You can use \r after erasing the line, to return 
/// the cursor to the start of the current line.
/// 
pub enum Erase {

    /// Erase from cursor until end of screen
    /// 
    FromCursor,

    /// Erase from cursor to beginning of screen
    /// 
    ToCursor,

    /// Erase entire screen
    /// 
    Screen,

    /// Erase saved lines
    /// 
    Saved,

    /// Erase from cursor to end of line
    /// 
    CursorToLine,

    /// Erase start of line to the cursor
    /// 
    LineToCursor,

    /// Erase the entire line
    /// 
    Line,
}


impl<W: Write> Command<W> for Erase {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        let csi = match self {
            Self::FromCursor    => "[0J",
            Self::ToCursor      => "[1J",
            Self::Screen        => "[2J",
            Self::Saved         => "[3J",
            Self::CursorToLine  => "[0K",
            Self::LineToCursor  => "[1K",
            Self::Line          => "[2K"
        };

        write!(
            w,
            ESC,
            csi.as_bytes()
        )
    }
}