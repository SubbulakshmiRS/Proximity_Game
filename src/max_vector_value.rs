use std::io::BufRead;

fn get_max(array_of_num: Vec<i32>) -> i32 {
    let size = array_of_num.len();
    if size < 1 {
        return 0;
    }
    let mut max_num = array_of_num[0];
    for i in 1..size {
        if array_of_num[i] > max_num {
            // println!("max update {} {}", i, array_of_num[i]);
            max_num = array_of_num[i];
        }
    }
    return max_num;
}

fn read_num<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<T>()
        .unwrap()
}

fn main() {
    let input = read_num::<usize>();
    let mut line_vec = vec![0; input];
    for ind in 0..input {
        line_vec[ind] = read_num::<i32>();
    }

    println!("vector: {}", line_vec[0]);
    let max_vec = get_max(line_vec);
    println!("max of vector: {}", max_vec);
}
