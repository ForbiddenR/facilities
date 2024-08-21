use std::{fs::File, io::Write};

fn main() {
    let mut local_file = File::create("test.txt").unwrap();
    say_hello(&mut local_file).unwrap();
    say_bye(&mut local_file);
}

fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()>  {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn say_bye(_out: &mut impl Write) {
    
}

