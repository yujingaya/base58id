pub const INVALID_VALUE: u8 = 255;

#[rustfmt::skip]
pub const ENCODE: &[u8; 58] = &[
    49, // input 0 (0x0) => '1' (0x31)
    50, // input 1 (0x1) => '2' (0x32)
    51, // input 2 (0x2) => '3' (0x33)
    52, // input 3 (0x3) => '4' (0x34)
    53, // input 4 (0x4) => '5' (0x35)
    54, // input 5 (0x5) => '6' (0x36)
    55, // input 6 (0x6) => '7' (0x37)
    56, // input 7 (0x7) => '8' (0x38)
    57, // input 8 (0x8) => '9' (0x39)
    65, // input 9 (0x9) => 'A' (0x41)
    66, // input 10 (0xA) => 'B' (0x42)
    67, // input 11 (0xB) => 'C' (0x43)
    68, // input 12 (0xC) => 'D' (0x44)
    69, // input 13 (0xD) => 'E' (0x45)
    70, // input 14 (0xE) => 'F' (0x46)
    71, // input 15 (0xF) => 'G' (0x47)
    72, // input 16 (0x10) => 'H' (0x48)
    74, // input 17 (0x11) => 'J' (0x4A)
    75, // input 18 (0x12) => 'K' (0x4B)
    76, // input 19 (0x13) => 'L' (0x4C)
    77, // input 20 (0x14) => 'M' (0x4D)
    78, // input 21 (0x15) => 'N' (0x4E)
    80, // input 22 (0x16) => 'P' (0x50)
    81, // input 23 (0x17) => 'Q' (0x51)
    82, // input 24 (0x18) => 'R' (0x52)
    83, // input 25 (0x19) => 'S' (0x53)
    84, // input 26 (0x1A) => 'T' (0x54)
    85, // input 27 (0x1B) => 'U' (0x55)
    86, // input 28 (0x1C) => 'V' (0x56)
    87, // input 29 (0x1D) => 'W' (0x57)
    88, // input 30 (0x1E) => 'X' (0x58)
    89, // input 31 (0x1F) => 'Y' (0x59)
    90, // input 32 (0x20) => 'Z' (0x5A)
    97, // input 33 (0x21) => 'a' (0x61)
    98, // input 34 (0x22) => 'b' (0x62)
    99, // input 35 (0x23) => 'c' (0x63)
    100, // input 36 (0x24) => 'd' (0x64)
    101, // input 37 (0x25) => 'e' (0x65)
    102, // input 38 (0x26) => 'f' (0x66)
    103, // input 39 (0x27) => 'g' (0x67)
    104, // input 40 (0x28) => 'h' (0x68)
    105, // input 41 (0x29) => 'i' (0x69)
    106, // input 42 (0x2A) => 'j' (0x6A)
    107, // input 43 (0x2B) => 'k' (0x6B)
    109, // input 44 (0x2C) => 'm' (0x6D)
    110, // input 45 (0x2D) => 'n' (0x6E)
    111, // input 46 (0x2E) => 'o' (0x6F)
    112, // input 47 (0x2F) => 'p' (0x60)
    113, // input 48 (0x30) => 'q' (0x71)
    114, // input 49 (0x31) => 'r' (0x72)
    115, // input 50 (0x32) => 's' (0x73)
    116, // input 51 (0x33) => 't' (0x74)
    117, // input 52 (0x34) => 'u' (0x75)
    118, // input 53 (0x35) => 'v' (0x76)
    119, // input 54 (0x36) => 'w' (0x77)
    120, // input 55 (0x37) => 'x' (0x78)
    121, // input 56 (0x38) => 'y' (0x79)
    122, // input 57 (0x39) => 'z' (0x7A)
];

