use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

type GenericError = Box<dyn Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

pub fn support_architecture_v3() -> Result<bool, impl Error> {
    let path = Path::new("/proc/cpuinfo");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for l in reader.lines() {
        let l = l?;
        if l.starts_with("model name") {
            let ll = l.to_lowercase();
            return Ok(ll.contains("v3")
                || ll.contains("v4")
                || ll.contains("amd")
                || ll.contains("xeon(r)"));
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Model name not found",
    ))
}

pub fn test_contains(name: &str) {
    println!("{}", name.contains("a|b"));
}

#[cfg(test)]
mod test {
    use crate::support_architecture_v3;

    #[test]
    fn test_support_architecture_v3() {
        if let Ok(result) = support_architecture_v3() {
            assert_eq!(true, result);
        }
    }
}
