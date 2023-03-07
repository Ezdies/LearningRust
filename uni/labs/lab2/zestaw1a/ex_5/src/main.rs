struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

fn main() {
    let time1 = Time {
        hours: 20,
        minutes: 1,
        seconds: 10,
    };

    let time2 = Time {
        hours: 10,
        minutes: 0,
        seconds: 2,
    };

    let res = Time {
        hours: time1.hours.abs_diff(time2.hours),
        minutes: time1.minutes.abs_diff(time2.minutes),
        seconds: time1.seconds.abs_diff(time2.seconds),
    };

    println!(
        "Time difference: {}:{}:{}",
        res.hours, res.minutes, res.seconds
    );
}
