use std::collections::HashMap;

fn build_lut(lengths: &[u8], symbols: &[u8]) -> HashMap<i32, u8> {
    let mut codes = HashMap::new();
    let mut n = 0;
    let mut idx = 0;
    let mut last = 0;
    let mut code = -1;

    for x in lengths.iter().map(|it| *it as usize) {
        n += 1;
        if x > 0 {
            code = (code + 1) * (1i32 << (n - last));
            last = n;
            for i in 0..x {
                if i > 0 { code += 1; }
                let key = code + (1i32 << n);
                let symbol = symbols[idx];
                codes.insert(key, symbol);
                idx += 1;
            }
        }
    }

    return codes;
}

fn output_symbol(symbol: u8, colors: &[(u8, u8, u8)], chars: &[u16]) {
    let first_char = (colors.len() * 2) as u8;
    if symbol < first_char {
        let attr = if (symbol & 1) == 1 { 38 } else { 48 };
        let (r, g, b) = colors[(symbol >> 1) as usize];
        print!("\x1b[{};2;{};{};{}m", attr, r, g, b);
    } else if symbol == 254 {
        print!("\x1b[0m\n");
    } else {
        let cc = chars[(symbol - first_char) as usize];
        print!("{}", char::from_u32(cc.into()).unwrap());
    }
}

