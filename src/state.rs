use vec::*;
use TotalMemory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeResult {
    Success,
    Full,
}

pub struct State<Ins> {
    /// The limit of how much memory is allowed to be used.
    max_size: usize,
    /// The current memory used.
    size: usize,
    /// Instructions are popped from this stack to be executed and this stack is not directly accessed.
    exe_stack: Vec<Ins>,
    /// This is a stack for handling instructions as data.
    ins_stack: Vec<Ins>,
    /// This is a stack for integers.
    int_stack: Vec<i64>,
    /// This is a stack for floats.
    float_stack: Vec<f64>,
    /// This is a stack for instruction vectors.
    ins_vec_stack: Vec<TrackedVec<Ins>>,
    /// This is a stack for integer vectors.
    int_vec_stack: Vec<TrackedVec<i64>>,
    /// This is a stack for float vectors.
    float_vec_stack: Vec<TrackedVec<f64>>,
}

impl<Ins> State<Ins> {
    pub fn new(max_size: usize) -> State<Ins> {
        State {
            max_size: max_size,
            size: 0,
            exe_stack: Vec::new(),
            ins_stack: Vec::new(),
            int_stack: Vec::new(),
            float_stack: Vec::new(),
            ins_vec_stack: Vec::new(),
            int_vec_stack: Vec::new(),
            float_vec_stack: Vec::new(),
        }
    }
}

impl<Ins> State<Ins>
    where Ins: TotalMemory
{
    pub fn push_exe(&mut self, ins: Ins) -> SizeResult {
        let size = ins.total_memory();
        if size + self.size > self.max_size {
            SizeResult::Full
        } else {
            self.size += size;
            self.exe_stack.push(ins);
            SizeResult::Success
        }
    }

    pub fn push_ins(&mut self, ins: Ins) -> SizeResult {
        let size = ins.total_memory();
        if size + self.size > self.max_size {
            SizeResult::Full
        } else {
            self.size += size;
            self.ins_stack.push(ins);
            SizeResult::Success
        }
    }

    pub fn push_int(&mut self, int: i64) -> SizeResult {
        let size = int.total_memory();
        if size + self.size > self.max_size {
            SizeResult::Full
        } else {
            self.size += size;
            self.int_stack.push(int);
            SizeResult::Success
        }
    }

    pub fn push_float(&mut self, float: f64) -> SizeResult {
        let size = float.total_memory();
        if size + self.size > self.max_size {
            SizeResult::Full
        } else {
            self.size += size;
            self.float_stack.push(float);
            SizeResult::Success
        }
    }

    pub fn push_ins_vec(&mut self, ins_vec: TrackedVec<Ins>) -> SizeResult {
        let size = ins_vec.total_memory();
        if size + self.size > self.max_size {
            SizeResult::Full
        } else {
            self.size += size;
            self.ins_vec_stack.push(ins_vec);
            SizeResult::Success
        }
    }

    pub fn push_int_vec(&mut self, int_vec: TrackedVec<i64>) -> SizeResult {
        let size = int_vec.total_memory();
        if size + self.size > self.max_size {
            SizeResult::Full
        } else {
            self.size += size;
            self.int_vec_stack.push(int_vec);
            SizeResult::Success
        }
    }

    pub fn push_float_vec(&mut self, float_vec: TrackedVec<f64>) -> SizeResult {
        let size = float_vec.total_memory();
        if size + self.size > self.max_size {
            SizeResult::Full
        } else {
            self.size += size;
            self.float_vec_stack.push(float_vec);
            SizeResult::Success
        }
    }
}
