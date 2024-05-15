use std::io;

pub fn get_input() -> (i32, i32, i32, i32, i32, i32, i32) {
    let input = io::stdin().lines().next().unwrap();

    return match input {
        Ok(txt) => {
            let vector: Vec<i32> = txt.split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            (vector[0], vector[1], vector[2], vector[3], vector[4], vector[5], vector[6])
        },
        Err(e) => panic!("{}", e)
    };
}

pub fn get_modular_inverse(mut num: i32, mut p: i32) -> i32 {

    if num == 0 {
        panic!("num must be non-zero");
    }

    num = num.rem_euclid(p);

    let buf_n = p;
    let (mut buf1, mut buf2): (i32, i32) = (0, 1);

    while num != 1 {
        (buf1, buf2) = (buf2, (buf1 - (buf2 * (p / num)).rem_euclid(buf_n)).rem_euclid(buf_n));
        (p, num) = (num, p % num);
    }

    buf2.rem_euclid(buf_n)
}