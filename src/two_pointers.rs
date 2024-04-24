pub fn trap(height: Vec<i32>) -> i32 {
    let mut walls = height.iter();
    let mut lw = walls.next().expect("This height is invalid").clone();
    let mut rw = lw.clone();
    let mut water = Vec::new();
    let mut searching = false;

    loop {
        let curr = walls.next();
        match curr {
            Some(i) => {
                if (*i > lw) && (lw == rw) {
                    lw = *i;
                    rw = *i;
                }

                if (*i < lw) && (!searching) {
                    rw = *i;
                    searching = true;
                    water.push(*i);
                }
            }
            None => break,
        }
    }
    42
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn trap_example_1() {
    //     let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    //     let output = 6;
    //     assert_eq!(output, trap(height));
    // }
    //
    // #[test]
    // fn trap_example_2() {
    //     let height = vec![4, 2, 0, 3, 2, 5];
    //     let output = 9;
    //     assert_eq!(output, trap(height));
    // }
}
