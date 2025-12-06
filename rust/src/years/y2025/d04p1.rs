use crate::utils::SolutionContext;

pub fn solve(ctx: &SolutionContext) -> String {
    let rows = ctx.input_lines.len();
    let cols = ctx.input_lines[0].len();

    let mut neighbors = vec![vec![0u8; cols + 2]; rows + 2];

    for (y, line) in ctx.input_lines.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '.' {
                continue;
            }

            neighbors[y][x] += 1;
            neighbors[y][x + 1] += 1;
            neighbors[y][x + 2] += 1;
            neighbors[y + 1][x] += 1;
            neighbors[y + 1][x + 2] += 1;
            neighbors[y + 2][x] += 1;
            neighbors[y + 2][x + 1] += 1;
            neighbors[y + 2][x + 2] += 1;
        }
    }

    let mut sum = 0;

    for (y, line) in ctx.input_lines.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '@' && neighbors[y + 1][x + 1] < 4 {
                sum += 1;
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::SolutionContext;

    #[test]
    fn test_test() {
        let ctx = SolutionContext::for_problem(2025, 4, 1, true);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }

    #[test]
    fn test_full() {
        let ctx = SolutionContext::for_problem(2025, 4, 1, false);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }
}
