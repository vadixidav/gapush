use vec::*;
use TotalMemory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeError {
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
    pub fn push_exe(&mut self, ins: Ins) -> Result<(), SizeError> {
        let size = ins.total_memory();
        if size + self.size > self.max_size {
            Err(SizeError::Full)
        } else {
            self.size += size;
            self.exe_stack.push(ins);
            Ok(())
        }
    }

    pub fn push_ins(&mut self, ins: Ins) -> Result<(), SizeError> {
        let size = ins.total_memory();
        if size + self.size > self.max_size {
            Err(SizeError::Full)
        } else {
            self.size += size;
            self.ins_stack.push(ins);
            Ok(())
        }
    }

    pub fn push_int(&mut self, int: i64) -> Result<(), SizeError> {
        let size = int.total_memory();
        if size + self.size > self.max_size {
            Err(SizeError::Full)
        } else {
            self.size += size;
            self.int_stack.push(int);
            Ok(())
        }
    }

    pub fn push_float(&mut self, float: f64) -> Result<(), SizeError> {
        let size = float.total_memory();
        if size + self.size > self.max_size {
            Err(SizeError::Full)
        } else {
            self.size += size;
            self.float_stack.push(float);
            Ok(())
        }
    }

    pub fn push_ins_vec(&mut self, ins_vec: TrackedVec<Ins>) -> Result<(), SizeError> {
        let size = ins_vec.total_memory();
        if size + self.size > self.max_size {
            Err(SizeError::Full)
        } else {
            self.size += size;
            self.ins_vec_stack.push(ins_vec);
            Ok(())
        }
    }

    pub fn push_int_vec(&mut self, int_vec: TrackedVec<i64>) -> Result<(), SizeError> {
        let size = int_vec.total_memory();
        if size + self.size > self.max_size {
            Err(SizeError::Full)
        } else {
            self.size += size;
            self.int_vec_stack.push(int_vec);
            Ok(())
        }
    }

    pub fn push_float_vec(&mut self, float_vec: TrackedVec<f64>) -> Result<(), SizeError> {
        let size = float_vec.total_memory();
        if size + self.size > self.max_size {
            Err(SizeError::Full)
        } else {
            self.size += size;
            self.float_vec_stack.push(float_vec);
            Ok(())
        }
    }

    pub fn pop_exe(&mut self) -> Option<Ins> {
        if let Some(e) = self.exe_stack.pop() {
            self.size -= e.total_memory();
            Some(e)
        } else {
            None
        }
    }

    pub fn pop_ins(&mut self) -> Option<Ins> {
        if let Some(e) = self.ins_stack.pop() {
            self.size -= e.total_memory();
            Some(e)
        } else {
            None
        }
    }

    pub fn pop_int(&mut self) -> Option<i64> {
        if let Some(e) = self.int_stack.pop() {
            self.size -= e.total_memory();
            Some(e)
        } else {
            None
        }
    }

    pub fn pop_float(&mut self) -> Option<f64> {
        if let Some(e) = self.float_stack.pop() {
            self.size -= e.total_memory();
            Some(e)
        } else {
            None
        }
    }

    pub fn pop_ins_vec(&mut self) -> Option<TrackedVec<Ins>> {
        if let Some(e) = self.ins_vec_stack.pop() {
            self.size -= e.total_memory();
            Some(e)
        } else {
            None
        }
    }

    pub fn pop_int_vec(&mut self) -> Option<TrackedVec<i64>> {
        if let Some(e) = self.int_vec_stack.pop() {
            self.size -= e.total_memory();
            Some(e)
        } else {
            None
        }
    }

    pub fn pop_float_vec(&mut self) -> Option<TrackedVec<f64>> {
        if let Some(e) = self.float_vec_stack.pop() {
            self.size -= e.total_memory();
            Some(e)
        } else {
            None
        }
    }

    pub fn rot_exe(&mut self, pos: usize) -> bool {
        let len = self.exe_stack.len();
        if pos < len {
            let e = self.exe_stack.remove(len - pos - 1);
            self.exe_stack.push(e);
            true
        } else {
            false
        }
    }

    pub fn rot_ins(&mut self, pos: usize) -> bool {
        let len = self.ins_stack.len();
        if pos < len {
            let e = self.ins_stack.remove(len - pos - 1);
            self.ins_stack.push(e);
            true
        } else {
            false
        }
    }

    pub fn rot_int(&mut self, pos: usize) -> bool {
        let len = self.int_stack.len();
        if pos < len {
            let e = self.int_stack.remove(len - pos - 1);
            self.int_stack.push(e);
            true
        } else {
            false
        }
    }

    pub fn rot_float(&mut self, pos: usize) -> bool {
        let len = self.float_stack.len();
        if pos < len {
            let e = self.float_stack.remove(len - pos - 1);
            self.float_stack.push(e);
            true
        } else {
            false
        }
    }

    pub fn rot_ins_vec(&mut self, pos: usize) -> bool {
        let len = self.float_stack.len();
        if pos < len {
            let e = self.ins_vec_stack.remove(len - pos - 1);
            self.ins_vec_stack.push(e);
            true
        } else {
            false
        }
    }

    pub fn rot_int_vec(&mut self, pos: usize) -> bool {
        let len = self.int_vec_stack.len();
        if pos < len {
            let e = self.int_vec_stack.remove(len - pos - 1);
            self.int_vec_stack.push(e);
            true
        } else {
            false
        }
    }

    pub fn rot_float_vec(&mut self, pos: usize) -> bool {
        let len = self.float_vec_stack.len();
        if pos < len {
            let e = self.float_vec_stack.remove(len - pos - 1);
            self.float_vec_stack.push(e);
            true
        } else {
            false
        }
    }

    pub fn copy_exe(&self, pos: usize) -> Option<Ins>
        where Ins: Clone
    {
        let len = self.exe_stack.len();
        if pos < len {
            unsafe {Some(self.exe_stack.get_unchecked(len - pos - 1).clone())}
        } else {
            None
        }
    }

    pub fn copy_ins(&self, pos: usize) -> Option<Ins>
        where Ins: Clone
    {
        let len = self.ins_stack.len();
        if pos < len {
            unsafe {Some(self.ins_stack.get_unchecked(len - pos - 1).clone())}
        } else {
            None
        }
    }

    pub fn copy_int(&self, pos: usize) -> Option<i64> {
        let len = self.int_stack.len();
        if pos < len {
            unsafe {Some(self.int_stack.get_unchecked(len - pos - 1).clone())}
        } else {
            None
        }
    }

    pub fn copy_float(&self, pos: usize) -> Option<f64> {
        let len = self.float_stack.len();
        if pos < len {
            unsafe {Some(self.float_stack.get_unchecked(len - pos - 1).clone())}
        } else {
            None
        }
    }

    pub fn copy_ins_vec(&self, pos: usize) -> Option<TrackedVec<Ins>>
        where Ins: Clone
    {
        let len = self.ins_vec_stack.len();
        if pos < len {
            unsafe {Some(self.ins_vec_stack.get_unchecked(len - pos - 1).clone())}
        } else {
            None
        }
    }

    pub fn copy_int_vec(&self, pos: usize) -> Option<TrackedVec<i64>> {
        let len = self.int_vec_stack.len();
        if pos < len {
            unsafe {Some(self.int_vec_stack.get_unchecked(len - pos - 1).clone())}
        } else {
            None
        }
    }

    pub fn copy_float_vec(&self, pos: usize) -> Option<TrackedVec<f64>> {
        let len = self.float_vec_stack.len();
        if pos < len {
            unsafe {Some(self.float_vec_stack.get_unchecked(len - pos - 1).clone())}
        } else {
            None
        }
    }

    pub fn push_ins_to_vec(&mut self, ins: Ins) -> Result<(), SizeError> {
        let size = ins.total_memory();
        if size + self.size > self.max_size {
            Err(SizeError::Full)
        } else {
            if let Some(ref mut v) = self.ins_vec_stack.last_mut() {
                self.size += size;
                v.push(ins);
            }
            Ok(())
        }
    }

    pub fn push_int_to_vec(&mut self, int: i64) -> Result<(), SizeError> {
        let size = int.total_memory();
        if size + self.size > self.max_size {
            Err(SizeError::Full)
        } else {
            if let Some(ref mut v) = self.int_vec_stack.last_mut() {
                self.size += size;
                v.push(int);
            }
            Ok(())
        }
    }

    pub fn push_float_to_vec(&mut self, float: f64) -> Result<(), SizeError> {
        let size = float.total_memory();
        if size + self.size > self.max_size {
            Err(SizeError::Full)
        } else {
            if let Some(ref mut v) = self.float_vec_stack.last_mut() {
                self.size += size;
                v.push(float);
            }
            Ok(())
        }
    }

    pub fn pop_ins_from_vec(&mut self) -> Option<Ins> {
        if let Some(ins) = self.ins_vec_stack.last_mut().and_then(TrackedVec::pop) {
            self.size -= ins.total_memory();
            Some(ins)
        } else {
            None
        }
    }

    pub fn pop_int_from_vec(&mut self) -> Option<i64> {
        if let Some(int) = self.int_vec_stack.last_mut().and_then(TrackedVec::pop) {
            self.size -= int.total_memory();
            Some(int)
        } else {
            None
        }
    }

    pub fn pop_float_from_vec(&mut self) -> Option<f64> {
        if let Some(float) = self.float_vec_stack.last_mut().and_then(TrackedVec::pop) {
            self.size -= float.total_memory();
            Some(float)
        } else {
            None
        }
    }

    pub fn get_ins_from_vec(&self, ix: usize) -> Option<Ins>
        where Ins: Clone
    {
        self.ins_vec_stack.last().and_then(|v| v.get(ix)).cloned()
    }

    pub fn get_int_from_vec(&self, ix: usize) -> Option<i64> {
        self.int_vec_stack.last().and_then(|v| v.get(ix)).cloned()
    }

    pub fn get_float_from_vec(&self, ix: usize) -> Option<f64> {
        self.float_vec_stack.last().and_then(|v| v.get(ix)).cloned()
    }

    pub fn write_ins_to_vec(&mut self, ix: usize, ins: Ins) {
        if let Some(e) = self.ins_vec_stack.last_mut().and_then(|v| v.get_mut(ix)) {
            self.size -= e.total_memory();
            self.size += ins.total_memory();
            *e = ins;
        }
    }

    pub fn write_int_to_vec(&mut self, ix: usize, int: i64) {
        if let Some(e) = self.int_vec_stack.last_mut().and_then(|v| v.get_mut(ix)) {
            self.size -= e.total_memory();
            self.size += int.total_memory();
            *e = int;
        }
    }

    pub fn write_float_to_vec(&mut self, ix: usize, float: f64) {
        if let Some(e) = self.float_vec_stack.last_mut().and_then(|v| v.get_mut(ix)) {
            self.size -= e.total_memory();
            self.size += float.total_memory();
            *e = float;
        }
    }
}
