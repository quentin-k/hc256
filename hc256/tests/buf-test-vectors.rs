use hc256::BufHc256;

#[test]
fn vector_1() {
    let mut k = [0; 32];
    let mut iv = [0; 32];
    let mut cipher = BufHc256::new(&mut k, &mut iv);
    let mut result: [u8; 32] = [0; 32];

    cipher.apply_stream(&mut result);
    assert_eq!(result, [0x5b, 0x07, 0x89, 0x85, 0xd8, 0xf6, 0xf3, 0x0d,
        0x42, 0xc5, 0xc0, 0x2f, 0xa6, 0xb6, 0x79, 0x51,
        0x53, 0xf0, 0x65, 0x34, 0x80, 0x1f, 0x89, 0xf2,
        0x4e, 0x74, 0x24, 0x8b, 0x72, 0x0b, 0x48, 0x18, ]);
}

#[test]
fn vector_2() {
    let mut k = [0; 32];
    let mut iv = [0; 32];
    iv[0] = 1;
    let mut cipher = BufHc256::new(&mut k, &mut iv);
    let mut result: [u8; 32] = [0; 32];

    cipher.apply_stream(&mut result);
    assert_eq!(result, [0xaf, 0xe2, 0xa2, 0xbf, 0x4f, 0x17, 0xce, 0xe9,
        0xfe, 0xc2, 0x05, 0x8b, 0xd1, 0xb1, 0x8b, 0xb1,
        0x5f, 0xc0, 0x42, 0xee, 0x71, 0x2b, 0x31, 0x01,
        0xdd, 0x50, 0x1f, 0xc6, 0x0b, 0x08, 0x2a, 0x50, ]);
}

#[test]
fn vector_3() {
    let mut k = [0; 32];
    let mut iv = [0; 32];
    k[0] = 0x55;
    let mut cipher = BufHc256::new(&mut k, &mut iv);
    let mut result: [u8; 32] = [0; 32];

    cipher.apply_stream(&mut result);
    assert_eq!(result, [0x1c, 0x40, 0x4a, 0xfe, 0x4f, 0xe2, 0x5f, 0xed,
        0x95, 0x8f, 0x9a, 0xd1, 0xae, 0x36, 0xc0, 0x6f,
        0x88, 0xa6, 0x5a, 0x3c, 0xc0, 0xab, 0xe2, 0x23,
        0xae, 0xb3, 0x90, 0x2f, 0x42, 0x0e, 0xd3, 0xa8, ]);
}

#[test]
fn split_vector_1() {
    let mut k = [0; 32];
    let mut iv = [0; 32];
    let mut cipher = BufHc256::new(&mut k, &mut iv);
    let mut a: [u8; 1] = [0; 1];
    let mut b: [u8; 12] = [0; 12];
    let mut c: [u8; 2] = [0; 2];
    let mut d: [u8; 3] = [0; 3];
    let mut e: [u8; 3] = [0; 3];
    let mut f: [u8; 2] = [0; 2];
    let mut g: [u8; 1] = [0; 1];
    let mut h: [u8; 1] = [0; 1];
    let mut i: [u8; 1] = [0; 1];
    let mut j: [u8; 3] = [0; 3];
    let mut k: [u8; 3] = [0; 3];

    cipher.apply_stream(&mut a);
    cipher.apply_stream(&mut b);
    cipher.apply_stream(&mut c);
    cipher.apply_stream(&mut d);
    cipher.apply_stream(&mut e);
    cipher.apply_stream(&mut f);
    cipher.apply_stream(&mut g);
    cipher.apply_stream(&mut h);
    cipher.apply_stream(&mut i);
    cipher.apply_stream(&mut j);
    cipher.apply_stream(&mut k);

    let result = [a[0], b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], c[0], c[1], d[0], d[1], d[2], e[0], e[1], e[2], f[0], f[1], g[0], h[0], i[0], j[0], j[1], j[2], k[0], k[1], k[2]];
    assert_eq!(result, [0x5b, 0x07, 0x89, 0x85, 0xd8, 0xf6, 0xf3, 0x0d,
        0x42, 0xc5, 0xc0, 0x2f, 0xa6, 0xb6, 0x79, 0x51,
        0x53, 0xf0, 0x65, 0x34, 0x80, 0x1f, 0x89, 0xf2,
        0x4e, 0x74, 0x24, 0x8b, 0x72, 0x0b, 0x48, 0x18, ]);
}

