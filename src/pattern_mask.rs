use std::fmt;

pub struct PatternMask {
    pub string_byte_array: Vec<String>,
    pub aob_pattern: Vec<u8>,
    pub mask: Vec<u8>,
}

impl PatternMask {
    pub fn aob_to_pattern_mask(search: &str) -> PatternMask {
        let mut d = PatternMask {
            string_byte_array: Vec::new(),
            aob_pattern: Vec::new(),
            mask: Vec::new(),
        };

        d.string_byte_array = search
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let length = d.string_byte_array.len();
        d.aob_pattern = vec![0; length];
        d.mask = vec![0; length];

        for (i, ba) in d.string_byte_array.iter().enumerate() {
            if ba == "??" || (ba.len() == 1 && ba == "?") {
                d.mask[i] = 0x00;
                d.aob_pattern[i] = 0x00; // Set aob_pattern for wildcards as well
            } else if ba.starts_with(|arg0: char| char::is_ascii_alphanumeric(&arg0)) && ba.chars().nth(1) == Some('?') {
                d.mask[i] = 0xF0;
                let hex_value =
                    u8::from_str_radix(&format!("{}0", ba.chars().next().unwrap()), 16).unwrap();
                d.aob_pattern[i] = hex_value;
            } else if ba.starts_with('?') && ba.chars().nth(1).expect("Failed to extract second character").is_ascii_alphanumeric() {
                d.mask[i] = 0x0F;
                let hex_value =
                    u8::from_str_radix(&format!("0{}", ba.chars().nth(1).unwrap()), 16).unwrap();
                d.aob_pattern[i] = hex_value;
            } else {
                d.mask[i] = 0xFF;
                // Correctly convert and apply mask to aob_pattern
                let hex_value = if let Some(stripped) = ba.strip_prefix("0x") {
                    u8::from_str_radix(stripped, 16).unwrap_or(0x00)
                } else {
                    u8::from_str_radix(ba, 16).unwrap_or(0x00)
                };
                d.aob_pattern[i] = hex_value & d.mask[i]; // Apply mask AFTER conversion
            }
        }

        d
    }

    // Method to convert mask to a string representation
    pub fn mask_to_string(&self) -> String {
        self.mask
            .iter()
            .map(|&byte| {
                if byte == 0xFF {
                    "x".to_string() // Fully defined byte
                } else {
                    "?".to_string() // Wildcard byte
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }
}

impl fmt::LowerHex for PatternMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pattern: ")?; // Label for clarity

        // Format aob_pattern
        for &byte in &self.aob_pattern {
            write!(f, "{:02x} ", byte)?; // Print each byte in hex with a space
        }

        write!(f, "Mask: ")?; // Label for clarity

        // Format mask
        for &byte in &self.mask {
            if byte == 0xFF {
                write!(f, "x")?; // Fully defined byte
            } else {
                write!(f, "?")?; // Wildcard byte
            }
        }
        Ok(())
    }
}
