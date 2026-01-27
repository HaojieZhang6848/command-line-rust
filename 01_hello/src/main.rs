fn main() {
    let utc_time = chrono::Utc::now();
    let shanghai_tz = chrono_tz::Asia::Shanghai;
    let nashville_tz = chrono_tz::America::Chicago;
    let shanghai_time = utc_time.with_timezone(&shanghai_tz);
    let nashville_time = utc_time.with_timezone(&nashville_tz);
    println!(
        "Hello, world, the time in Shanghai is {}, and the time in Nashville is {}!",
        shanghai_time.format("%Y-%m-%d %H:%M:%S"),
        nashville_time.format("%Y-%m-%d %H:%M:%S")
    );
}
