pub fn fmt_duration(s: i32) -> String {
    let h = s / 3600;
    let m = (s % 3600) / 60;
    let sec = s % 60;
    if h > 0 {
        format!("{h}:{m:02}:{sec:02}")
    } else {
        format!("{m}:{sec:02}")
    }
}

pub fn fmt_distance(m: f64) -> String {
    if m >= 1000.0 {
        format!("{:.2} km", m / 1000.0)
    } else {
        format!("{:.0} m", m)
    }
}

pub fn fmt_elevation(m: f64) -> String {
    format!("{:.0} m", m)
}

pub fn fmt_pace(s_per_km: f64) -> String {
    let secs = s_per_km as i64;
    format!("{}:{:02} /km", secs / 60, secs % 60)
}

pub fn format_published(s: &str) -> String {
    // Validate that `s` begins with a well-formed ISO 8601 datetime prefix:
    // YYYY-MM-DDTHH:MM  (positions 0-15, length >= 16)
    // Separator positions: 4='-', 7='-', 10='T', 13=':'
    // All other positions in 0..16 must be ASCII digits.
    let is_valid = s.len() >= 16 && {
        let b = s.as_bytes();
        const SEP: [(usize, u8); 4] = [(4, b'-'), (7, b'-'), (10, b'T'), (13, b':')];
        let seps_ok = SEP.iter().all(|&(i, c)| b[i] == c);
        let digits_ok = (0..16)
            .filter(|i| !SEP.iter().any(|&(si, _)| si == *i))
            .all(|i| b[i].is_ascii_digit());
        seps_ok && digits_ok
    };

    if is_valid {
        // Safe: we verified the byte layout above, so these slices are on char boundaries.
        format!("{} {}", &s[..10], &s[11..16])
    } else {
        s.to_string()
    }
}
