/// Commands and such for the cursor
/// 
pub mod cursor;


use std::io::Write;
use std::io::Result as IOResult;


/// A command that can be written to a terminal
/// 
pub trait Command<W: Write> {
    fn write_ansi(w: &mut W) -> IOResult<()>; 
}
