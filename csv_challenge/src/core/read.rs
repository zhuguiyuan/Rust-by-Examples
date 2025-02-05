use super::{Error, File, PathBuf, Read, Write};

fn open(path: PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

fn read(path: PathBuf) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut buffer)?;
    if buffer.is_empty() {
        Err("input file missing")?
    } else {
        Ok(buffer)
    }
}

fn write(data: &str, filename: &str) -> Result<(), Error> {
    let mut buffer = File::create(filename)?;
    buffer.write_all(data.as_bytes())?;
    Ok(())
}

pub fn load_csv(csv_file: PathBuf) -> Result<String, Error> {
    let file = read(csv_file)?;
    Ok(file)
}

pub fn write_csv(csv_data: &str, filename: &str) -> Result<(), Error> {
    write(csv_data, filename)?;
    Ok(())
}
