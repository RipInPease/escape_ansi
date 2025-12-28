use std::io::Write;
use std::io::Result as IOResult;




/// Write all u8 slices to a writer
/// 
#[macro_export]
macro_rules! write {
    ($w:ident, $($b:expr),*) => {{
        let mut bytes = Vec::new();
        $(bytes.extend_from_slice($b);)*
        $w.write(&bytes)?;
        Ok(())
    }};
}


/// Commands and such for the cursor
/// 
pub mod cursor;


/// A command that can be written to a terminal
/// 
pub trait Command<W: Write> {
    fn write_ansi(&self, w: &mut W) -> IOResult<()>; 
}
