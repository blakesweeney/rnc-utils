use std::path::{Path, PathBuf};
use std::str;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Urs(u64);

impl Urs {
    pub fn to_string(&self) -> String {
        format!("URS{:010X}", self.0)
    }
}

impl FromStr for Urs {
    type Err = std::num::ParseIntError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        u64::from_str_radix(&raw[3..], 16).map(|s| Urs(s))
    }
}

impl From<u64> for Urs {
    fn from(raw: u64) -> Urs {
        Urs(raw)
    }
}

impl From<&Urs> for String {
    fn from(urs: &Urs) -> String {
        format!("URS{:010X}", urs.0)
    }
}

impl From<Urs> for u64 {
    fn from(urs: Urs) -> u64 {
        urs.0
    }
}

impl From<&Urs> for u64 {
    fn from(urs: &Urs) -> u64 {
        urs.0
    }
}

impl Urs {
    pub fn looks_like_urs(urs: &str) -> bool {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r"URS[0-9A-F]{10}$").unwrap();
        }
        PATTERN.is_match(urs)
    }

    pub fn directory_path(&self, base: &Path) -> PathBuf {
        let mut path = PathBuf::from(base);
        path.push("URS");
        let urs: String = self.into();
        for x in (3..11).step_by(2) {
            path.push(urs[x..(x + 2)].to_string());
        }
        path
    }

    pub fn path_for(&self, base: &Path, extension: &str) -> PathBuf {
        let mut path = self.directory_path(&base);
        let urs: String = self.into();
        path.push(urs);
        path.set_extension(extension);
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn can_convert_string_to_urs() -> Result<(), Box<dyn Error>> {
        assert_eq!(Urs::try_from("URS0000000009")?, Urs(9));
        assert_eq!(Urs::try_from("URS000000000A")?, Urs(10));
        assert_eq!(Urs::try_from("URS0000C0472E")?, Urs(12601134));
        assert_eq!(Urs::try_from("URS0000000001")?, Urs(1));
        assert_eq!(Urs::try_from("URS00001EE391")?, Urs(2024337));
        Ok(())
    }

    #[test]
    fn can_convert_string_to_urs_taxid() -> Result<(), Box<dyn Error>> {
        assert_eq!(UrsTaxid::try_from("URS0000000009_1")?, UrsTaxid(9, 1));
        assert_eq!(UrsTaxid::try_from("URS0000C0472E_12445")?, UrsTaxid(12601134, 12445));
        assert_eq!(UrsTaxid::try_from("URS0000000001_562")?, UrsTaxid(1, 562));
        assert_eq!(UrsTaxid::try_from("URS00008B8A75_9606")?, UrsTaxid(9144949, 9606));
        assert_eq!(UrsTaxid::try_from("URS00001EE391_1250050")?, UrsTaxid(2024337, 1250050));
        assert_eq!(UrsTaxid::try_from("URS00008C3642_9606")?, UrsTaxid(9188930, 9606));
        Ok(())
    }

    #[test]
    fn matches_urs() {
        assert_eq!(Urs::looks_like_urs("URS00000001AAB82D"), false);
        assert_eq!(Urs::looks_like_urs("URS00000001B1"), true);
        assert_eq!(Urs::looks_like_urs("URS0000000362"), true);
    }

    #[test]
    fn correctly_generates_urs() {
        assert_eq!(Urs::from(12601134u64).to_string(), String::from("URS0000C0472E"));
        assert_eq!(Urs::from(1u64).to_string(), String::from("URS0000000001"));
        assert_eq!(Urs::from(9u64).to_string(), String::from("URS0000000009"));
    }
}

//     #[test]
//     fn extracts_urs() {
//         assert_eq!(
//             filename_urs(Path::new("a/b/URS0000000372..svg.gz")),
//             Some("URS0000000372".to_string())
//         );
//         assert_eq!(
//             filename_urs(Path::new("URS0000000372.svg.gz")),
//             Some("URS0000000372".to_string())
//         );
//         assert_eq!(
//             filename_urs(Path::new("URS0000000372.svg")),
//             Some("URS0000000372".to_string())
//         );
//         assert_eq!(
//             filename_urs(Path::new("URS0000000372")),
//             Some("URS0000000372".to_string())
//         );
//         assert_eq!(
//             filename_urs(Path::new("URS000042DD9D.colored.svg")),
//             Some("URS000042DD9D".to_string())
//         );
//         assert_eq!(filename_urs(Path::new("URS00000002D191B..svg.gz")), None);
//         assert_eq!(filename_urs(Path::new("URS00000002C67ED..svg.gz")), None);
//         assert_eq!(filename_urs(Path::new("URS00000002C67ED..svg")), None);
//         assert_eq!(filename_urs(Path::new("URS00000002C67ED.")), None);
//         assert_eq!(filename_urs(Path::new("URS00000002C67ED")), None);
//         assert_eq!(
//             filename_urs(Path::new("URS0000C2D164-E-Ser.colored.svg")),
//             Some("URS0000C2D164".to_string())
//         );
//     }

//     #[test]
//     fn creates_correct_final_path() {
//         let mut result = PathBuf::from("foo");
//         result.push("URS");
//         result.push("00");
//         result.push("00");
//         result.push("00");
//         result.push("03");
//         result.push("URS0000000372");
//         result.set_extension("svg.gz");
//         assert_eq!(
//             path_for(&PathBuf::from("foo"), &"URS0000000372".to_string()),
//             result
//         );
//     }
