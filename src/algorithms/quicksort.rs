//! [Quicksort](https://en.wikipedia.org/wiki/Quicksort)

use super::{Algorithm, Array};

/// [Quicksort](https://en.wikipedia.org/wiki/Quicksort)
pub struct Quicksort;

impl Algorithm for Quicksort {
  fn sort(&self, array: Array) {
      // isize = 8 bytes for x64 target
    self.sort_slice(&array, 0, array.len() as isize - 1); // 0 is min 100 is max
  }

  fn name(&self) -> String {
    "Quicksort".to_string()
  }
}

// divide and conquer type of algo
impl Quicksort {
  #[allow(clippy::range_minus_one)]
  // current array, lowest and highest - this fn has recursive calls
  fn sort_slice(&self, array: &Array, low: isize, high: isize) {
    // if low high array will be sorted, and no need for another recursive call
    if low < high {
      // pick a pivot - key process of Quicksort
      let pivot = self.partition(array, low, high);
      print!("pivot {:?}", pivot);

      // set color from lowest to selected pivot
      for i in low..=pivot - 1 {
        array.set_color(i as usize, [0.0, 1.0, 0.0, 0.3]);
      }

      // set color for pivot
      array.set_color(pivot as usize, [1.0, 0.0, 0.0, 1.0]);

      // set color from pivot to selected highest
      for i in pivot + 1..=high {
        array.set_color(i as usize, [0.0, 0.0, 1.0, 0.3]);
      }

      // recursive sort left from pivot
      self.sort_slice(array, low, pivot - 1);

      // sort sort left from pivot
      self.sort_slice(array, pivot + 1, high);

        // reset color from lowest to selected pivot
      for i in low..=pivot - 1 {
        array.reset_color(i as usize);
      }

      // reset color for pivot
      array.reset_color(pivot as usize);

       // reset color from pivot to selected highest
      for i in pivot + 1..=high {
        array.reset_color(i as usize);
      }
    }
  }

  fn partition(&self, array: &Array, low: isize, high: isize) -> isize {
    let pivot = array.get(high as usize);
    let mut i = low;
    for j in low..high {
      if array.get(j as usize) <= pivot {
        array.swap(i as usize, j as usize);
        i += 1; // increment lowest
      }
      array.wait(15);
    }
    array.swap(i as usize, high as usize);
    i
  }
}
