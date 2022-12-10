#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PuzzlePart {
    Part1,
    Part2,
}

pub trait PuzzleSolver {
    fn solve(&self, input: &mut dyn Iterator<Item = u8>, part: PuzzlePart) -> String;
}

pub struct YearSolverCollection {
    solvers: Vec<Box<dyn PuzzleSolver>>,
}

impl YearSolverCollection {
    pub fn new() -> YearSolverCollection {
        YearSolverCollection { solvers: vec![] }
    }

    pub fn add<T>(&mut self)
    where
        T: PuzzleSolver + Default + 'static,
    {
        let solver = T::default();
        self.solvers.push(Box::new(solver));
    }

    pub fn solve(&self, day: u8, input: &mut dyn Iterator<Item = u8>, part: PuzzlePart) -> String {
        let day = usize::from(day);
        assert!(day >= 1);
        assert!(day <= self.solvers.len());

        let index = day - 1;
        self.solvers[index].solve(input, part)
    }
}
