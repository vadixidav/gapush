mod simple_instruction;
pub use self::simple_instruction::*;

use std::collections::BTreeSet;
use vec;

use rand::Rng;
use rand::distributions::{Exp, IndependentSample};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Chromosome {
    genes: Vec<PlainOp>,
    crossovers: BTreeSet<usize>,
}

impl Chromosome {
    pub fn new_rand<R: Rng>(rng: &mut R, len: usize, crossovers: usize) -> Chromosome {
        Chromosome {
            genes: rng.gen_iter().take(len).collect(),
            crossovers: (0..crossovers).map(|_| rng.gen_range(0, len)).collect(),
        }
    }

    pub fn mutate<R: Rng>(&mut self, maximum: usize, exp: &Exp, rng: &mut R) {
        let mut index = 0;
        for _ in 0..maximum {
            index += exp.ind_sample(rng) as usize;
            if index >= self.genes.len() {
                return;
            }
            // Determine if we want to insert, remove, mutate, add crossover, or remove crossover.
            match rng.gen_range(0, 5usize) {
                0 => {
                    self.genes.insert(index, rng.gen());
                }
                1 => {
                    self.genes.remove(index);
                    // This could potentially invalidate some crossover-point, so remove any such point.
                    let filters = self.crossovers.iter().cloned().filter(|&n| n >= self.genes.len()).collect::<Vec<_>>();
                    for n in filters {
                        if !self.crossovers.remove(&n) {
                            panic!("Error: Attempted to remove invalid crossover, but it didn't exist.");
                        }
                    }
                }
                2 => {
                    self.genes[index] = rng.gen();
                }
                3 => {
                    self.crossovers.insert(index);
                }
                _ => {
                    if self.crossovers.len() != 0 {
                        let cross_choice = rng.gen_range(0, self.crossovers.len());
                        let cross_choice = self.crossovers.iter().cloned().nth(cross_choice)
                        .unwrap_or_else(|| panic!("Error: Tried to remove random crossover point and failed."));
                        self.crossovers.remove(&cross_choice);
                    }
                }
            }
        }
    }

    pub fn mate(&self, other: &Self) -> Self {
        use std::iter::once;
        let mut its = (self.crossovers.iter().cloned().chain(once(self.genes.len())),
                       other.crossovers.iter().cloned().chain(once(other.genes.len())));
        let mut genes = Vec::new();
        let mut crossovers = BTreeSet::new();
        let mut prev = 0;
        loop {
            // Work on the first chromosome.
            let next = its.0.find(|&n| n > prev);
            if let Some(next) = next {
                genes.extend_from_slice(&self.genes[prev..next]);
                crossovers.insert(next);
                prev = next;
            } else {
                break;
            }

            // Work on the second chromosome.
            let next = its.1.find(|&n| n > prev);
            if let Some(next) = next {
                genes.extend_from_slice(&self.genes[prev..next]);
                crossovers.insert(next);
                prev = next;
            } else {
                break;
            }
        }
        Chromosome {
            genes: genes,
            crossovers: crossovers,
        }
    }

    pub fn gene_len(&self) -> usize {
        self.genes.len()
    }

    pub fn crossover_len(&self) -> usize {
        self.crossovers.len()
    }
}

impl<'a> Into<SimpleInstruction> for &'a Chromosome {
    fn into(self) -> SimpleInstruction {
        SimpleInstruction::BasicBlock(
            vec::TrackedVec::new_from_vec(
                self.genes.iter().cloned().map(SimpleInstruction::PlainOp).collect()).into_iter())
    }
}


