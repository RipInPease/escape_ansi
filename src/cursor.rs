use crate::Command;


use std::io::Write;
use std::io::Result as IOResult;


const ESC: &[u8; 1] = b"\x1b";


/// Moves cursor to home position (0, 0)
/// 
pub struct MoveHome;


impl<W: Write> Command<W> for MoveHome {
    fn write_ansi(w: &mut W) -> IOResult<()> {
        let bytes = [ESC[0], b'[', b'H'];

        w.write(&bytes)?;
        Ok(())
    }
}