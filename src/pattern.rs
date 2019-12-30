use std::io::{Error, ErrorKind};

const WILDCARD_STAR: u8 = 0x2A;
const WILDCARD_QUESTION: u8 = 0x3F;

pub enum MatchType {
    Both,
    Left,
    Right,
    None,
}

pub struct MemoryPattern {
    pub data: Vec<(u8, MatchType)>,
    pub offset: usize,
}

impl MemoryPattern {
    pub fn from_string(offset: usize, tokens: &str) -> Result<MemoryPattern, Error> {
        fn invalid_token(token: &str) -> Error {
            Error::new(ErrorKind::Other, format!("invalid token: {}", token))
        }

        fn is_wildcard(char_code: u8) -> bool {
            char_code == WILDCARD_STAR || char_code == WILDCARD_QUESTION
        }

        let mut data = Vec::new();

        for token in tokens.split_whitespace() {
            let bytes = token.as_bytes();

            match bytes.len() {
                1 => {
                    if is_wildcard(bytes[0]) {
                        data.push((0x00, MatchType::None));
                    } else {
                        return Err(invalid_token(token));
                    }
                }

                2 => {
                    let lwild = is_wildcard(bytes[0]);
                    let rwild = is_wildcard(bytes[1]);

                    if lwild && rwild {
                        data.push((0x00, MatchType::None));
                    } else if !lwild && !rwild {
                        let byte =
                            u8::from_str_radix(token, 16).map_err(|_| invalid_token(token))?;
                        data.push((byte, MatchType::Both));
                    } else if lwild {
                        let byte = u8::from_str_radix(&token[1..2], 16)
                            .map_err(|_| invalid_token(token))?;
                        data.push((byte, MatchType::Right));
                    } else {
                        let byte = u8::from_str_radix(&token[0..1], 16)
                            .map_err(|_| invalid_token(token))?;
                        data.push((byte << 4, MatchType::Left));
                    }
                }

                _ => return Err(invalid_token(token)),
            }
        }

        println!("created pattern with {} bytes", data.len());

        Ok(MemoryPattern { data, offset })
    }
}
