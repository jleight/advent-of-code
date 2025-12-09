pub fn solve(input: &str) -> String {
    let junction_boxes: Vec<Point> = input
        .lines()
        .map(Point::parse)
        .collect();
    let junction_box_count = junction_boxes.len();

    let mut distances: Vec<((&Point, &Point), f64)> = Vec::new();

    for ai in 0..junction_box_count - 1 {
        let a = &junction_boxes[ai];

        for b in junction_boxes
            .iter()
            .skip(ai + 1)
        {
            distances.push(((a, b), a.distance_to(b)));
        }
    }

    distances.sort_by(|a, b| b.1.total_cmp(&a.1));

    let connections = if junction_box_count < 100 { 10 } else { 1000 };
    let mut circuits: Vec<Box<Vec<&Point>>> = Vec::new();

    for _ in 0..connections {
        let connection = distances
            .pop()
            .expect("no more connections");
        let (a, b) = connection.0;

        let ai = circuits
            .iter()
            .position(|c| c.contains(&a));
        let bi = circuits
            .iter()
            .position(|c| c.contains(&b));

        if let Some(ai) = ai
            && let Some(bi) = bi
        {
            if ai == bi {
                continue;
            }

            let b_circuit = circuits
                .get_mut(bi)
                .unwrap()
                .clone();
            let a_circuit = circuits
                .get_mut(ai)
                .unwrap();

            for x in b_circuit.iter() {
                a_circuit.push(*x);
            }
            circuits.remove(bi);
            continue;
        }

        if let Some(ai) = ai {
            let a_circuit = circuits
                .get_mut(ai)
                .unwrap();

            a_circuit.push(b);
            continue;
        }

        if let Some(bi) = bi {
            let b_circuit = circuits
                .get_mut(bi)
                .unwrap();

            b_circuit.push(a);
            continue;
        }

        circuits.push(Box::new(vec![a, b]));
    }

    circuits.sort_by_key(|c| std::cmp::Reverse(c.len()));

    circuits
        .iter()
        .take(3)
        .map(|c| c.len())
        .product::<usize>()
        .to_string()
}

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn parse(line: &str) -> Self {
        let split: Vec<&str> = line.split(',').collect();

        Self {
            x: split
                .first()
                .expect("invalid point: missing x")
                .parse()
                .expect("invalid point: invalid x"),
            y: split
                .get(1)
                .expect("invalid point: missing y")
                .parse()
                .expect("invalid point: invalid y"),
            z: split
                .get(2)
                .expect("invalid point: missing z")
                .parse()
                .expect("invalid point: invalid z"),
        }
    }

    fn distance_to(&self, other: &Self) -> f64 {
        let dx = (self.x - other.x).powi(2);
        let dy = (self.y - other.y).powi(2);
        let dz = (self.z - other.z).powi(2);

        (dx + dy + dz).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 8)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 8)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