pub fn egg() {
    let colors: &[(u8, u8, u8)] = &[
        (1, 0, 5), (1, 1, 10), (4, 6, 18), (4, 9, 25), (4, 21, 10), (5, 8, 20), (5, 10, 39),
        (5, 34, 26), (6, 7, 26), (6, 12, 56), (6, 20, 22), (7, 6, 7), (8, 9, 20), (9, 6, 18),
        (13, 41, 48), (14, 19, 45), (15, 19, 75), (16, 136, 202), (21, 14, 10), (23, 4, 7),
        (25, 27, 105), (26, 106, 155), (28, 35, 86), (28, 35, 116), (30, 37, 130), (31, 39, 135),
        (33, 41, 125), (35, 42, 131), (35, 42, 139), (37, 42, 132), (37, 44, 147), (38, 45, 152),
        (38, 83, 100), (38, 127, 200), (38, 142, 204), (39, 42, 121), (41, 45, 139), (41, 46, 146),
        (41, 46, 153), (43, 44, 90), (44, 40, 45), (44, 46, 110), (44, 49, 169), (44, 50, 132),
        (44, 50, 141), (45, 8, 9), (45, 50, 148), (45, 50, 155), (45, 106, 148), (46, 50, 161),
        (48, 51, 162), (48, 52, 149), (48, 52, 153), (49, 51, 134), (50, 139, 169), (56, 162, 224),
        (75, 171, 218), (75, 178, 227), (77, 170, 207), (78, 15, 16), (79, 149, 173), (81, 44, 20),
        (82, 42, 39), (82, 165, 187), (83, 92, 105), (96, 120, 134), (101, 128, 125), (101, 138, 157),
        (103, 182, 201), (110, 15, 19), (110, 41, 23), (114, 45, 39), (118, 91, 14), (121, 139, 149),
        (129, 16, 20), (130, 51, 35), (137, 45, 26), (137, 147, 170), (138, 43, 35), (140, 140, 152),
        (143, 149, 193), (144, 71, 72), (146, 124, 155), (147, 37, 36), (147, 43, 33), (149, 42, 29),
        (151, 36, 28), (153, 173, 203), (154, 42, 30), (154, 167, 182), (156, 43, 33), (158, 108, 24),
        (160, 43, 33), (162, 42, 31), (166, 163, 176), (170, 185, 199), (173, 197, 213), (176, 185, 209),
        (178, 205, 230), (182, 195, 215), (183, 137, 44), (212, 148, 40), (216, 147, 20), (232, 158, 13),
        (239, 168, 8),
    ];

    let chars: &[u16] = &[
        32, 9146, 9147, 9148, 9149, 9472, 9473, 9474, 9475, 9487, 9491, 9495, 9499, 9507, 9515,
        9531, 9588, 9589, 9590, 9592, 9594, 9601, 9602, 9603, 9604, 9605, 9606, 9607, 9610, 9612,
        9614, 9622, 9623, 9624, 9625, 9626, 9627, 9628, 9629, 9630, 9631,
    ];

    let lengths: &[u8] = & [0, 0, 0, 1, 7, 14, 24, 34, 56, 45, 44, 19, 2];

    let symbols: &[u8] = &[
        210, 1, 10, 11, 231, 232, 234, 236, 0, 6, 7, 24, 25, 92, 181, 233, 235, 237, 238, 239, 240,
        242, 2, 3, 4, 5, 13, 17, 52, 55, 72, 73, 93, 94, 95, 98, 99, 101, 102, 103, 104, 105, 180,
        241, 243, 248, 12, 19, 20, 22, 26, 27, 31, 33, 48, 53, 56, 57, 58, 65, 74, 75, 79, 82, 83,
        88, 89, 100, 138, 168, 176, 185, 216, 219, 220, 221, 227, 244, 250, 254, 9, 16, 18, 21,
        29, 32, 41, 44, 45, 49, 50, 51, 54, 59, 60, 61, 63, 64, 70, 71, 76, 77, 81, 86, 87, 91,
        97, 116, 118, 119, 121, 125, 129, 131, 139, 143, 147, 152, 154, 155, 156, 157, 169, 170,
        171, 177, 184, 191, 194, 198, 208, 215, 218, 222, 229, 246, 8, 14, 23, 28, 30, 36, 38, 39,
        42, 62, 80, 90, 106, 107, 109, 112, 115, 120, 123, 126, 128, 141, 142, 148, 149, 151, 161,
        165, 167, 174, 179, 190, 193, 196, 197, 199, 201, 203, 205, 207, 213, 217, 225, 245, 247,
        15, 34, 35, 37, 40, 43, 46, 47, 66, 68, 78, 85, 96, 110, 111, 113, 117, 127, 130, 132, 134,
        135, 140, 145, 146, 150, 153, 158, 159, 164, 166, 173, 175, 178, 183, 192, 200, 202, 204,
        206, 211, 223, 226, 230, 108, 114, 122, 124, 133, 137, 144, 163, 172, 182, 187, 188, 189,
        195, 209, 212, 214, 224, 249, 228, 255,
    ];

    let compressed: &[u8] = &[
        185, 140, 201, 238, 99, 30, 57, 217, 142, 120, 241, 143, 30, 51, 49, 143, 115, 27, 224,
        185, 140, 123, 150, 99, 198, 60, 120, 199, 142, 123, 152, 197, 143, 27, 50, 230, 49, 163,
        236, 25, 142, 212, 120, 217, 241, 227, 31, 99, 176, 62, 198, 51, 49, 227, 51, 30, 51, 49,
        227, 124, 17, 227, 30, 60, 96, 143, 24, 209, 205, 30, 173, 201, 174, 99, 55, 183, 140, 93,
        188, 98, 237, 227, 22, 222, 51, 246, 49, 139, 172, 140, 91, 152, 200, 185, 141, 161, 30,
        49, 227, 191, 177, 57, 227, 181, 30, 49, 207, 28, 241, 238, 26, 208, 142, 212, 125, 131,
        87, 119, 7, 215, 213, 143, 113, 153, 173, 184, 107, 91, 112, 211, 87, 7, 184, 62, 180, 231,
        62, 180, 250, 251, 131, 220, 31, 95, 112, 141, 125, 193, 245, 231, 215, 220, 53, 112, 125,
        125, 194, 53, 247, 5, 214, 220, 51, 173, 184, 207, 214, 220, 53, 112, 214, 182, 224, 250,
        55, 123, 3, 26, 236, 215, 123, 6, 100, 159, 99, 120, 125, 139, 59, 27, 195, 236, 15, 176,
        60, 155, 198, 164, 236, 15, 177, 60, 155, 198, 100, 150, 77, 225, 246, 59, 3, 201, 188,
        60, 146, 201, 188, 52, 155, 195, 201, 188, 60, 150, 164, 222, 30, 77, 225, 181, 87, 129,
        99, 120, 88, 177, 141, 6, 49, 224, 198, 104, 91, 100, 11, 231, 35, 231, 32, 90, 180, 106,
        219, 64, 181, 104, 23, 212, 69, 188, 10, 194, 211, 55, 7, 86, 136, 56, 45, 65, 140, 120,
        145, 141, 129, 176, 13, 142, 193, 248, 247, 141, 108, 111, 30, 216, 222, 103, 179, 119,
        120, 89, 39, 187, 188, 45, 221, 225, 110, 238, 187, 23, 119, 85, 239, 238, 143, 119, 174,
        62, 207, 92, 82, 221, 221, 63, 179, 215, 30, 238, 233, 155, 187, 166, 89, 187, 186, 102,
        238, 232, 251, 50, 236, 238, 141, 119, 174, 107, 103, 116, 109, 153, 118, 119, 71, 102,
        238, 232, 247, 122, 228, 107, 179, 238, 245, 199, 187, 215, 30, 239, 92, 125, 27, 249, 7,
        191, 60, 153, 7, 147, 32, 242, 100, 30, 76, 131, 158, 76, 131, 200, 60, 153, 8, 147, 32,
        247, 242, 15, 32, 247, 231, 62, 36, 138, 178, 100, 103, 226, 30, 254, 67, 55, 231, 196,
        144, 123, 249, 8, 183, 144, 17, 36, 29, 77, 179, 80, 245, 45, 138, 18, 32, 165, 90, 10, 89,
        130, 0, 23, 255, 190, 1, 0, 146, 56, 146, 1, 0, 0, 0, 146, 114, 10, 78, 193, 72, 53, 137,
        4, 28, 19, 219, 108, 3, 111, 120, 18, 100, 59, 38, 70, 132, 155, 164, 73, 144, 91, 249, 15,
        223, 236, 180, 46, 145, 127, 124, 139, 251, 226, 200, 53, 254, 200, 213, 111, 246, 65, 127,
        124, 27, 59, 231, 175, 246, 79, 108, 205, 180, 217, 63, 180, 190, 127, 105, 178, 62, 211,
        100, 125, 166, 201, 173, 161, 247, 62, 13, 93, 167, 130, 141, 146, 54, 155, 38, 182, 77,
        52, 214, 207, 100, 213, 209, 244, 105, 174, 252, 197, 207, 41, 115, 223, 34, 85, 242, 37,
        95, 30, 81, 165, 97, 158, 86, 25, 240, 207, 42, 248, 235, 179, 41, 154, 43, 159, 13, 252,
        74, 25, 180, 112, 207, 43, 12, 248, 101, 148, 186, 38, 46, 31, 71, 14, 190, 243, 76, 214,
        6, 151, 183, 7, 82, 136, 105, 0, 146, 1, 36, 132, 144, 36, 128, 64, 32, 18, 72, 18, 69, 16,
        0, 0, 0, 0, 146, 68, 6, 224, 103, 137, 38, 181, 144, 2, 38, 218, 174, 222, 64, 74, 145, 95,
        19, 13, 187, 249, 8, 191, 144, 220, 171, 230, 77, 127, 125, 155, 127, 124, 141, 205, 246,
        84, 171, 226, 238, 111, 153, 217, 223, 118, 111, 239, 145, 127, 184, 46, 228, 251, 159, 4,
        251, 131, 238, 119, 8, 153, 184, 57, 247, 50, 242, 166, 110, 13, 184, 44, 205, 193, 247, 7,
        153, 184, 49, 166, 110, 31, 153, 184, 171, 185, 240, 80, 141, 25, 148, 59, 114, 165, 158,
        157, 2, 209, 61, 58, 1, 78, 135, 94, 157, 14, 189, 58, 8, 162, 186, 40, 210, 61, 21, 209,
        64, 210, 171, 209, 160, 118, 104, 208, 255, 234, 52, 25, 92, 178, 168, 127, 74, 43, 150,
        197, 119, 225, 237, 143, 98, 144, 9, 36, 9, 36, 36, 129, 26, 10, 65, 128, 144, 126, 100,
        143, 211, 63, 77, 158, 154, 70, 228, 23, 116, 144, 8, 240, 115, 132, 113, 0, 0, 0, 67, 8,
        41, 127, 76, 8, 25, 118, 240, 43, 219, 237, 158, 183, 144, 22, 50, 42, 201, 195, 44, 165,
        221, 148, 185, 229, 47, 90, 98, 245, 229, 46, 121, 151, 199, 92, 198, 152, 252, 197, 217,
        92, 243, 37, 158, 100, 179, 30, 89, 231, 30, 89, 231, 77, 235, 204, 173, 58, 111, 95, 175,
        215, 157, 55, 175, 58, 105, 230, 232, 78, 151, 159, 62, 89, 231, 75, 171, 163, 78, 147,
        180, 232, 22, 157, 35, 83, 102, 157, 38, 169, 208, 61, 51, 211, 164, 205, 58, 14, 208, 45,
        58, 65, 76, 180, 66, 137, 139, 70, 129, 104, 208, 45, 19, 106, 176, 194, 46, 236, 246, 41,
        4, 20, 128, 73, 23, 1, 32, 255, 3, 126, 104, 7, 245, 172, 154, 221, 127, 185, 181, 30, 185,
        180, 254, 185, 181, 29, 67, 106, 58, 165, 189, 234, 22, 247, 215, 13, 71, 174, 110, 157,
        155, 94, 181, 153, 126, 82, 64, 36, 130, 8, 8, 4, 2, 55, 116, 27, 160, 229, 170, 126, 95,
        115, 87, 186, 73, 236, 11, 74, 241, 126, 118, 132, 76, 23, 119, 152, 111, 74, 92, 180, 87,
        126, 85, 14, 196, 170, 78, 202, 164, 89, 82, 203, 42, 145, 102, 4, 183, 39, 4, 195, 78,
        150, 89, 197, 156, 89, 211, 75, 57, 153, 243, 89, 158, 252, 215, 231, 77, 122, 113, 167,
        205, 102, 114, 39, 205, 44, 236, 87, 39, 226, 162, 126, 46, 92, 252, 82, 232, 211, 165, 88,
        65, 78, 147, 52, 233, 30, 141, 42, 194, 164, 1, 78, 150, 117, 58, 78, 83, 164, 0, 20, 95,
        149, 64, 244, 79, 188, 160, 26, 170, 5, 183, 92, 184, 80, 171, 65, 72, 4, 145, 196, 126,
        233, 32, 226, 36, 2, 243, 27, 211, 164, 16, 82, 8, 41, 0, 160, 34, 10, 65, 5, 32, 20, 4,
        105, 210, 1, 121, 141, 241, 18, 14, 36, 0, 131, 202, 107, 144, 175, 199, 200, 179, 171,
        221, 36, 81, 36, 2, 1, 37, 149, 128, 144, 114, 224, 104, 114, 253, 80, 228, 119, 15, 10,
        3, 162, 128, 141, 212, 7, 245, 155, 186, 216, 139, 189, 69, 116, 81, 92, 244, 232, 61, 70,
        134, 93, 58, 15, 206, 160, 122, 52, 28, 167, 45, 218, 103, 153, 44, 231, 153, 53, 217, 225,
        58, 105, 231, 205, 206, 9, 230, 159, 52, 243, 177, 93, 159, 52, 248, 167, 60, 252, 80, 16,
        79, 154, 232, 130, 121, 231, 226, 187, 163, 18, 145, 172, 105, 27, 121, 72, 38, 80, 171,
        62, 150, 109, 26, 89, 242, 169, 4, 170, 15, 211, 161, 255, 146, 168, 22, 141, 32, 163, 75,
        42, 101, 3, 98, 47, 155, 110, 187, 241, 62, 131, 100, 211, 55, 17, 32, 20, 6, 224, 54, 36,
        155, 18, 64, 47, 229, 7, 178, 32, 18, 77, 164, 219, 109, 164, 219, 112, 27, 110, 10, 65,
        190, 128, 111, 25, 38, 126, 220, 3, 111, 146, 1, 125, 214, 248, 150, 97, 221, 36, 2, 75,
        67, 116, 144, 113, 97, 87, 131, 202, 106, 15, 229, 70, 145, 32, 130, 150, 109, 186, 65,
        183, 239, 52, 55, 152, 111, 74, 195, 44, 165, 235, 74, 160, 221, 38, 215, 110, 101, 32,
        153, 53, 18, 168, 26, 84, 182, 101, 75, 52, 201, 101, 159, 73, 153, 210, 222, 153, 53, 217,
        211, 92, 197, 111, 21, 185, 248, 181, 231, 98, 158, 126, 43, 115, 241, 77, 63, 20, 198,
        157, 138, 228, 242, 253, 80, 159, 138, 92, 82, 253, 92, 80, 209, 81, 86, 148, 27, 88, 90,
        90, 248, 84, 2, 157, 15, 138, 157, 7, 238, 233, 4, 77, 222, 90, 155, 180, 69, 219, 33, 77,
        219, 86, 219, 186, 190, 42, 239, 92, 174, 17, 244, 206, 197, 27, 208, 224, 61, 5, 42, 194,
        72, 0, 64, 115, 128, 128, 0, 4, 7, 56, 154, 17, 197, 1, 200, 191, 90, 174, 163, 233, 152,
        158, 57, 224, 249, 103, 225, 89, 101, 125, 248, 7, 189, 72, 56, 137, 23, 132, 145, 184,
        80, 25, 249, 73, 0, 160, 102, 65, 75, 175, 172, 128, 253, 182, 238, 180, 173, 222, 134, 34,
        230, 148, 185, 229, 47, 86, 85, 3, 74, 126, 82, 231, 149, 45, 201, 82, 207, 110, 187, 241,
        23, 58, 152, 103, 136, 185, 236, 104, 85, 222, 75, 9, 82, 207, 50, 107, 91, 201, 97, 110,
        88, 146, 207, 18, 89, 237, 230, 151, 121, 138, 19, 49, 75, 60, 243, 177, 95, 185, 197, 11,
        28, 83, 41, 44, 241, 41, 31, 89, 138, 26, 36, 76, 209, 66, 40, 221, 194, 78, 188, 74, 6,
        194, 160, 20, 126, 131, 150, 233, 188, 68, 204, 162, 117, 212, 31, 199, 108, 168, 96, 120,
        142, 239, 52, 206, 216, 166, 10, 15, 58, 28, 7, 160, 164, 240, 146, 8, 38, 130, 2, 72, 162,
        8, 41, 0, 140, 32, 17, 64, 70, 18, 70, 16, 91, 36, 27, 232, 6, 189, 128, 91, 222, 239, 177,
        109, 1, 239, 75, 194, 71, 234, 238, 115, 244, 150, 117, 96, 249, 136, 139, 247, 17, 167,
        251, 200, 210, 121, 40, 225, 89, 86, 210, 64, 11, 100, 131, 111, 0, 246, 216, 97, 43, 15,
        228, 250, 43, 132, 69, 206, 76, 54, 150, 92, 234, 97, 180, 166, 25, 237, 151, 12, 69, 205,
        110, 184, 109, 211, 172, 160, 206, 160, 209, 20, 125, 184, 186, 183, 48, 175, 158, 156,
        184, 125, 89, 101, 222, 88, 58, 181, 131, 113, 71, 150, 160, 251, 81, 19, 5, 53, 104, 183,
        195, 171, 42, 109, 92, 73, 161, 177, 176, 122, 34, 96, 166, 148, 234, 13, 26, 196, 195,
        68, 67, 5, 1, 77, 91, 158, 45, 129, 227, 233, 75, 133, 165, 175, 109, 131, 86, 220, 111,
        18, 170, 129, 21, 58, 164, 239, 28, 192, 241, 29, 196, 211, 35, 18, 192, 246, 240, 130, 12,
        7, 96, 192, 11, 104, 15, 219, 64, 12, 4, 139, 226, 192, 103, 9, 58, 164, 128, 130, 64, 65,
        32, 52, 180, 6, 150, 128, 117, 143, 13, 34, 193, 60, 31, 172, 229, 239, 128, 254, 251, 107,
        242, 124, 207, 1, 191, 181, 100, 109, 63, 220, 207, 211, 254, 93, 13, 68, 0, 192, 179, 13,
        215, 226, 123, 214, 238, 251, 59, 238, 224, 191, 127, 187, 204, 211, 217, 213, 139, 130,
        239, 139, 74, 182, 178, 128, 88, 208, 52, 76, 54, 98, 42, 241, 6, 133, 6, 201, 19, 114, 42,
        121, 209, 55, 110, 202, 145, 216, 213, 88, 61, 21, 48, 38, 91, 81, 95, 137, 165, 11, 25,
        14, 204, 95, 46, 100, 218, 183, 41, 187, 20, 127, 250, 68, 221, 18, 96, 152, 69, 79, 177,
        109, 124, 244, 201, 174, 76, 219, 57, 110, 155, 209, 83, 203, 18, 96, 10, 39, 155, 163, 21,
        48, 221, 6, 21, 134, 135, 139, 64, 186, 170, 1, 244, 108, 42, 235, 6, 22, 195, 2, 58, 160,
        19, 74, 241, 59, 199, 34, 96, 178, 166, 25, 212, 221, 181, 99, 8, 9, 0, 202, 66, 111, 3,
        188, 127, 197, 212, 162, 142, 11, 116, 232, 124, 20, 117, 46, 88, 140, 8, 175, 101, 101,
        91, 34, 109, 172, 155, 107, 38, 218, 138, 182, 180, 38, 176, 19, 5, 54, 173, 111, 190, 147,
        223, 99, 106, 231, 202, 251, 206, 250, 91, 240, 242, 54, 173, 249, 30, 75, 127, 103, 106,
        223, 145, 221, 163, 237, 237, 91, 249, 251, 254, 214, 251, 185, 122, 218, 200, 34, 120,
        149, 169, 208, 102, 58, 111, 68, 24, 16, 111, 17, 86, 214, 25, 162, 191, 20, 117, 109,
        134, 27, 165, 222, 148, 189, 109, 85, 131, 202, 39, 158, 162, 110, 168, 248, 195, 9, 50,
        219, 97, 214, 153, 64, 243, 151, 114, 221, 55, 162, 142, 184, 235, 40, 155, 105, 154, 39,
        245, 183, 221, 189, 99, 44, 219, 196, 235, 68, 24, 40, 158, 96, 128, 20, 77, 189, 18, 38,
        212, 81, 129, 53, 45, 147, 86, 217, 48, 81, 172, 24, 40, 4, 2, 13, 161, 181, 20, 117, 200,
        171, 81, 71, 217, 82, 185, 19, 208, 32, 235, 44, 4, 77, 165, 6, 4, 212, 183, 245, 27, 193,
        65, 48, 81, 98, 168, 17, 165, 154, 32, 218, 88, 117, 212, 174, 68, 218, 88, 97, 108, 58,
        234, 109, 92, 223, 125, 55, 126, 206, 213, 205, 71, 81, 239, 99, 239, 55, 237, 121, 40,
        246, 188, 182, 189, 175, 45, 175, 241, 242, 90, 242, 60, 182, 245, 30, 59, 247, 182, 65,
        167, 241, 28, 163, 226, 55, 70, 134, 110, 169, 55, 162, 171, 253, 84, 27, 164, 27, 203, 1,
        6, 218, 195, 204, 194, 77, 252, 5, 194, 221, 119, 236, 83, 5, 19, 1, 2, 154, 87, 73, 96,
        230, 179, 118, 246, 178, 89, 118, 242, 203, 42, 89, 113, 52, 217, 145, 19, 237, 69, 76,
        20, 77, 166, 161, 166, 17, 107, 197, 172, 162, 168, 138, 152, 17, 63, 251, 16, 2, 143, 232,
        144, 109, 195, 49, 6, 114, 13, 146, 15, 178, 65, 152, 196, 49, 140, 65, 152, 196, 174, 65,
        152, 102, 32, 192, 198, 88, 102, 25, 140, 177, 150, 30, 82, 195, 116, 144, 140, 65, 130,
        195, 49, 140, 51, 44, 11, 42, 254, 251, 188, 208, 241, 188, 12, 223, 153, 245, 195, 125,
        212, 123, 218, 246, 115, 125, 175, 100, 252, 191, 103, 175, 197, 253, 189, 126, 46, 253,
        221, 71, 142, 255, 10, 200, 20, 78, 190, 18, 117, 236, 112, 205, 111, 64, 219, 205, 51,
        177, 7, 85, 65, 186, 65, 152, 196, 127, 10, 193, 249, 88, 45, 225, 46, 24, 154, 103, 34,
        38, 241, 19, 49, 12, 160, 235, 195, 31, 196, 165, 130, 9, 96, 218, 154, 182, 240, 81, 188,
        29, 101, 6, 17, 83, 206, 80, 4, 4, 76, 198, 88, 20, 76, 198, 81, 58, 194, 0, 81, 55, 116,
        73, 9, 225, 4, 49, 189, 15, 52, 117, 136, 51, 136, 8, 55, 132, 0, 48, 2, 13, 193, 0, 16,
        117, 200, 60, 193, 1, 21, 105, 146, 50, 69, 90, 16, 192, 131, 120, 64, 177, 97, 170, 241,
        50, 214, 203, 89, 80, 89, 80, 223, 42, 253, 239, 210, 15, 153, 224, 103, 252, 255, 167,
        87, 167, 253, 149, 253, 46, 168, 123, 30, 201, 184, 190, 203, 255, 171, 203, 13, 70, 253,
        251, 219, 32, 81, 90, 203, 38, 209, 7, 94, 40, 206, 77, 43, 107, 88, 32, 137, 161, 97, 160,
        138, 184, 176, 44, 171, 228, 212, 182, 166, 10, 22, 213, 183, 17, 80, 139, 11, 40, 131, 2,
        13, 162, 12, 63, 172, 48, 34, 111, 8, 0, 96, 76, 178, 38, 226, 149, 196, 10, 13, 226, 13,
        194, 13, 161, 38, 0, 162, 110, 40, 155, 68, 124, 111, 104, 195, 31, 96, 196, 132, 200, 18,
        188, 49, 160, 22, 1, 1, 25, 32, 235, 67, 31, 101, 106, 194, 0, 5, 128, 138, 160, 67, 0,
        88, 12, 54, 150, 87, 44, 138, 229, 17, 84, 90, 170, 93, 234, 165, 224, 218, 23, 141, 104,
        110, 55, 122, 214, 251, 129, 155, 196, 249, 97, 246, 62, 91, 255, 103, 192, 207, 249, 255,
        92, 255, 107, 213, 111, 133, 250, 209, 246, 254, 183, 111, 198, 241, 251, 55, 190, 56, 120,
        221, 200, 113, 237, 13, 199, 84, 222, 82, 165, 224, 170, 93, 234, 165, 239, 213, 47, 126,
        50, 145, 52, 44, 51, 44, 85, 159, 211, 170, 101, 6, 120, 163, 15, 73, 87, 253, 43, 39, 86,
        27, 196, 27, 196, 1, 12, 8, 54, 200, 48, 32, 16, 4, 48, 2, 0, 128, 8, 4, 27, 134, 80, 8,
        98, 13, 194, 24, 131, 108, 131, 13, 18, 66, 120, 143, 195, 24, 18, 19, 176, 159, 35, 74,
        180, 65, 180, 212, 38, 161, 52, 33, 128, 0, 1, 21, 112, 128, 65, 230, 44, 60, 162, 12, 246,
        170, 151, 134, 169, 120, 234, 155, 209, 180, 63, 55, 190, 107, 157, 225, 161, 142, 3, 108,
        99, 33, 143, 61, 166, 60, 243, 177, 140, 142, 38, 56, 113, 252, 150, 184, 251, 243, 240,
        247, 230, 226, 121, 38, 189, 242, 203, 196, 223, 155, 135, 253, 166, 225, 248, 236, 240,
        188, 234, 220, 238, 102, 131, 24, 232, 99, 241, 183, 206, 243, 155, 231, 126, 6, 255, 39,
        1, 190, 119, 134, 142, 119, 124, 215, 252, 119, 166, 233, 170, 127, 91, 234, 55, 211, 240,
        154, 245, 172, 141, 197, 84, 56, 190, 168, 114, 59, 135, 150, 27, 196, 27, 77, 16, 109, 52,
        241, 7, 89, 103, 200, 54, 154, 204, 16, 17, 167, 132, 0, 4, 203, 104, 109, 16, 109, 17,
        173, 18, 66, 120, 110, 173, 9, 232, 99, 121, 96, 35, 48, 198, 228, 54, 89, 101, 146, 66,
        236, 143, 52, 131, 101, 84, 17, 197, 135, 154, 69, 89, 32, 235, 112, 85, 14, 61, 161, 249,
        190, 26, 57, 222, 114, 24, 199, 109, 140, 131, 177, 82, 182, 77, 70, 170, 51, 147, 81, 150,
        42, 55, 147, 83, 41, 138, 153, 204, 100, 85, 99, 159, 149, 208, 200, 237, 251, 152, 225,
        239, 114, 89, 247, 249, 38, 247, 249, 133, 247, 57, 133, 253, 152, 225, 238, 115, 3, 221,
        199, 47, 187, 142, 95, 54, 165, 86, 50, 62, 38, 57, 255, 23, 59, 25, 28, 110, 103, 199,
        233, 243, 13, 233, 255, 125, 126, 47, 43, 51, 139, 235, 151, 145, 212, 237, 114, 250, 166,
        226, 250, 253, 143, 249, 251, 174, 110, 187, 135, 150, 85, 197, 134, 241, 7, 217, 32, 217,
        35, 36, 31, 102, 214, 19, 43, 13, 146, 50, 4, 206, 32, 217, 32, 217, 37, 85, 128, 140, 178,
        201, 132, 0, 76, 221, 18, 12, 22, 0, 134, 11, 1, 6, 208, 161, 4, 56, 78, 136, 0, 2, 28, 39,
        33, 255, 34, 52, 65, 215, 32, 243, 45, 67, 131, 104, 255, 155, 192, 208, 231, 115, 16, 198,
        59, 76, 115, 219, 232, 212, 6, 30, 98, 163, 153, 61, 39, 178, 122, 78, 228, 212, 115, 38,
        160, 48, 94, 141, 64, 98, 163, 221, 12, 128, 232, 84, 171, 189, 199, 127, 191, 253, 205,
        121, 95, 196, 190, 239, 88, 63, 235, 172, 143, 195, 214, 47, 236, 254, 37, 252, 61, 96,
        247, 250, 193, 229, 127, 171, 251, 220, 114, 243, 106, 3, 24, 249, 124, 108, 128, 244, 255,
        5, 126, 31, 229, 63, 230, 218, 179, 200, 243, 28, 189, 229, 85, 226, 245, 29, 246, 255, 91,
        126, 199, 118, 230, 146, 201, 213, 134, 233, 6, 208, 220, 16, 16, 109, 13, 235, 88, 79, 44,
        98, 66, 51, 36, 60, 56, 71, 26, 33, 142, 178, 192, 32, 0, 32, 220, 16, 4, 81, 229, 232,
        195, 30, 108, 49, 246, 72, 55, 190, 26, 176, 217, 101, 150, 68, 4, 133, 86, 24, 222, 32,
        50, 11, 101, 144, 121, 155, 165, 67, 131, 140, 247, 27, 152, 247, 58, 163, 172, 84, 117,
        138, 128, 197, 71, 88, 101, 138, 149, 178, 122, 89, 217, 61, 44, 220, 209, 0, 100, 212,
        121, 128, 99, 35, 49, 142, 75, 156, 30, 249, 238, 233, 38, 191, 205, 32, 181, 253, 199,
        252, 63, 196, 61, 238, 244, 254, 237, 153, 189, 206, 246, 191, 185, 214, 15, 127, 195, 115,
        131, 102, 27, 223, 58, 175, 55, 30, 175, 67, 33, 223, 67, 158, 30, 142, 54, 119, 155, 249,
        222, 229, 242, 130, 247, 149, 157, 247, 253, 80, 246, 63, 111, 199, 236, 119, 14, 105, 21,
        117, 101, 65, 101, 94, 88, 108, 178, 32, 0, 2, 13, 150, 73, 9, 219, 88, 69, 90, 209, 145,
        129, 1, 108, 226, 42, 203, 44, 172, 55, 136, 171, 36, 27, 154, 36, 26, 22, 31, 194, 65,
        160, 131, 5, 159, 181, 132, 117, 144, 55, 161, 194, 104, 104, 132, 132, 13, 3, 64, 209,
        106, 63, 231, 106, 54, 173, 124, 39, 56, 216, 207, 122, 31, 161, 214, 42, 118, 152, 168,
        239, 111, 38, 167, 201, 147, 81, 8, 227, 116, 159, 227, 84, 47, 26, 161, 253, 78, 149, 86,
        58, 79, 177, 83, 61, 138, 159, 35, 25, 29, 142, 142, 70, 119, 67, 195, 207, 244, 108, 203,
        189, 72, 189, 248, 126, 203, 58, 252, 63, 224, 30, 247, 88, 61, 206, 246, 183, 205, 253,
        213, 125, 255, 242, 115, 202, 179, 13, 239, 157, 91, 155, 207, 173, 209, 231, 155, 162, 94,
        118, 51, 190, 159, 227, 14, 63, 40, 252, 63, 197, 87, 139, 202, 237, 116, 250, 134, 246,
        251, 151, 61, 110, 224, 186, 117, 76, 69, 64, 138, 246, 150, 2, 12, 240, 198, 241, 50, 161,
        143, 48, 144, 158, 134, 51, 144, 103, 88, 127, 200, 138, 161, 97, 129, 6, 132, 17, 8, 204,
        16, 1, 51, 180, 72, 55, 4, 16, 223, 68, 53, 92, 181, 102, 209, 152, 121, 182, 176, 153,
        100, 64, 16, 193, 150, 109, 107, 195, 180, 11, 95, 9, 206, 14, 51, 220, 111, 208, 235, 21,
        30, 232, 212, 172, 197, 78, 219, 21, 51, 216, 233, 29, 142, 145, 255, 37, 64, 253, 60, 10,
        188, 111, 61, 239, 67, 253, 51, 216, 168, 102, 42, 51, 147, 80, 245, 25, 169, 158, 199, 75,
        41, 142, 99, 124, 6, 250, 61, 242, 24, 179, 107, 123, 143, 91, 155, 251, 234, 251, 220, 12,
        255, 71, 190, 107, 205, 254, 12, 249, 92, 159, 133, 142, 98, 57, 217, 1, 209, 200, 236, 49,
        144, 247, 67, 25, 223, 71, 158, 239, 67, 35, 47, 205, 252, 238, 114, 255, 91, 189, 62, 171,
        222, 223, 85, 28, 78, 168, 123, 118, 71, 226, 89, 5, 170, 181, 109, 85, 126, 212, 117, 86,
        236, 90, 194, 101, 109, 2, 13, 239, 231, 11, 227, 32, 235, 8, 0, 0, 131, 101, 146, 15, 178,
        65, 178, 201, 51, 116, 72, 62, 193, 6, 130, 144, 101, 181, 8, 118, 141, 53, 14, 210, 172,
        50, 150, 28, 34, 144, 165, 36, 34, 150, 28, 34, 218, 194, 205, 249, 182, 129, 193, 198,
        123, 155, 141, 95, 212, 168, 243, 21, 14, 199, 99, 38, 163, 204, 23, 38, 160, 100, 228, 61,
        254, 220, 7, 120, 56, 206, 250, 30, 120, 122, 31, 163, 63, 244, 212, 12, 156, 135, 63, 79,
        75, 225, 16, 100, 134, 72, 100, 212, 66, 24, 200, 109, 140, 131, 177, 201, 67, 24, 200, 99,
        147, 159, 208, 253, 231, 247, 185, 53, 185, 185, 14, 244, 50, 3, 160, 95, 67, 32, 56, 63,
        129, 207, 83, 207, 205, 245, 49, 207, 230, 254, 119, 61, 62, 83, 220, 190, 165, 94, 159,
        84, 189, 62, 169, 120, 253, 83, 122, 63, 220, 215, 55, 190, 62, 246, 208, 22, 180, 207,
        181, 180, 5, 149, 203, 134, 136, 112, 138, 66, 195, 26, 7, 152, 65, 157, 110, 208, 8, 0, 0,
        0, 131, 123, 71, 255, 224,
    ];

    let lut = build_lut(lengths, symbols);
    let mut pos = 0;
    let mut bit = 0;
    let mut code = 1;
    loop {
        let b = (compressed[pos] >> (7 - bit)) & 1;
        code = (code << 1) + (b as i32);
        let res = lut.get(&code);
        if res.is_some() {
            let symbol = *res.unwrap();
            if symbol == 255 { break; }
            output_symbol(symbol, colors, chars);
            code = 1;
        }

        bit += 1;
        if bit == 8 {
            bit = 0;
            pos += 1;
        }
    }
}
