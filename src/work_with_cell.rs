pub mod work_with_cell {
    use std::collections::{HashSet, HashMap};
    use std::borrow::Borrow;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct InputCellId (usize);


    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ComputeCellId(usize);

    impl Borrow<usize> for ComputeCellId {
        fn borrow(&self) -> &usize {
            &self.0
        }
    }


    pub struct InputCell<T> {
        value: T,
    }


    pub struct ComputeCell<'a, T> {
        value: Option<T>,
        func: Box<dyn 'a + Fn(&[T]) -> T>,
        dependecies: Vec<CellId>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CallbackId();

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum CellId {
        Input(InputCellId),
        Compute(ComputeCellId),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum RemoveCallbackError {
        NonexistentCell,
        NonexistentCallback,
    }

    #[derive(Default)]
    pub struct Reactor<'a, T> {
        all_inputs: Vec<InputCell<T>>,
        all_computes: Vec<ComputeCell<'a, T>>,
        dependecies: HashMap<CellId, HashSet<ComputeCellId>>
    }

    impl<'a, T: Copy + PartialEq + Default> Reactor<'a, T> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn create_input(&mut self, _initial: T) -> InputCellId {
            self.all_inputs.push(InputCell::<T> {
                value: _initial
            });

            InputCellId(self.all_inputs.len() - 1)
        }

        pub fn create_compute<F: 'a + Fn(&[T]) -> T> (
            &mut self,
            _dependecies: &[CellId],
            _compute_func: F,
        ) -> ComputeCellId {
            let curr_id = ComputeCellId(self.all_computes.len());

            self.all_computes.push(ComputeCell::<T> {
                value: None,
                dependecies: _dependecies.to_vec(),
                func: Box::new(_compute_func)
            });

            for &i in _dependecies {
                self.dependecies.entry(i).or_default().insert(curr_id);
            }

            self.change_compute_value(&curr_id);

            curr_id
        }

        pub fn change_compute<F: 'a + Fn(&[T]) -> T> (
            &mut self, 
            _dependecies: &[CellId],
            _compute_func: F,
            cell: ComputeCellId
        ) {
            for i in self.all_computes[cell.0].dependecies.iter() {
                self.dependecies.get_mut(&i).unwrap().remove(&cell);
            }

            for &i in _dependecies {
                self.dependecies.entry(i).or_default().insert(cell);
            }

            self.all_computes[cell.0].func = Box::new(_compute_func);
            self.all_computes[cell.0].dependecies = _dependecies.to_vec();

            self.change_compute_value(&cell);
            self.update_value_in_depth(CellId::Compute(cell));
        }


        fn calc_value(&self, val: CellId) -> Option<T> {
            match val {
                CellId::Input(i) => self.all_inputs.get(i.0).map(|c| c.value),
                CellId::Compute(i) => self.all_computes.get(i.0).map(|c| c.value).unwrap_or(None),
            }
        }

        fn calc_values(&self, arr_val: &[CellId]) -> Result<Vec<T>, CellId> {
            arr_val.iter().map(|&id| self.calc_value(id).ok_or(id)).collect()
        }

        fn change_compute_value(&mut self, cell: &ComputeCellId) {
            let values = self.calc_values(&self.all_computes[cell.0].dependecies);

            if let Ok(val) = values {
                self.all_computes[cell.0].value = Some((self.all_computes[cell.0].func)(&val));
            }
        }

        pub fn change_input(&mut self, cell: InputCellId, new_val: T) {
            self.all_inputs[cell.0].value = new_val;
            self.update_value_in_depth(CellId::Input(cell));
        }


        fn update_value_in_depth(&mut self, start_cell: CellId) {
            let mut queue = Vec::new();
            let mut beg_ind = 0;

            if let Some(vals) = self.dependecies.get(&start_cell) {
                for &i in vals.iter() {
                    queue.push(i);
                }
            }


            while beg_ind != queue.len() {
                let v = queue[beg_ind];

                self.change_compute_value(&v);
                beg_ind += 1;

                if let Some(vals) = self.dependecies.get(&CellId::Compute(v)) {
                    for &i in vals.iter() {
                        queue.push(i);
                    }
                }

            }
        }

        pub fn get_val(&self, cell: CellId) -> Option<T> {
            match cell {
                CellId::Compute(i) => self.all_computes.get(i.0).unwrap().value,
                CellId::Input(i) => Some(self.all_inputs.get(i.0).unwrap().value),
            }
        }

    }
}