#[rustfmt::skip]
pub const DECODE: &[u8; 256] = &[
	INVALID_VALUE, // input 0 (0x0)
    INVALID_VALUE, // input 1 (0x1)
    INVALID_VALUE, // input 2 (0x2)
    INVALID_VALUE, // input 3 (0x3)
    INVALID_VALUE, // input 4 (0x4)
    INVALID_VALUE, // input 5 (0x5)
    INVALID_VALUE, // input 6 (0x6)
    INVALID_VALUE, // input 7 (0x7)
    INVALID_VALUE, // input 8 (0x8)
    INVALID_VALUE, // input 9 (0x9)
    INVALID_VALUE, // input 10 (0xA)
    INVALID_VALUE, // input 11 (0xB)
    INVALID_VALUE, // input 12 (0xC)
    INVALID_VALUE, // input 13 (0xD)
    INVALID_VALUE, // input 14 (0xE)
    INVALID_VALUE, // input 15 (0xF)
	INVALID_VALUE, // input 16 (0x10)
    INVALID_VALUE, // input 17 (0x11)
    INVALID_VALUE, // input 18 (0x12)
    INVALID_VALUE, // input 19 (0x13)
    INVALID_VALUE, // input 20 (0x14)
    INVALID_VALUE, // input 21 (0x15)
    INVALID_VALUE, // input 22 (0x16)
    INVALID_VALUE, // input 23 (0x17)
    INVALID_VALUE, // input 24 (0x18)
    INVALID_VALUE, // input 25 (0x19)
    INVALID_VALUE, // input 26 (0x1A)
    INVALID_VALUE, // input 27 (0x1B)
    INVALID_VALUE, // input 28 (0x1C)
    INVALID_VALUE, // input 29 (0x1D)
    INVALID_VALUE, // input 30 (0x1E)
    INVALID_VALUE, // input 31 (0x1F)
	INVALID_VALUE, // input 32 (0x20)
    INVALID_VALUE, // input 33 (0x21)
    INVALID_VALUE, // input 34 (0x22)
    INVALID_VALUE, // input 35 (0x23)
    INVALID_VALUE, // input 36 (0x24)
    INVALID_VALUE, // input 37 (0x25)
    INVALID_VALUE, // input 38 (0x26)
    INVALID_VALUE, // input 39 (0x27)
    INVALID_VALUE, // input 40 (0x28)
    INVALID_VALUE, // input 41 (0x29)
    INVALID_VALUE, // input 42 (0x2A)
    INVALID_VALUE, // input 43 (0x2B)
    INVALID_VALUE, // input 44 (0x2C)
    INVALID_VALUE, // input 45 (0x2D)
    INVALID_VALUE, // input 46 (0x2E)
    INVALID_VALUE, // input 47 (0x2F)
	INVALID_VALUE, // input 48 (0x30)
    0, // input 49 (0x31 char '1') => 0 (0x0)
    1, // input 50 (0x32 char '2') => 1 (0x1)
    2, // input 51 (0x33 char '3') => 2 (0x2)
    3, // input 52 (0x34 char '4') => 3 (0x3)
    4, // input 53 (0x35 char '5') => 4 (0x4)
    5, // input 54 (0x36 char '6') => 5 (0x5)
    6, // input 55 (0x37 char '7') => 6 (0x6)
    7, // input 56 (0x38 char '8') => 7 (0x7)
    8, // input 57 (0x39 char '9') => 8 (0x8)
    INVALID_VALUE, // input 58 (0x3A)
    INVALID_VALUE, // input 59 (0x3B)
    INVALID_VALUE, // input 60 (0x3C)
    INVALID_VALUE, // input 61 (0x3D)
    INVALID_VALUE, // input 62 (0x3E)
    INVALID_VALUE, // input 63 (0x3F)
	INVALID_VALUE, // input 64 (0x40)
    9, // input 65 (0x41 char 'A') => 9 (0x9)
    10, // input 66 (0x42 char 'B') => 10 (0xA)
    11, // input 67 (0x43 char 'C') => 11 (0xB)
    12, // input 68 (0x44 char 'D') => 12 (0xC)
    13, // input 69 (0x45 char 'E') => 13 (0xD)
    14, // input 70 (0x46 char 'F') => 14 (0xE)
    15, // input 71 (0x47 char 'G') => 15 (0xF)
    16, // input 72 (0x48 char 'H') => 16 (0x10)
    INVALID_VALUE, // input 73 (0x49)
    17, // input 74 (0x4A char 'J') => 17 (0x11)
    18, // input 75 (0x4B char 'K') => 18 (0x12)
    19, // input 76 (0x4C char 'L') => 19 (0x13)
    20, // input 77 (0x4D char 'M') => 20 (0x14)
    21, // input 78 (0x4E char 'N') => 21 (0x15)
    INVALID_VALUE, // input 79 (0x4F)
	22, // input 80 (0x50 char 'P') => 22 (0x16)
    23, // input 81 (0x51 char 'Q') => 23 (0x17)
    24, // input 82 (0x52 char 'R') => 24 (0x18)
    25, // input 83 (0x53 char 'S') => 25 (0x19)
    26, // input 84 (0x54 char 'T') => 26 (0x1A)
    27, // input 85 (0x55 char 'U') => 27 (0x1B)
    28, // input 86 (0x56 char 'V') => 28 (0x1C)
    29, // input 87 (0x57 char 'W') => 29 (0x1D)
    30, // input 88 (0x58 char 'X') => 30 (0x1E)
    31, // input 89 (0x59 char 'Y') => 31 (0x1F)
    32, // input 90 (0x5A char 'Z') => 32 (0x20)
    INVALID_VALUE, // input 91 (0x5B)
    INVALID_VALUE, // input 92 (0x5C)
    INVALID_VALUE, // input 93 (0x5D)
    INVALID_VALUE, // input 94 (0x5E)
    INVALID_VALUE, // input 95 (0x5F)
	INVALID_VALUE, // input 96 (0x60)
    33, // input 97 (0x61 char 'a') => 33 (0x21)
    34, // input 98 (0x62 char 'b') => 34 (0x22)
    35, // input 99 (0x63 char 'c') => 35 (0x23)
    36, // input 100 (0x64 char 'd') => 36 (0x24)
    37, // input 101 (0x65 char 'e') => 37 (0x25)
    38, // input 102 (0x66 char 'f') => 38 (0x26)
    39, // input 103 (0x67 char 'g') => 39 (0x27)
    40, // input 104 (0x68 char 'h') => 40 (0x28)
    41, // input 105 (0x69 char 'i') => 41 (0x29)
    42, // input 106 (0x6A char 'j') => 42 (0x2A)
    43, // input 107 (0x6B char 'k') => 43 (0x2B)
    INVALID_VALUE, // input 108 (0x6C)
    44, // input 109 (0x6D char 'm') => 44 (0x2C)
    45, // input 110 (0x6E char 'n') => 45 (0x2D)
    46, // input 111 (0x6F char 'o') => 46 (0x2E)
	47, // input 112 (0x70 char 'p') => 47 (0x2F)
    48, // input 113 (0x71 char 'q') => 48 (0x30)
    49, // input 114 (0x72 char 'r') => 49 (0x31)
    50, // input 115 (0x73 char 's') => 50 (0x32)
    51, // input 116 (0x74 char 't') => 51 (0x33)
    52, // input 117 (0x75 char 'u') => 52 (0x34)
    53, // input 118 (0x76 char 'v') => 53 (0x35)
    54, // input 119 (0x77 char 'w') => 54 (0x36)
    55, // input 120 (0x78 char 'x') => 55 (0x37)
    56, // input 121 (0x79 char 'y') => 56 (0x38)
    57, // input 122 (0x7A char 'z') => 57 (0x39)
    INVALID_VALUE, // input 123 (0x7B)
    INVALID_VALUE, // input 124 (0x7C)
    INVALID_VALUE, // input 125 (0x7D)
    INVALID_VALUE, // input 126 (0x7E)
    INVALID_VALUE, // input 127 (0x7F)
    INVALID_VALUE, // input 128 (0x80)
    INVALID_VALUE, // input 129 (0x81)
    INVALID_VALUE, // input 130 (0x82)
    INVALID_VALUE, // input 131 (0x83)
    INVALID_VALUE, // input 132 (0x84)
    INVALID_VALUE, // input 133 (0x85)
    INVALID_VALUE, // input 134 (0x86)
    INVALID_VALUE, // input 135 (0x87)
    INVALID_VALUE, // input 136 (0x88)
    INVALID_VALUE, // input 137 (0x89)
    INVALID_VALUE, // input 138 (0x8A)
    INVALID_VALUE, // input 139 (0x8B)
    INVALID_VALUE, // input 140 (0x8C)
    INVALID_VALUE, // input 141 (0x8D)
    INVALID_VALUE, // input 142 (0x8E)
    INVALID_VALUE, // input 143 (0x8F)
    INVALID_VALUE, // input 144 (0x90)
    INVALID_VALUE, // input 145 (0x91)
    INVALID_VALUE, // input 146 (0x92)
    INVALID_VALUE, // input 147 (0x93)
    INVALID_VALUE, // input 148 (0x94)
    INVALID_VALUE, // input 149 (0x95)
    INVALID_VALUE, // input 150 (0x96)
    INVALID_VALUE, // input 151 (0x97)
    INVALID_VALUE, // input 152 (0x98)
    INVALID_VALUE, // input 153 (0x99)
    INVALID_VALUE, // input 154 (0x9A)
    INVALID_VALUE, // input 155 (0x9B)
    INVALID_VALUE, // input 156 (0x9C)
    INVALID_VALUE, // input 157 (0x9D)
    INVALID_VALUE, // input 158 (0x9E)
    INVALID_VALUE, // input 159 (0x9F)
    INVALID_VALUE, // input 160 (0xA0)
    INVALID_VALUE, // input 161 (0xA1)
    INVALID_VALUE, // input 162 (0xA2)
    INVALID_VALUE, // input 163 (0xA3)
    INVALID_VALUE, // input 164 (0xA4)
    INVALID_VALUE, // input 165 (0xA5)
    INVALID_VALUE, // input 166 (0xA6)
    INVALID_VALUE, // input 167 (0xA7)
    INVALID_VALUE, // input 168 (0xA8)
    INVALID_VALUE, // input 169 (0xA9)
    INVALID_VALUE, // input 170 (0xAA)
    INVALID_VALUE, // input 171 (0xAB)
    INVALID_VALUE, // input 172 (0xAC)
    INVALID_VALUE, // input 173 (0xAD)
    INVALID_VALUE, // input 174 (0xAE)
    INVALID_VALUE, // input 175 (0xAF)
    INVALID_VALUE, // input 176 (0xB0)
    INVALID_VALUE, // input 177 (0xB1)
    INVALID_VALUE, // input 178 (0xB2)
    INVALID_VALUE, // input 179 (0xB3)
    INVALID_VALUE, // input 180 (0xB4)
    INVALID_VALUE, // input 181 (0xB5)
    INVALID_VALUE, // input 182 (0xB6)
    INVALID_VALUE, // input 183 (0xB7)
    INVALID_VALUE, // input 184 (0xB8)
    INVALID_VALUE, // input 185 (0xB9)
    INVALID_VALUE, // input 186 (0xBA)
    INVALID_VALUE, // input 187 (0xBB)
    INVALID_VALUE, // input 188 (0xBC)
    INVALID_VALUE, // input 189 (0xBD)
    INVALID_VALUE, // input 190 (0xBE)
    INVALID_VALUE, // input 191 (0xBF)
    INVALID_VALUE, // input 192 (0xC0)
    INVALID_VALUE, // input 193 (0xC1)
    INVALID_VALUE, // input 194 (0xC2)
    INVALID_VALUE, // input 195 (0xC3)
    INVALID_VALUE, // input 196 (0xC4)
    INVALID_VALUE, // input 197 (0xC5)
    INVALID_VALUE, // input 198 (0xC6)
    INVALID_VALUE, // input 199 (0xC7)
    INVALID_VALUE, // input 200 (0xC8)
    INVALID_VALUE, // input 201 (0xC9)
    INVALID_VALUE, // input 202 (0xCA)
    INVALID_VALUE, // input 203 (0xCB)
    INVALID_VALUE, // input 204 (0xCC)
    INVALID_VALUE, // input 205 (0xCD)
    INVALID_VALUE, // input 206 (0xCE)
    INVALID_VALUE, // input 207 (0xCF)
    INVALID_VALUE, // input 208 (0xD0)
    INVALID_VALUE, // input 209 (0xD1)
    INVALID_VALUE, // input 210 (0xD2)
    INVALID_VALUE, // input 211 (0xD3)
    INVALID_VALUE, // input 212 (0xD4)
    INVALID_VALUE, // input 213 (0xD5)
    INVALID_VALUE, // input 214 (0xD6)
    INVALID_VALUE, // input 215 (0xD7)
    INVALID_VALUE, // input 216 (0xD8)
    INVALID_VALUE, // input 217 (0xD9)
    INVALID_VALUE, // input 218 (0xDA)
    INVALID_VALUE, // input 219 (0xDB)
    INVALID_VALUE, // input 220 (0xDC)
    INVALID_VALUE, // input 221 (0xDD)
    INVALID_VALUE, // input 222 (0xDE)
    INVALID_VALUE, // input 223 (0xDF)
    INVALID_VALUE, // input 224 (0xE0)
    INVALID_VALUE, // input 225 (0xE1)
    INVALID_VALUE, // input 226 (0xE2)
    INVALID_VALUE, // input 227 (0xE3)
    INVALID_VALUE, // input 228 (0xE4)
    INVALID_VALUE, // input 229 (0xE5)
    INVALID_VALUE, // input 230 (0xE6)
    INVALID_VALUE, // input 231 (0xE7)
    INVALID_VALUE, // input 232 (0xE8)
    INVALID_VALUE, // input 233 (0xE9)
    INVALID_VALUE, // input 234 (0xEA)
    INVALID_VALUE, // input 235 (0xEB)
    INVALID_VALUE, // input 236 (0xEC)
    INVALID_VALUE, // input 237 (0xED)
    INVALID_VALUE, // input 238 (0xEE)
    INVALID_VALUE, // input 239 (0xEF)
    INVALID_VALUE, // input 240 (0xF0)
    INVALID_VALUE, // input 241 (0xF1)
    INVALID_VALUE, // input 242 (0xF2)
    INVALID_VALUE, // input 243 (0xF3)
    INVALID_VALUE, // input 244 (0xF4)
    INVALID_VALUE, // input 245 (0xF5)
    INVALID_VALUE, // input 246 (0xF6)
    INVALID_VALUE, // input 247 (0xF7)
    INVALID_VALUE, // input 248 (0xF8)
    INVALID_VALUE, // input 249 (0xF9)
    INVALID_VALUE, // input 250 (0xFA)
    INVALID_VALUE, // input 251 (0xFB)
    INVALID_VALUE, // input 252 (0xFC)
    INVALID_VALUE, // input 253 (0xFD)
    INVALID_VALUE, // input 254 (0xFE)
    INVALID_VALUE, // input 255 (0xFF)
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse() {
        for (digit, &encoded) in ENCODE.iter().enumerate() {
            assert_eq!(DECODE[encoded as usize], digit as u8);
        }

        let mut counts = [0; 58];

        for (encoded, &digit) in DECODE.iter().enumerate() {
            if digit == INVALID_VALUE {
                continue;
            }

            counts[digit as usize] += 1;
            assert_eq!(ENCODE[digit as usize], encoded as u8);
        }

        assert!(counts.iter().all(|&count| count == 1));
    }
}
