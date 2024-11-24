use crate::database::DB_LOCATION;
use std::fs::remove_file;
use std::io;
use std::path::Path;

pub fn delete_database() -> Result<(), crate::Error> {
    let path = &*DB_LOCATION;

    delete_or_not_found(path)?;
    delete_or_not_found(format!("{}-wal", path.to_string_lossy()))?;
    delete_or_not_found(format!("{}-shm", path.to_string_lossy()))?;

    Ok(())
}

fn delete_or_not_found<P: AsRef<Path>>(path: P) -> Result<(), crate::Error> {
    match remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                return Ok(());
            }

            Err(crate::Error::DatabaseIoError(err))
        }
    }
}
