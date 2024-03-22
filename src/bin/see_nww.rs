fn see_nww(mut arg: i64) {
    let mut n = 2;
    loop {
        if arg % n == 0 {
            println!("{} : {}", arg, n);
            arg /= n;
            n = 2;
        } else {
            n += 1;
        }
        if arg == 1 {
            // println!("{} : {} ", arg,n);
            println!("{}", 1);
            break;
        }
    }
}

fn main() {
    see_nww(72);
}