#[test]
fn split_vector_2() {
    let mut k = [0; 32];
    let mut iv = [0; 32];
    iv[0] = 1;
    let mut cipher = BufHc256::new(&mut k, &mut iv);

    let mut a: [u8; 1] = [0; 1];
    let mut b: [u8; 12] = [0; 12];
    let mut c: [u8; 2] = [0; 2];
    let mut d: [u8; 3] = [0; 3];
    let mut e: [u8; 3] = [0; 3];
    let mut f: [u8; 2] = [0; 2];
    let mut g: [u8; 1] = [0; 1];
    let mut h: [u8; 1] = [0; 1];
    let mut i: [u8; 1] = [0; 1];
    let mut j: [u8; 3] = [0; 3];
    let mut k: [u8; 3] = [0; 3];

    cipher.apply_stream(&mut a);
    cipher.apply_stream(&mut b);
    cipher.apply_stream(&mut c);
    cipher.apply_stream(&mut d);
    cipher.apply_stream(&mut e);
    cipher.apply_stream(&mut f);
    cipher.apply_stream(&mut g);
    cipher.apply_stream(&mut h);
    cipher.apply_stream(&mut i);
    cipher.apply_stream(&mut j);
    cipher.apply_stream(&mut k);

    let result = [a[0], b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], c[0], c[1], d[0], d[1], d[2], e[0], e[1], e[2], f[0], f[1], g[0], h[0], i[0], j[0], j[1], j[2], k[0], k[1], k[2]];
    assert_eq!(result, [0xaf, 0xe2, 0xa2, 0xbf, 0x4f, 0x17, 0xce, 0xe9,
        0xfe, 0xc2, 0x05, 0x8b, 0xd1, 0xb1, 0x8b, 0xb1,
        0x5f, 0xc0, 0x42, 0xee, 0x71, 0x2b, 0x31, 0x01,
        0xdd, 0x50, 0x1f, 0xc6, 0x0b, 0x08, 0x2a, 0x50, ]);
}

#[test]
fn split_vector_3() {
    let mut k = [0; 32];
    let mut iv = [0; 32];
    k[0] = 0x55;
    let mut cipher = BufHc256::new(&mut k, &mut iv);
    let mut a: [u8; 1] = [0; 1];
    let mut b: [u8; 12] = [0; 12];
    let mut c: [u8; 2] = [0; 2];
    let mut d: [u8; 3] = [0; 3];
    let mut e: [u8; 3] = [0; 3];
    let mut f: [u8; 2] = [0; 2];
    let mut g: [u8; 1] = [0; 1];
    let mut h: [u8; 1] = [0; 1];
    let mut i: [u8; 1] = [0; 1];
    let mut j: [u8; 3] = [0; 3];
    let mut k: [u8; 3] = [0; 3];

    cipher.apply_stream(&mut a);
    cipher.apply_stream(&mut b);
    cipher.apply_stream(&mut c);
    cipher.apply_stream(&mut d);
    cipher.apply_stream(&mut e);
    cipher.apply_stream(&mut f);
    cipher.apply_stream(&mut g);
    cipher.apply_stream(&mut h);
    cipher.apply_stream(&mut i);
    cipher.apply_stream(&mut j);
    cipher.apply_stream(&mut k);

    let result = [a[0], b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], c[0], c[1], d[0], d[1], d[2], e[0], e[1], e[2], f[0], f[1], g[0], h[0], i[0], j[0], j[1], j[2], k[0], k[1], k[2]];
    assert_eq!(result, [0x1c, 0x40, 0x4a, 0xfe, 0x4f, 0xe2, 0x5f, 0xed,
        0x95, 0x8f, 0x9a, 0xd1, 0xae, 0x36, 0xc0, 0x6f,
        0x88, 0xa6, 0x5a, 0x3c, 0xc0, 0xab, 0xe2, 0x23,
        0xae, 0xb3, 0x90, 0x2f, 0x42, 0x0e, 0xd3, 0xa8, ]);
}