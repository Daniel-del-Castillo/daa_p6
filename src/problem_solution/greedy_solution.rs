use super::{ProblemInstance, ProblemSolution};
use std::collections::hash_set::HashSet;

pub struct GreedySolution {
    tcts_by_machine: Vec<usize>,
    task_assignment_matrix: Vec<Vec<usize>>,
}

impl ProblemSolution for GreedySolution {
    fn solve(instance: &ProblemInstance) -> Self {
        let mut asigned_tasks = HashSet::with_capacity(instance.task_times().len());
        let mut solution = GreedySolution {
            tcts_by_machine: Vec::with_capacity(instance.number_of_machines()),
            task_assignment_matrix: Vec::with_capacity(instance.number_of_machines()),
        };
        solution.choose_initial_tasks(instance, &mut asigned_tasks);
        while asigned_tasks.len() < instance.task_times().len() {
            solution.add_task(instance, &mut asigned_tasks);
        }
        solution
    }

    fn get_total_completion_time(&self) -> usize {
        self.tcts_by_machine.iter().sum()
    }

    fn get_tcts_by_machine(&self) -> &Vec<usize> {
        &self.tcts_by_machine
    }

    fn get_tasks_by_machine(&self) -> &Vec<Vec<usize>> {
        &self.task_assignment_matrix
    }
}

struct NewTask {
    machine: usize,
    task: usize,
    position: usize,
    tct_increment: usize,
}

impl GreedySolution {
    fn choose_initial_tasks(
        &mut self,
        instance: &ProblemInstance,
        asigned_tasks: &mut HashSet<usize>,
    ) {
        for _ in 0..instance.number_of_machines() {
            let task = instance.setup_times()[0]
                .iter()
                .skip(1) // Skip the starting setup times column
                .enumerate()
                .filter(|(index, _)| !asigned_tasks.contains(index))
                .map(|(index, setup)| (index, setup + instance.task_times()[index]))
                .min_by_key(|(_, cost)| *cost);
            let (task, tct_increment) = match task {
                Some(values) => values,
                None => break,
            };
            self.task_assignment_matrix.push(vec![task]);
            self.tcts_by_machine.push(tct_increment);
            asigned_tasks.insert(task);
        }
    }

    fn add_task(&mut self, instance: &ProblemInstance, asigned_tasks: &mut HashSet<usize>) {
        let new_task = self.get_best_next_machine_and_task(instance, asigned_tasks);
        self.task_assignment_matrix[new_task.machine].insert(new_task.position, new_task.task);
        asigned_tasks.insert(new_task.task);
        self.tcts_by_machine[new_task.machine] += new_task.tct_increment;
    }

    fn get_best_next_machine_and_task(
        &self,
        instance: &ProblemInstance,
        asigned_tasks: &HashSet<usize>,
    ) -> NewTask {
        (0..instance.number_of_machines())
            .map(|machine| self.get_best_next_task_for_machine(instance, machine, &asigned_tasks))
            .min_by_key(|new_task| new_task.tct_increment)
            // Panics if all the tasks have been asigned. This function shouldn't be called in such cases
            .unwrap()
    }

    fn get_best_next_task_for_machine(
        &self,
        instance: &ProblemInstance,
        machine: usize,
        asigned_tasks: &HashSet<usize>,
    ) -> NewTask {
        (0..instance.task_times().len())
            .filter(|index| !asigned_tasks.contains(index))
            .map(|task| self.get_best_pos_for_task_in_machine(instance, task, machine))
            .min_by_key(|new_task| new_task.tct_increment)
            // Panics if all the tasks have been asigned. This function shouldn't be called in such cases
            .unwrap()
    }

    fn get_best_pos_for_task_in_machine(
        &self,
        instance: &ProblemInstance,
        task: usize,
        machine: usize,
    ) -> NewTask {
        (0..=self.task_assignment_matrix[machine].len())
            .map(|position| {
                let mut task_list = self.task_assignment_matrix[machine].clone();
                task_list.insert(position, task);
                let tct_increment = instance.calculate_total_completion_time(task_list)
                    - self.tcts_by_machine[machine];
                NewTask {
                    task,
                    position,
                    machine,
                    tct_increment,
                }
            })
            .min_by_key(|new_task| new_task.tct_increment)
            // Panics if all the tasks have been asigned. This function shouldn't be called in such cases
            .unwrap()
    }
}
