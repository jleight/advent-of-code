use crate::utils::SolutionContext;

pub fn solve(ctx: &SolutionContext) -> String {
    let rows = ctx.input_lines.len();
    let cols = ctx.input_lines[0].len();

    let mut floor = vec![vec![false; cols]; rows];

    for (y, line) in ctx.input_lines.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '.' {
                continue;
            }

            floor[y][x] = true;
        }
    }

    let mut sum = 0;

    loop {
        let mut changed = false;

        let mut neighbors = vec![vec![0u8; cols + 2]; rows + 2];

        for y in 0..rows {
            for x in 0..cols {
                if !floor[y][x] {
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

        for y in 0..rows {
            for x in 0..cols {
                if floor[y][x] && neighbors[y + 1][x + 1] < 4 {
                    floor[y][x] = false;
                    sum += 1;
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::SolutionContext;

    #[test]
    fn test_test() {
        let ctx = SolutionContext::for_problem(2025, 4, 2, true);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }

    #[test]
    fn test_full() {
        let ctx = SolutionContext::for_problem(2025, 4, 2, false);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }
}
