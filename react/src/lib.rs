struct Node<T> {
    value: T,
    children: Vec<usize>,
    parents: Vec<usize>,
}

struct SimpleDAG<T>(Vec<Node<T>>);

impl<T> SimpleDAG<T> {
    fn new() -> SimpleDAG<T> {
        Self(Vec::new())
    }

    fn add(&mut self, value: T) -> usize {
        self.0.push(Node {
            value,
            children: Vec::new(),
            parents: Vec::new(),
        });
        self.0.len() - 1
    }

    fn get(&self, id: usize) -> Option<&T> {
        self.0.get(id).map(|node| &node.value)
    }

    fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.0.get_mut(id).map(|node| &mut node.value)
    }

    fn link(&mut self, parent: usize, child: usize) -> Result<(), &'static str> {
        // enforce the acyclic nature by simply allowing to link
        // only from early nodes to later nodes
        if parent >= child {
            return Err("`a` must be lte `b`");
        }

        self.0
            .get_mut(parent)
            .expect("`a` does not exist in children")
            .children
            .push(child);
        self.0
            .get_mut(child)
            .expect("`a` does not exist in children")
            .parents
            .push(parent);

        Ok(())
    }

    fn children(&self, a: usize) -> &Vec<usize> {
        &self.0[a].children
    }
}

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID();

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

impl CellID {
    fn raw_id(&self) -> usize {
        match self {
            Self::Input(id) => id.0,
            Self::Compute(id) => id.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

// More on storing functions refs
// https://stackoverflow.com/a/52934680/2224875
// https://www.reddit.com/r/rust/comments/9vnumk/what_are_the_drawbacks_of_using_a_boxed_closure/e9dpkvq?utm_source=share&utm_medium=web2x&context=3
// https://godbolt.org/z/qTobzqesE
enum Cell<T> {
    Input(T),
    Compute(T, Vec<CellID>, Box<dyn Fn(&[T]) -> T>),
}

pub struct Reactor<T>(SimpleDAG<Cell<T>>);

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Self(SimpleDAG::new())
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        InputCellID(self.0.add(Cell::Input(initial)))
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
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellID],
        f: F,
    ) -> Result<ComputeCellID, CellID> {
        let inputs = self.get_values(dependencies)?;

        let id = self
            .0
            .add(Cell::Compute(f(&inputs), dependencies.into(), Box::new(f)));

        for d in dependencies {
            self.0.link(d.raw_id(), id).unwrap();
        }

        Ok(ComputeCellID(id))
    }

    fn get_values(&self, dependencies: &[CellID]) -> Result<Vec<T>, CellID> {
        dependencies
            .iter()
            .map(|&id| self.value(id).ok_or(id))
            .collect()
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(id) => match self.0.get(id.0) {
                Some(Cell::Input(value)) => Some(*value),
                _ => None,
            },
            CellID::Compute(id) => match self.0.get(id.0) {
                Some(Cell::Compute(value, _, _)) => Some(*value),
                _ => None,
            },
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if let Some(Cell::Input(value)) = self.0.get_mut(id.0) {
            *value = new_value;
            self.propagate_change(id);
            true
        } else {
            false
        }
    }

    fn propagate_change(&mut self, id: InputCellID) -> Option<()> {
        // POC with a priority queue done as a simple vector
        // let mut cell = self.0.get_mut(id.0)?;
        let mut pq = Vec::new();
        pq.push(id.0);

        while let Some(id) = pq.pop() {
            for c in self.0.children(id) {
                pq.push(*c);
            }
            pq.sort_by(|a, b| b.cmp(a));

            // does nothing if it's not a compute cell
            self.update_compute(id);
        }
        None
    }

    fn update_compute(&mut self, id: usize) -> Option<()> {
        let inputs = if let Cell::Compute(_, dependencies, _) = self.0.get(id)? {
            self.get_values(dependencies).ok()
        } else {
            None
        }?;

        if let Cell::Compute(value, _, f) = self.0.get_mut(id)? {
            *value = f(&inputs);
            Some(())
        } else {
            None
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
    pub fn add_callback<F1: FnMut(T)>(
        &mut self,
        _id: ComputeCellID,
        _callback: F1,
    ) -> Option<CallbackID> {
        unimplemented!()
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        unimplemented!(
            "Remove the callback identified by the CallbackID {:?} from the cell {:?}",
            callback,
            cell,
        )
    }
}
