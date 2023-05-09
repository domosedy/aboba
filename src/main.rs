pub mod work_with_cell;
use crate::work_with_cell::work_with_cell::{Reactor, CellId};
use std::cmp::max;

fn main() {
    let mut r: Reactor<i32> = Reactor::new();
    let cell_input = r.create_input(12);
    let cell_input2 = r.create_input(13);


    let cell_compute = r.create_compute(&[CellId::Input(cell_input), CellId::Input(cell_input2)], |a: &[i32]| a[0] - a[1]);

    println!("{}", r.get_val(CellId::Compute(cell_compute)).unwrap());

    r.change_input(cell_input, 13);

    println!("{}", r.get_val(CellId::Compute(cell_compute)).unwrap());

    r.change_compute(&[CellId::Input(cell_input), CellId::Input(cell_input2)], |a: &[i32]| a[0] * a[1], cell_compute);
    println!("{}", r.get_val(CellId::Compute(cell_compute)).unwrap());

    let cell_compute2 = r.create_compute(&[CellId::Input(cell_input), CellId::Compute(cell_compute)], |a: &[i32]| max(a[0], a[1]));
    println!("{}", r.get_val(CellId::Compute(cell_compute2)).unwrap());

    r.change_input(cell_input2, -2);
    println!("{}", r.get_val(CellId::Compute(cell_compute2)).unwrap());
}