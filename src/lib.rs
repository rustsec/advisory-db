extern crate rustsec;

#[cfg(test)]
mod tests {
    // Name of the advisory database in the current repo
    const ADVISORY_DB_FILE: &'static str = "Advisories.toml";

    use rustsec::AdvisoryDatabase;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn advisories_toml_is_well_formed() {
        let path = Path::new(ADVISORY_DB_FILE);

        let mut file = File::open(&path).unwrap();

        let mut toml = String::new();
        file.read_to_string(&mut toml).unwrap();

        // Ensure Advisories.toml parses
        AdvisoryDatabase::from_toml(&toml).unwrap();
    }
}
