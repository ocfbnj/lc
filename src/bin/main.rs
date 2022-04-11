fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut it = line.split(" ");

    let n = it.next().unwrap().trim().parse::<i32>().unwrap();
    let l = it.next().unwrap().trim().parse::<i32>().unwrap();

    let mut first = -1;
    let mut count = 0;

    for i in l..=100 {
        if (2 * n) % i != 0 {
            continue;
        }

        let temp = 2 * n / i + 1 - i;
        if temp < 0 {
            break;
        }

        if temp % 2 != 0 {
            continue;
        }

        first = temp / 2;
        count = i;
        break;
    }

    match first {
        -1 => println!("No"),
        _ => {
            for i in first..first + count {
                print!("{}", i);
                if i != first + count - 1 {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
