use crate::websocket::opcode::Opcode;

#[derive(Clone)]
pub struct Frame {
    fin: bool,
    rsv1: bool,
    rsv2: bool,
    rsv3: bool,
    opcode: Opcode,
    mask: bool,
    payload_len: usize,
    masking_key: Option<[u8; 4]>,
    payload_data: Vec<u8>,
}

impl Frame {
    pub fn fin(&self) -> bool { self.fin }
    pub fn rsv1(&self) -> bool { self.rsv1 }
    pub fn rsv2(&self) -> bool { self.rsv2 }
    pub fn rsv3(&self) -> bool { self.rsv3 }
    pub fn opcode(&self) -> Opcode { self.opcode.clone() }
    pub fn mask(&self) -> bool { self.mask }
    pub fn payload_len(&self) -> usize { self.payload_len }
    pub fn masking_key(&self) -> Option<[u8; 4]> { self.masking_key }
    pub fn payload_data(&self) -> Vec<u8> { self.payload_data.clone() }

    pub fn new(opcode: Opcode, /*mask: bool,*/ payload_data: Option<Vec<u8>>) -> Self {
        let (payload_len, payload_data) = match payload_data {
            Some(payload_data) => (payload_data.len(), payload_data),
            None => (0, vec![]),
        };

        Frame {
            fin: true, // Fragmentation is not support, so always 1
            rsv1: false,
            rsv2: false,
            rsv3: false,
            opcode,
            mask: false,
            payload_len,
            masking_key: None,
            payload_data,
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(
            (self.fin as u8) << 7
                | (self.rsv1 as u8) << 6
                | (self.rsv2 as u8) << 5
                | (self.rsv3 as u8) << 4
                | u8::from(self.opcode),
        );

        if self.payload_len < 126 {
            buffer.push((self.mask as u8) << 7 | self.payload_len as u8);
        } else if self.payload_len < 65536 {
            buffer.push((self.mask as u8) << 7 | 126_u8);
        } else {
            buffer.push((self.mask as u8) << 7 | 127_u8);
            buffer.extend_from_slice((self.payload_len as u64).to_be_bytes().as_ref());
        }

        if self.mask {
            buffer.extend(self.masking_key.unwrap().clone());
        }

        for (i, b) in self.payload_data.iter().enumerate() {
            buffer.push(if self.mask {
                b ^ self.masking_key.unwrap()[i % 4]
            } else {
                *b
            });
        }

        return buffer
    }
}

impl From<&[u8]> for Frame {
    fn from(buffer: &[u8]) -> Self {
        let fin = buffer[0] & 0x80 != 0x00;
        let rsv1 = buffer[0] & 0x40 != 0x00;
        let rsv2 = buffer[0] & 0x20 != 0x00;
        let rsv3 = buffer[0] & 0x10 != 0x00;
        let opcode = Opcode::from(buffer[0]);

        let mask = buffer[1] & 0x80 != 0;
        let (payload_len, mut i) = match buffer[1] & 0x7F {
            0x7E => {
                let mut payload_len = [0; 2];
                payload_len.copy_from_slice(&buffer[2..4]);
                (u16::from_be_bytes(payload_len) as usize, 4)
            }
            0x7F => {
                let mut payload_len = [0; 8];
                payload_len.copy_from_slice(&buffer[2..10]);
                (usize::from_be_bytes(payload_len), 10)
            }
            n => (n as usize, 2)
        };

        let masking_key = if mask {
            let mut masking_key = [0; 4];
            masking_key.copy_from_slice(&buffer[i..i + 4]);
            i += 4;
            Some(masking_key)
        } else {
            None
        };
        let payload_data: Vec<u8> = if mask {
            buffer[i..i + payload_len]
                .iter()
                .enumerate()
                .map(|(i, b)| b ^ masking_key.unwrap()[i % 4])
                .collect()
        } else {
            buffer[i..i + payload_len].to_vec()
        };

        Frame {
            fin,
            rsv1,
            rsv2,
            rsv3,
            opcode,
            mask,
            payload_len,
            masking_key,
            payload_data,
        }
    }
}
