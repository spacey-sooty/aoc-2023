pub fn execute() {
    let path = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(&path).unwrap();
    let mut tmp_num: Vec<u8> = Vec::new();
    let mut result: i32 = 0;

    for line in contents.lines() {
        for char in line.as_bytes() {
            if char.is_ascii_digit() {
                tmp_num.push(*char);
            }
        }

        if tmp_num.len() == 1 {
            match tmp_num[0] {
                48 => result += 0,
                49 => result += 11,
                50 => result += 22,
                51 => result += 33,
                52 => result += 44,
                53 => result += 55,
                54 => result += 66,
                55 => result += 77,
                56 => result += 88,
                57 => result += 99,
                _ => panic!("Invalid Input (not a number)"),
            }
        } else {
            match tmp_num[0] {
                48 => result += 0,
                49 => result += 10,
                50 => result += 20,
                51 => result += 30,
                52 => result += 40,
                53 => result += 50,
                54 => result += 60,
                55 => result += 70,
                56 => result += 80,
                57 => result += 90,
                _ => panic!("Invalid Input (not a number)"),
            }

            match tmp_num[tmp_num.len() - 1] {
                48 => result += 0,
                49 => result += 1,
                50 => result += 2,
                51 => result += 3,
                52 => result += 4,
                53 => result += 5,
                54 => result += 6,
                55 => result += 7,
                56 => result += 8,
                57 => result += 9,
                _ => panic!("Invalid Input (not a number)"),
            }
        }

        tmp_num.clear()
    }

    println!("{}", result)
}
