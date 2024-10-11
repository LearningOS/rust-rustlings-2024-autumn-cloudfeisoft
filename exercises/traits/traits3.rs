pub trait Licensed {
    // Provide a default implementation for the licensing_info method
    fn licensing_info(&self) -> String {
        String::from("Licensed under the MIT License")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

// Since we provided a default implementation, we don't need to implement the method again for these structs
impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Licensed under the MIT License");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}