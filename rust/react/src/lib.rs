use std::collections::{HashMap, HashSet, VecDeque};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: T,
}

struct ComputeCell<'a, T> {
    value: T,
    dependencies: Vec<CellId>,
    compute_func: Box<dyn 'a + Fn(&[T]) -> T>,
    callbacks: HashMap<CallbackId, Box<dyn 'a + FnMut(T)>>,
}

#[derive(Default)]
pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    input_cells: HashMap<InputCellId, InputCell<T>>,
    compute_cells: HashMap<ComputeCellId, ComputeCell<'a, T>>,
    dependencies: HashMap<CellId, HashSet<ComputeCellId>>,
    next_cell_id: usize,
    next_callback_id: usize,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + Default + std::fmt::Debug> Reactor<'a, T> {
    pub fn new() -> Self {
        Self::default()
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let next_cell_id = InputCellId(self.next_cell_id);
        self.next_cell_id += 1;
        self.input_cells
            .insert(next_cell_id, InputCell { value: initial });
        next_cell_id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let values = self.values(dependencies)?;
        let next_cell_id = ComputeCellId(self.next_cell_id);
        self.next_cell_id += 1;
        for &dependency in dependencies {
            self.dependencies
                .entry(dependency)
                .or_default()
                .insert(next_cell_id);
        }
        let compute_cell = ComputeCell {
            value: compute_func(&values),
            dependencies: dependencies.to_vec(),
            compute_func: Box::new(compute_func),
            callbacks: HashMap::new(),
        };
        self.compute_cells.insert(next_cell_id, compute_cell);
        Ok(next_cell_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(id) => self.input_cells.get(&id).map(|cell| cell.value),
            CellId::Compute(id) => self.compute_cells.get(&id).map(|cell| cell.value),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(InputCell { value }) = self.input_cells.get_mut(&id) {
            *value = new_value;
            self.update(CellId::Input(id));
            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let compute_cell = self.compute_cells.get_mut(&id)?;
        let next_callback_id = CallbackId(self.next_callback_id);
        self.next_callback_id += 1;
        compute_cell
            .callbacks
            .insert(next_callback_id, Box::new(callback));
        Some(next_callback_id)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let compute_cell = self
            .compute_cells
            .get_mut(&cell)
            .ok_or(RemoveCallbackError::NonexistentCell)?;
        let _ = compute_cell
            .callbacks
            .remove(&callback)
            .ok_or(RemoveCallbackError::NonexistentCallback)?;
        Ok(())
    }

    fn values(&self, ids: &[CellId]) -> Result<Vec<T>, CellId> {
        ids.iter().map(|&id| self.value(id).ok_or(id)).collect()
    }

    fn update(&mut self, id: CellId) {
        if let Some(compute_cell_ids) = self.dependencies.get(&id) {
            let mut queue = VecDeque::new();
            let mut updated = HashMap::new();
            queue.extend(compute_cell_ids.iter());
            while let Some(compute_cell_id) = queue.pop_front() {
                if let Some(compute_cell_ids) =
                    self.dependencies.get(&CellId::Compute(compute_cell_id))
                {
                    queue.extend(compute_cell_ids.iter());
                }
                let compute_cell = self.compute_cells.get(&compute_cell_id).unwrap();
                let values = self.values(&compute_cell.dependencies).unwrap();
                let value = (compute_cell.compute_func)(&values);
                if compute_cell.value != value {
                    let compute_cell = self.compute_cells.get_mut(&compute_cell_id).unwrap();
                    updated.entry(compute_cell_id).or_insert(compute_cell.value);
                    compute_cell.value = value;
                }
            }
            for (compute_cell_id, old_value) in updated {
                let compute_cell = self.compute_cells.get_mut(&compute_cell_id).unwrap();
                if old_value != compute_cell.value {
                    for (_, callback) in &mut compute_cell.callbacks {
                        callback(compute_cell.value);
                    }
                }
            }
        }
    }
}
