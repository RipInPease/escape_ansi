// Not implemented:
// ESC[6n

use crate::Command;


use std::io::Write;
use std::io::Result as IOResult;


const ESC: &[u8; 1] = b"\x1b";


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


/// Moves cursor up desired rows
/// 
#[derive(Debug, Clone)]
pub struct MoveUp(pub u16);


impl<W: Write> Command<W> for MoveUp {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        write!(
            w,
            ESC,
            format!("[{}A", self.0).as_bytes()
        )
    }
}


/// Moves cursor down desired rows
/// 
#[derive(Debug, Clone)]
pub struct MoveDown(pub u16);


impl<W: Write> Command<W> for MoveDown {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        write!(
            w,
            ESC,
            format!("[{}B", self.0).as_bytes()
        )
    }
}


/// Moves cursor right desired columns
/// 
#[derive(Debug, Clone)]
pub struct MoveRight(pub u16);


impl<W: Write> Command<W> for MoveRight {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        write!(
            w,
            ESC,
            format!("[{}C", self.0).as_bytes()
        )
    }
}


/// Moves cursor left desired columns
/// 
#[derive(Debug, Clone)]
pub struct MoveLeft(pub u16);


impl<W: Write> Command<W> for MoveLeft {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        write!(
            w,
            ESC,
            format!("[{}D", self.0).as_bytes()
        )
    }
}


/// Moves cursor to beginning of next row, # rows down
/// 
#[derive(Debug, Clone)]
pub struct MoveDownRows(pub u16);


impl<W: Write> Command<W> for MoveDownRows {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        write!(
            w,
            ESC,
            format!("[{}E", self.0).as_bytes()
        )
    }
}



/// Moves cursor to previous of next row, # rows up
/// 
#[derive(Debug, Clone)]
pub struct MoveUpRows(pub u16);


impl<W: Write> Command<W> for MoveUpRows {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        write!(
            w,
            ESC,
            format!("[{}F", self.0).as_bytes()
        )
    }
}


/// Moves cursor to desired column
/// 
#[derive(Debug, Clone)]
pub struct MoveToCol(pub u16);


impl<W: Write> Command<W> for MoveToCol {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        write!(
            w,
            ESC,
            format!("[{}G", self.0).as_bytes()
        )
    }
}


/// Moves the cursor up once, scrolling if needed
/// 
pub struct MoveUpOnce;


impl<W: Write> Command<W> for MoveUpOnce {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        write!(
            w,
            ESC,
            "M".as_bytes()
        )
    }
}


/// Save cursor position
/// 
pub struct SavePos;


impl<W: Write> Command<W> for SavePos {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        write!(
            w,
            ESC,
            "[s".as_bytes()
        )
    }
}


/// Restore saved cursor position
/// 
pub struct RestorePos;


impl<W: Write> Command<W> for RestorePos {
    fn write_ansi(&self, w: &mut W) -> IOResult<()> {
        write!(
            w,
            ESC,
            "[r".as_bytes()
        )
    }
}