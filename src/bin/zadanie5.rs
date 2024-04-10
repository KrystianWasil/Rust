fn main() {
    let hour = 12; // dane wejściowe
    let min = 43;
    let sec = 12;
    let hour2 = 10;
    let min2 = 53;
    let sec2 = 15;

    let result = (hour * 3600 + min * 60 + sec) - (hour2 * 3600 + min2 * 60 + sec2);

    let hour_res = result / 3600;
    let min_res = (result % 3600) / 60;
    let sec_res = result % 60;

    println!("{}:{}:{}", hour_res, min_res, sec_res);
}
