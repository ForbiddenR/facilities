use std::{fs::File, io::Write};

trait Observer {
    type Persion;

    fn observe() -> Option<Self::Persion>;
}

fn main() {
    let mut local_file = File::create("test.txt").unwrap();
    say_hello(&mut local_file);
    say_bye(&mut local_file);
}

fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()>  {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn say_bye(out: &mut impl Write) {
    
}

