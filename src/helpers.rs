pub fn format_number(n: u64) -> String {
    let mut step = 0;
    let suffix = ["b", "kb", "mb", "gb", "tb"];
    let mut float_val: f64 = n as f64;
    while float_val >= 1024.0 {
        step += 1;
        float_val = float_val / 1024.0;
    }
    format!("{:.2} {}", float_val, suffix[step])
}
