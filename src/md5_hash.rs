pub(crate) fn md5_hash(user_input: &str) {
    // Приведем в битовую последовательность сообщение
    let mut message_bytes = user_input.as_bytes().to_vec();
    let message_len_bits: u64 = (user_input.len() * 8) as u64;
    // Добавим единичный бит
    message_bytes.push(0x80);
    // Добавим биты "0" до тех пор, пока длина сообщения не станет 448 по модулю 512
    while (message_bytes.len() * 8) % 512 != 448 {
        message_bytes.push(0x00);
    }
    // Добавим длину исходного сообщения (64 битное число)
    for byte in message_len_bits.to_le_bytes().iter() {
        message_bytes.push(*byte);
    }
    // Инициализация буфера
    let mut A: u32 = 0x67452301;
    let mut B: u32 = 0xEFCDAB89;
    let mut C: u32 = 0x98BADCFE;
    let mut D: u32 = 0x10325476;
    // Определим функции
    fn f(x: u32, y: u32, z: u32) -> u32 { (x & y) | (!x & z) }
    fn g(x: u32, y: u32, z: u32) -> u32 { (x & z) | (y & !z) }
    fn h(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }
    fn i(x: u32, y: u32, z: u32) -> u32 { y ^ (x | !z) }
    fn left_rotate(x: u32, c: u32) -> u32 {
        (x << c) | (x >> (32 - c))
    }
    // Определим таблицу констант T[i]=2^32 * ∣sin(i)∣
    let t_constants: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x2441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x4881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];
    let shifts: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, // Раунд 1
        5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,     // Раунд 2
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, // Раунд 3
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21  // Раунд 4
    ];


    // Внутри этого цикла мы обрабатываем каждый 64-байтовый (512-битный) блок.
    for chunk in message_bytes.chunks_exact(64) {
        // Создаем массив из 16-ти 32-битных слов
        let mut w = [0u32; 16];
        for (i, word) in w.iter_mut().enumerate() {
            *word = u32::from_le_bytes([chunk[4 * i], chunk[4 * i + 1], chunk[4 * i + 2], chunk[4 * i + 3]]);
        }

        let mut a = A;
        let mut b = B;
        let mut c = C;
        let mut d = D;
        for j in 0..64 {
            let (f, g) = match j {
                0..=15 => (f(b, c, d), j),
                16..=31 => (g(b, c, d), (5 * j + 1) % 16),
                32..=47 => (h(b, c, d), (3 * j + 5) % 16),
                48..=63 => (i(b, c, d), (7 * j) % 16),
                _ => unreachable!(),
            };

            let temp = b.wrapping_add(left_rotate(a.wrapping_add(f)
                                                      .wrapping_add(t_constants[j])
                                                      .wrapping_add(w[g]),
                                                  shifts[j]));
            a = d;
            d = c;
            c = b;
            b = temp;
        }
        A = A.wrapping_add(a);
        B = B.wrapping_add(b);
        C = C.wrapping_add(c);
        D = D.wrapping_add(d);
    }
    let digest = [A.to_le_bytes(), B.to_le_bytes(), C.to_le_bytes(), D.to_le_bytes()].concat();
    let digest_string = digest.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
    println!("MD5 hash: {:02x?}", digest_string);

    let lib_digest = md5::compute(user_input);
    println!("MD5 lib hash: {:02x?}", lib_digest);
}
