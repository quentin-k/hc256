use super::*;

macro_rules! set_cipher_tables {
    ($var:ident, $key:ident, $iv:ident) => {
        let mut w: [u32; 2560] = [0; 2560];

        for i in 0..8 {
            w[i] = ($key[4 * i] as u32)
                | (($key[(4 * i) + 1] as u32) << 8)
                | (($key[(4 * i) + 2] as u32) << 16)
                | (($key[(4 * i) + 3] as u32) << 24);

            w[i + 8] = ($iv[4 * i] as u32)
                | (($iv[(4 * i) + 1] as u32) << 8)
                | (($iv[(4 * i) + 2] as u32) << 16)
                | (($iv[(4 * i) + 3] as u32) << 24);
        }

        for i in 16..2560 {
            w[i] = f2(w[i - 2])
                .wrapping_add(w[i - 7])
                .wrapping_add(f1(w[i - 15]))
                .wrapping_add(w[i - 16])
                .wrapping_add(i as u32);
        }

        $var.p[..1024].clone_from_slice(&w[512..(1024 + 512)]);
        $var.q[..1024].clone_from_slice(&w[1536..(1024 + 1536)]);

        w.zeroize();

        for _ in 0..4096 {
            $var.gen_word();
        }

        $var.i.zeroize();
    };
}

#[derive(Zeroize)]
#[zeroize(drop)]
pub struct BufHc256 {
    p: TABLE,
    q: TABLE,
    i: usize,
    r: [u8; 3],
    c: usize,
}

impl BufHc256 {
    pub fn new(k: &[u8; 32], iv: &[u8; 32]) -> Self {
        let mut cipher = BufHc256 { p: [0; 1024], q: [0; 1024], i: 0, r: [0; 3], c: 0 };
        set_cipher_tables!(cipher, k, iv);
        cipher
    }

    pub fn set_state(&mut self, k: &[u8; 32], iv: &[u8; 32], offset: usize) {
        set_cipher_tables!(self, k, iv);
        for _ in 0..(offset / 4) {
            self.gen_word();
        }
        let rem = offset % 4;
        if rem > 0 {
            let word = self.gen_word().to_le_bytes();

            match rem {
                3 => {
                    self.r = [0, 0, word[3]];
                    self.c = 1;
                }
                2 => {
                    self.r = [0, word[2], word[3]];
                    self.c = 2;
                }
                1 => {
                    self.r = [word[1], word[2], word[3]];
                    self.c = 3;
                }
                _ => panic!("Rem check failed!")
            }
        }
    }

    pub fn apply_stream(&mut self, dest: &mut [u8]) {
        let mut dlen = dest.len();

        let mut pad_i = 0;
        if self.c >= dlen {
            for i in (3 - self.c)..(3 - self.c + dlen) {
                dest[pad_i] ^= self.r[i];
                self.r[i] = 0;
                pad_i += 1;
            }
            self.c -= dlen;
            return;
        } else {
            for i in (3 - self.c)..3 {
                dest[pad_i] ^= self.r[i];
                self.r[i] = 0;
                pad_i += 1;
            }

            self.c = 0;
        }

        dlen -= pad_i;

        let mut ifull = dlen / 4;
        let mut pad = dlen % 4;
        for i in 0..ifull {
            let mut word: [u8; 4] = self.gen_word().to_le_bytes();

            let o = (i * 4) + pad_i;

            dest[o] ^= word[0];
            dest[o + 1] ^= word[1];
            dest[o + 2] ^= word[2];
            dest[o + 3] ^= word[3];

            word.zeroize();
        }
        if pad != 0 {
            let mut word: [u8; 4] = self.gen_word().to_le_bytes();
            let o = (ifull * 4) + pad_i;
            match pad {
                3 => {
                    dest[o] ^= word[0];
                    dest[o + 1] ^= word[1];
                    dest[o + 2] ^= word[2];
                    self.r = [0, 0, word[3]];
                    self.c = 1;
                }
                2 => {
                    dest[o] ^= word[0];
                    dest[o + 1] ^= word[1];
                    self.r = [0, word[2], word[3]];
                    self.c = 2;
                }
                1 => {
                    dest[o] ^= word[0];
                    self.r = [word[1], word[2], word[3]];
                    self.c = 3;
                }
                _ => panic!("Pad check failed!")
            }

            word.zeroize();
        } else {
            self.r.zeroize()
        }

        dlen.zeroize();
        ifull.zeroize();
        pad.zeroize();
    }

    #[inline]
    fn gen_word(&mut self) -> u32 {
        let i = self.i;
        let (j, j3, j10, j12, j1023) = self.offsets();

        self.i = (self.i + 1) & (2048 - 1);

        if i < 1024 {
            self.p[j] = self.p[j]
                .wrapping_add(self.p[j10])
                .wrapping_add(self.g1(
                    self.p[j3],
                    self.p[j1023],
                ));

            self.h1(self.p[j12]) ^ self.p[j]
        } else {
            self.q[j] = self.q[j]
                .wrapping_add(self.q[j10])
                .wrapping_add(self.g2(
                    self.q[j3],
                    self.q[j1023],
                ));

            self.h2(self.q[j12]) ^ self.q[j]
        }
    }
    #[inline]
    fn g1(&self, x: u32, y: u32) -> u32 {
        (x.rotate_right(10) ^ y.rotate_right(23)).wrapping_add(self.q[(x ^ y) as usize & 1023])
    }

    #[inline]
    fn g2(&self, x: u32, y: u32) -> u32 {
        (x.rotate_right(10) ^ y.rotate_right(23)).wrapping_add(self.p[(x ^ y) as usize & 1023])
    }

    #[inline]
    fn h1(&self, x: u32) -> u32 {
        self.q[(x & 0xff) as usize]
            .wrapping_add(self.q[256 + ((x >> 8) & 0xff) as usize])
            .wrapping_add(self.q[512 + ((x >> 16) & 0xff) as usize])
            .wrapping_add(self.q[768 + ((x >> 24) & 0xff) as usize])
    }

    #[inline]
    fn h2(&self, x: u32) -> u32 {
        self.p[(x & 0xff) as usize]
            .wrapping_add(self.p[256 + ((x >> 8) & 0xff) as usize])
            .wrapping_add(self.p[512 + ((x >> 16) & 0xff) as usize])
            .wrapping_add(self.p[768 + ((x >> 24) & 0xff) as usize])
    }

    #[inline]
    fn offsets(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.i & 1023,
            self.i.wrapping_sub(3) & 1023,
            self.i.wrapping_sub(10) & 1023,
            self.i.wrapping_sub(12) & 1023,
            self.i.wrapping_add(1) & 1023
        )
    }
}