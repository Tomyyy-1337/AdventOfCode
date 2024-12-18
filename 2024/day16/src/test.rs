#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part_1_test_1() {
        let result = part1("input/test");
        assert_eq!(result, 7036);
    }

    #[test]
    fn part_1_test_2() {
        let result = part1("input/test2");
        assert_eq!(result, 11048);
    }

    #[test]
    fn part_1_test_3() {
        let result = part1("input/puzzle");
        assert_eq!(result, 92432);
    }

    #[test]
    fn part_2_test_1() {
        let result = part2("input/test");
        assert_eq!(result, 45);
    }

    #[test]
    fn part_2_test_2() {
        let result = part2("input/test2");
        assert_eq!(result, 64);
    }

    #[test]
    fn part_2_test_3() {
        let result = part2("input/puzzle");
        assert_eq!(result, 458);
    }
}
