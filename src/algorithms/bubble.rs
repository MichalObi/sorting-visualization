//! [Bubble sort](https://en.wikipedia.org/wiki/Bubble_sort)

use super::{Algorithm, Array};

/// [Bubble sort](https://en.wikipedia.org/wiki/Bubble_sort)
pub struct BubbleSort;

impl Algorithm for BubbleSort {
  fn sort(&self, array: Array) {
    let len = array.len(); //length of array to be sort
    // the last elem will be sorted if all other elem will be - that's why len - 1
    for i in 0..len -1 {
      let last = len - 1 - i; // current last element of array
      array.set_color(last, [0.0, 1.0, 0.0, 0.8]);

      for j in 0..last { // in second loop sort to max last not sorted element
        array.set_color(j, [0.0, 1.0, 0.0, 0.8]);
        if array.get(j) > array.get(j + 1) { // if current element is greater then next elem ...
          array.swap(j, j + 1); // change position of this two elems
        }
        array.wait(5);
        array.reset_color(j);
      }

      array.reset_color(last);
    }
  }

  fn name(&self) -> String {
    "Bubble sort".to_string()
  }
}
