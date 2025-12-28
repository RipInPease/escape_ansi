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
        let mut bytes = Vec::with_capacity(2);
        bytes.extend_from_slice(ESC);
        bytes.extend_from_slice(CSI);
        bytes.extend_from_slice("H".as_bytes());

        w.write(&bytes)?;
        Ok(())
    }
}



/// Moves cursor to desires (column, row)
/// 
#[derive(Debug, Clone)]
pub struct MoveTo(pub u16, pub u16);


impl<W: Write> Command<W> for MoveTo {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        let mut bytes = Vec::with_capacity(10);
        bytes.extend_from_slice(ESC);
        bytes.extend_from_slice(CSI);

        let column = &self.1.to_string();
        bytes.extend_from_slice(column.as_bytes());

        bytes.extend_from_slice(SEMI_COLON);

        let row = &self.0.to_string();
        bytes.extend_from_slice(row.as_bytes());

        bytes.extend_from_slice("H".as_bytes());

        w.write(&bytes)?;

        Ok(())
    }
}