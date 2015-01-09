use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use Satisfiability::{Satisfiable, NotSatisfiable};

enum Satisfiability {
    Satisfiable(Vec<Vec<bool>>),
    NotSatisfiable,
}


struct Clause {
    literals: HashMap<String, bool>,
}

impl Clause {
    pub fn add(&mut self, variable: &str, negated: bool) {
         self.literals.insert(String::from_str(variable), negated);
    }

    pub fn new() -> Clause {
        Clause { literals: HashMap::new() }    
    }

    pub fn test(&self, variables: &Vec<String>, assignments: &Vec<bool>) -> bool {

        for (variable, assignment) in variables.iter().zip(assignments.iter()) {
            match self.literals.get(variable) {
                Some(&negated) => {
                    if (negated && !*assignment) || (!negated && *assignment) {
                        return true;
                    }
                },
                None => {},
            }
        }
        return false;
    }
}

struct Solver {
    clauses: Vec<Clause>,
    solutions: Vec<Vec<bool>>,
    variables: Vec<String>,
}


impl Solver {
    pub fn new() -> Solver {
        Solver {
            variables: Vec::new(),
            clauses: Vec::new(),
            solutions: Vec::new()
        }   
    }
    
    pub fn solve(&mut self) -> Satisfiability {
        self.variables.sort();
        self.variables.dedup();
        let length = self.variables.len();
        self.recurse(0, Vec::with_capacity(length));

        if self.solutions.len() == 0 {
            NotSatisfiable
        } else {
            Satisfiable(self.solutions.clone())
        }
    }

    fn recurse(&mut self, index: uint, mut assignments: Vec<bool>) {
        if index == self.variables.len() {
            for clause in self.clauses.iter() {
                if !clause.test(&self.variables, &assignments) {
                    return;
                }
            }

            self.solutions.push(assignments);
        } else {
            let mut copy = assignments.clone();

            assignments.push(true);
            copy.push(false);
            
            self.recurse(index + 1, assignments);
            self.recurse(index + 1, copy);
        }
    }

    pub fn add(&mut self, clause: Clause) {
        for key in clause.literals.keys() {
            self.variables.push(key.clone());
        }
        self.clauses.push(clause);  
    }    
}


fn main() {
    let mut solver = Solver::new();
    
    let mut clause1 = Clause::new();
    clause1.add("A", false);
    clause1.add("B", false);
    clause1.add("C", true);
    
    solver.add(clause1);
    
    let mut clause2 = Clause::new();
    clause2.add("B", false);
    clause2.add("C", false);
    
    solver.add(clause2);
    
    let mut clause3 = Clause::new();

    clause3.add("B", true);

    solver.add(clause3);

    let mut clause4 = Clause::new();
   
    clause4.add("A", true);
    clause4.add("C", false);

    solver.add(clause4);
    
    match solver.solve() {
        Satisfiable(solutions) => {
            println!("{}", solutions);
        }
        NotSatisfiable => {
            println!("Not Satisfiable!");
        }
    }
}
