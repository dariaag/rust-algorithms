fn linear_search(haystack: &[i32], needle: &i32) -> Vec<usize> {
    let res = haystack
        .iter()
        .enumerate()
        .map(|(idx, _)| return idx)
        .filter(|&x| return haystack[x] == *needle)
        .collect::<Vec<usize>>();
    res
}

fn main() {
    let haystack = [1, 2, 3, 4, 5];
    let needle = 9;
    let res = linear_search(&haystack, &needle);
    println!("{:?}", res);
}
