mod simple_instruction;
pub use self::simple_instruction::*;

use std::collections::BTreeSet;
use vec;

use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Chromosome {
    genes: Vec<PlainOp>,
    crossovers: BTreeSet<usize>,
}

impl Chromosome {
    pub fn new_rand<R: Rng>(rng: &mut R, len: usize, crossovers: usize) -> Chromosome {
        Chromosome {
            genes: rng.gen_iter().collect(),
            crossovers: (0..crossovers).map(|_| rng.gen_range(0, len)).collect(),
        }
    }

    pub fn mate(&self, other: &Self) -> Self {
        let mut its = (self.crossovers.iter(), other.crossovers.iter());
        let mut genes = Vec::new();
        let mut crossovers = BTreeSet::new();
        let mut prev = 0;
        loop {
            // Work on the first chromosome.
            let next = its.0.find(|&&n| n > prev);
            if let Some(&next) = next {
                genes.extend_from_slice(&self.genes[prev..next]);
                crossovers.insert(next);
                prev = next;
            } else {
                break;
            }

            // Work on the second chromosome.
            let next = its.1.find(|&&n| n > prev);
            if let Some(&next) = next {
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


