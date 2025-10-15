/// Common test utilities and fixture loading for disruption_types tests
use std::path::PathBuf;

/// Load a JSON fixture file from the fixtures directory
pub fn load_fixture(category: &str, filename: &str) -> String {
    let fixture_path = fixture_path(category, filename);
    std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|e| panic!("Failed to load fixture {}: {}", fixture_path.display(), e))
}

/// Get the path to a fixture file
pub fn fixture_path(category: &str, filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(category);
    path.push(filename);
    path
}

/// Check if a fixture file exists
pub fn fixture_exists(category: &str, filename: &str) -> bool {
    fixture_path(category, filename).exists()
}

/// Load and parse a JSON fixture into a type
pub fn load_fixture_as<T>(category: &str, filename: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let json = load_fixture(category, filename);
    serde_json::from_str(&json)
        .unwrap_or_else(|e| panic!("Failed to parse fixture {}/{}: {}", category, filename, e))
}

/// Test that a value can be serialized and deserialized without loss
pub fn assert_roundtrip<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(value).expect("Failed to serialize");
    let deserialized: T = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(value, &deserialized, "Roundtrip failed");
}

/// Test that a JSON string can be deserialized and re-serialized to equivalent JSON
pub fn assert_json_roundtrip<T>(json: &str)
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let parsed: T = serde_json::from_str(json).expect("Failed to parse JSON");
    let reserialized = serde_json::to_value(&parsed).expect("Failed to serialize");
    let original: serde_json::Value = serde_json::from_str(json).expect("Failed to parse original");

    // Note: We compare as JSON values to allow for ordering differences
    assert_eq!(
        original, reserialized,
        "JSON roundtrip produced different output"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_path() {
        let path = fixture_path("users", "basic_user.json");
        let path_str = path.to_string_lossy();

        // Check for key path components (cross-platform compatible)
        assert!(path_str.contains("tests"));
        assert!(path_str.contains("fixtures"));
        assert!(path_str.contains("users"));
        assert!(path_str.contains("basic_user.json"));

        // Verify it ends with the filename
        assert!(path_str.ends_with("basic_user.json"));
    }
}
