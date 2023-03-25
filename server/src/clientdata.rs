const MAX_NICKNAME_LENGTH: usize = 16 * 4; // 64 utf-8 fulfill symbols
const MAX_CLAN_LENGTH: usize = 4 * 4; // 4 utf-8 fulfill symbols

pub struct ArrayString<const L: usize> {
    array: [u8; L]
}

impl<const L: usize> ArrayString<L> {
    pub fn new() -> Self {
        Self {
            array: [0; L]
        }
    }

    pub fn from_string<S: Into<String>>(string: S) -> Self {
        let string = string.into();
        let mut array = [0; L];

        let string_slice = string.into_bytes();
        let length = usize::min(L, string_slice.len());
        
        array.copy_from_slice(&string_slice[..length]);

        Self {
            array
        }
    }

    pub fn to_string(&self) -> String {
        std::string::String::from_utf8_lossy(&self.array).to_string()
    }
}

impl<const L: usize> std::fmt::Debug for ArrayString<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub struct ClientData {
    pub nickname: ArrayString<MAX_NICKNAME_LENGTH>,
    pub clan: ArrayString<MAX_CLAN_LENGTH>
}

impl ClientData {
    pub fn new() -> Self {
        Self {
            nickname: ArrayString::new(),
            clan: ArrayString::new()
        }
    }
}
