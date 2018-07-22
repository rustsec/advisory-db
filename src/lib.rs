extern crate rustsec;

#[cfg(test)]
mod tests {
    use rustsec::{AdvisoryDatabase, Repository};

    #[test]
    fn advisories_toml_is_well_formed() {
        let repo = Repository::open(".").unwrap();

        // Ensure Advisories.toml parses
        let advisory_count = AdvisoryDatabase::from_repository(&repo)
            .unwrap()
            .advisories()
            .count();

        // Ensure we're parsing some advisories
        assert!(advisory_count > 5);
    }
}
