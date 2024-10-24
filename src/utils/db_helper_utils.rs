pub fn get_seconds_per_interval(interval: &str) -> i32 {
    match interval {
        "hour" => 3600,
        "day" => 86_400,
        "week" => 604_800,
        "month" => 2_678_400,
        "quarter" => 7_948_800,
        "year" => 31_622_400,
        _ => 3_600,
    }
}