mod number;
pub use number::{Number, BASE};

#[cfg(test)]
#[cfg(not(feature = "base10"))]
mod tests {
    use super::*;

    #[test]
    fn calc_1_plus_1() {
        let a: Number = "1".parse().unwrap();
        let b: Number = "1".parse().unwrap();
        let result = (a + b).to_string();

        assert_eq!(result, "2");
    }

    #[test]
    fn calc_minus_1_plus_2() {
        let a: Number = "-1".parse().unwrap();
        let b: Number = "2".parse().unwrap();
        let result = (a + b).to_string();

        assert_eq!(result, "1");
    }


    #[test]
    fn calc_3_times_3_dot_0() {
        let a: Number = "3".parse().unwrap();
        let b: Number = "3,0".parse().unwrap();
        let result = (a * b).to_string();

        assert_eq!(result, "10");
    }


    #[test]
    fn calc_1_dot_3_plus_2_dot_6() {
        let a: Number = "1,3".parse().unwrap();
        let b: Number = "2,6".parse().unwrap();
        let result = (a + b).to_string();

        assert_eq!(result, "4");
    }


    #[test]
    fn calc_1_dot_3_minus_2_dot_6() {
        let a: Number = "1,3".parse().unwrap();
        let b: Number = "2,6".parse().unwrap();
        let result = (a - b).to_string();

        assert_eq!(result, "-1,3");
    }


    #[test]
    fn calc_10_times_12_dot_3() {
        let a: Number = "10".parse().unwrap();
        let b: Number = "12,3".parse().unwrap();
        let result = (a * b).to_string();

        assert_eq!(result, "123");
    }

    #[test]
    fn calc_0034_plus_5_dot_0() {
        let a: Number = "0034".parse().unwrap();
        let b: Number = "5,0".parse().unwrap();
        let result = (a + b).to_string();

        assert_eq!(result, "40");
    }

    #[test]
    fn calc_2_dot_4_times_1_dot_3() {
        let a: Number = "2,4".parse().unwrap();
        let b: Number = "1,3".parse().unwrap();
        let result = (a * b).to_string();

        assert_eq!(result, "3,23");
    }

    #[test]
    fn calc_with_many_digits() {
        let a: Number = "800000000000000000000000000000000".parse().unwrap();
        let b: Number = "0,0000000000000000000000000007".parse().unwrap();
        let result = (a + b).to_string();

        assert_eq!(result, "800000000000000000000000000000000,0000000000000000000000000007");
    }


    #[test]
    fn calc_0_dot_2_times_0_dot_2() {
        let a: Number = "0,2".parse().unwrap();
        let b: Number = "0,2".parse().unwrap();
        let result = (a * b).to_string();

        assert_eq!(result, "0,04");
    }
}
