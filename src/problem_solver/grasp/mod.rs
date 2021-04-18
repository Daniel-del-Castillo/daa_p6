use super::{ProblemInstance, ProblemSolution, ProblemSolver, RandomizedGreedySolver};

pub mod local_search;
use local_search::LocalSearch;

pub struct GRASP<L: LocalSearch> {
    size_to_choose_from: usize,
    repetitions: usize,
    local_search: L,
}

impl<L: LocalSearch> ProblemSolver for GRASP<L> {
    fn solve(self, instance: &ProblemInstance) -> ProblemSolution {
        (0..self.repetitions)
            .map(|_| {
                let solver = RandomizedGreedySolver::new(self.size_to_choose_from);
                let solution = solver.solve(instance);
                self.local_search.improve(instance, solution)
            })
            .min_by_key(|solution| solution.get_total_completion_time())
            .unwrap()
    }
}

impl<L: LocalSearch> GRASP<L> {
    pub fn new(size_to_choose_from: usize, repetitions: usize, local_search: L) -> Self {
        assert!(size_to_choose_from > 0 && repetitions > 0);
        GRASP {
            size_to_choose_from,
            repetitions,
            local_search,
        }
    }
}