fn parse_decimal(s: &str) -> (i64, i64) {
    if !s.contains(".") {
        let int_part: i64 = s.parse().unwrap();
        (int_part, 0)
    } else {
        let mut split = s.split(".");
        let int_part: i64 = split.next().unwrap().parse().unwrap();
        let dec_part: i64 = split.next().unwrap().parse().unwrap();
        (int_part, dec_part)
    }
}
