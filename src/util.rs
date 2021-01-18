pub fn gen_intervals(start: usize, interval_count: usize, typesize: usize) -> impl Iterator<Item = usize> {
    (0..interval_count).map(move |x| start + (x * typesize))
}