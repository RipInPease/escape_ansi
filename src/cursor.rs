use crate::Command;


use std::io::Write;
use std::io::Result as IOResult;


const ESC: &[u8; 1] = b"\x1b";
const CSI: &[u8; 1] = b"[";
const SEMI_COLON: &[u8; 1] = b";";


/// Moves cursor to home position (0, 0)
/// 
#[derive(Debug, Clone)]
pub struct MoveHome;


impl<W: Write> Command<W> for MoveHome {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        crate::write!(
            w,
            ESC,
            "[H".as_bytes()
        )
    }
}



/// Moves cursor to desired (column, row)
/// 
#[derive(Debug, Clone)]
pub struct MoveTo(pub u16, pub u16);


impl<W: Write> Command<W> for MoveTo {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        write!(
            w,
            ESC,
            format!("[{};{}H", self.1, self.0).as_bytes()
        )
    }
}