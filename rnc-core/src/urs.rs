use std::path::{Path, PathBuf};
use std::str;
use std::convert::TryFrom;

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Urs(u64);

#[derive(Debug, PartialEq)]
struct UrsTaxid(u64, u64);

impl TryFrom<&str> for Urs {
    type Error = std::num::ParseIntError;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        u64::from_str_radix(&raw[3..], 16).map(|s| Urs(s))
    }
}

impl From<&Urs> for String {
    fn from(urs: &Urs) -> String {
        format!("URS{:010X}", urs.0)
    }
}

impl From<&UrsTaxid> for String {
    fn from(urs: &UrsTaxid) -> String {
        format!("URS{:010X}_{}", urs.0, urs.1)
    }
}

impl From<&UrsTaxid>for Urs {
    fn from(urs: &UrsTaxid) -> Urs {
        Urs(urs.0)
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
        return PATTERN.is_match(urs);
    }

    pub fn directory_path(&self, base: &Path) -> PathBuf {
        let mut path = PathBuf::from(base);
        path.push("URS");
        let urs: String = self.into();
        for x in (3..11).step_by(2) {
            path.push(urs[x..(x + 2)].to_string());
        }
        return path;
    }

    pub fn path_for(&self, base: &Path, extension: &str) -> PathBuf {
        let mut path = self.directory_path(&base);
        let urs: String = self.into();
        path.push(urs);
        path.set_extension(extension);
        return path;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn matches_urs() {
//         assert_eq!(looks_like_urs("URS00000001AAB82D"), false);
//         assert_eq!(looks_like_urs("URS00000001B1"), true);
//         assert_eq!(looks_like_urs("URS0000000362"), true);
//     }

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

//     #[test]
//     fn correctly_generates_urs() {
//         assert_eq!(int_to_urs(12601134), String::from("URS0000C0472E"));
//         assert_eq!(int_to_urs(1), String::from("URS0000000001"));
//         assert_eq!(int_to_urs(9), String::from("URS0000000009"));
//     }

//     #[test]
//     fn correctly_generates_urs_index() -> Result<()> {
//         assert_eq!(urs_to_index(&String::from("URS0000000009"))?, 9);
//         assert_eq!(urs_to_index(&String::from("URS0000C0472E"))?, 12601134);
//         assert_eq!(urs_to_index(&String::from("URS0000000001"))?, 1);
//         return Ok(());
//     }
// }
