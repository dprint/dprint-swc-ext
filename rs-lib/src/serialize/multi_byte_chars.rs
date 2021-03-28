use swc_common::BytePos;

// type from swc
pub struct MultiByteChar {
    /// The absolute offset of the character in the SourceMap
    pub pos: BytePos,
    /// The number of bytes, >=2
    pub bytes: u8,
}

pub fn get_multi_byte_chars(file_text: &str) -> Vec<MultiByteChar> {
    let mut multi_byte_chars = Vec::new();
    let mut last_indice = 0;
    for (indice, _) in file_text.char_indices() {
        let diff = (indice - last_indice) as u8;
        if diff >= 2 {
            multi_byte_chars.push(MultiByteChar {
                pos: BytePos(last_indice as u32),
                bytes: diff,
            });
        }
        last_indice = indice;
    }

    // check the last char
    let diff = (file_text.len() - last_indice) as u8;
    if diff >= 2 {
        multi_byte_chars.push(MultiByteChar {
            pos: BytePos(last_indice as u32),
            bytes: diff,
        });
    }
    multi_byte_chars
}

pub fn byte_pos_to_char_pos(multi_byte_chars: &Vec<MultiByteChar>, byte_pos: BytePos) -> u32 {
    let mut char_pos = byte_pos.0;
    for multi_byte_char in multi_byte_chars.iter() {
        if multi_byte_char.pos < byte_pos {
            char_pos -= (multi_byte_char.bytes - 1) as u32;
        } else {
            break;
        }
    }
    char_pos as u32
}
