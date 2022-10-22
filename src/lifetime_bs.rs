#![allow(dead_code)]

fn noalias(input: &i32, output: &mut i32) {
    if *input == 1 {
        *output = 2;
    }
    // this is optimized by the compiler to an if else
    if *input != 1 {
        *output = 3;
    }
}

fn replace_with_84(s: &mut Box<i32>) {
    // this is not okay as *s would be empty
    // let was = *s
    // but this is
    let was = std::mem::take(s);
    // so is this because there is a value in s
    *s = was;

    // we can exchange values behind &mut
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);

    assert_ne!(*r, 84)

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_noalias() {
        let inp = 1;
        let mut out = 0;
        noalias(&inp, &mut out);
        assert_eq!(out, 2);

        let inp = 2;
        let mut out = 0;
        noalias(&inp, &mut out);
        assert_eq!(out, 3);
    }

    #[test]
    fn test_replace_with84() {
        let mut s = Box::new(42);
        replace_with_84(&mut s);
    }
}
