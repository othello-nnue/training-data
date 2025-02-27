use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct OthelloAnalysis {
    pub board: [u64; 2],
    pub settings: [u8; 2],
    pub bounds: [i8; 2],
    pub nodes: i64,
}

impl FromStr for OthelloAnalysis {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PATTERN: Regex =
                Regex::new(r"^([-OX]{64}) X;,(\d+),(\d+),(-?\d+),(-?\d+),(-?\d+)$").unwrap();
        }
        let captures = PATTERN.captures(s).ok_or("Unmatched sequence")?;
        
        let board_str = captures.get(1).unwrap().as_str();
        let settings0 = captures.get(2).unwrap().as_str().parse::<u8>()?;
        let settings1 = captures.get(3).unwrap().as_str().parse::<u8>()?;
        let bounds0 = captures.get(4).unwrap().as_str().parse::<i8>()?;
        let bounds1 = captures.get(5).unwrap().as_str().parse::<i8>()?;
        let nodes = captures.get(6).unwrap().as_str().parse::<i64>()?;

        let mut board: [u64; 2] = [0, 0];
        for (i, c) in board_str.chars().enumerate() {
            match c {
                'O' => board[0] |= 1 << i,
                'X' => board[1] |= 1 << i,
                '-' => {}
                _ => panic!("Invalid character in board string"),
            }
        }

        Ok(OthelloAnalysis {
            board,
            settings: [settings0, settings1],
            bounds: [bounds0, bounds1],
            nodes,
        })
    }
}

impl OthelloAnalysis {
    pub fn to_bytes(&self) -> [u8; 18] {
        let mut bytes = [0u8; 18];

        bytes[0..8].copy_from_slice(&self.board[0].to_le_bytes());
        bytes[8..16].copy_from_slice(&self.board[1].to_le_bytes());
        bytes[16..18].copy_from_slice(&self.bounds.map(|x| x as u8));
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let s = "---------------------------OX------XO--------------------------- X;,0,0,0,0,10";
        let analysis = OthelloAnalysis::from_str(s).unwrap();
        assert_eq!(analysis.board[0], 0x0000_0010_0800_0000);
        assert_eq!(analysis.board[1], 0x0000_0008_1000_0000);
        assert_eq!(analysis.settings, [0, 0]);
        assert_eq!(analysis.bounds, [0, 0]);
        assert_eq!(analysis.nodes, 10);
    }

    #[test]
    fn test_to_bytes() {
        let analysis = OthelloAnalysis {
            board: [0x0000_0010_0800_0000, 0x0000_0008_1000_0000],
            settings: [1, 2],
            bounds: [-3, 4],
            nodes: 12345,
        };

        let bytes = analysis.to_bytes();
        let expected_bytes: [u8; 18] = [
            0x00, 0x00, 0x00, 0x08, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x08, 0x00,
            0x00, 0x00, 0xFD, 0x04,
        ];
        assert_eq!(bytes, expected_bytes);
    }
}
