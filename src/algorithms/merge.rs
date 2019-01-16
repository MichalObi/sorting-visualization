//! [Merge sort](https://en.wikipedia.org/wiki/Merge_sort)

use super::{Algorithm, Array};

/// [Merge sort](https://en.wikipedia.org/wiki/Merge_sort)
pub struct MergeSort;

impl Algorithm for MergeSort {
  fn sort(&self, array: Array) {
      let len = array.len();
      let last = len as usize - 1; // 99 in our case
      self.split(&array, 0, last);
  }

  fn name(&self) -> String {
    "Merge sort".to_string()
  }
}

impl MergeSort {
    fn split(&self, array: &Array, l: usize, r: usize) { // [0..99], 0, 99
        if  l < r { // if l and r are equal do not proceed
            // get index in the middle of array + overflow optimization ( becouse you divide large r-l)
            // for our case 49 / 24 / 12 ...
            let m: usize = l + (r - l) / 2;

            // recursive calls
            self.split(array, l, m); // half of array from lowest to middle
            self.split(array, m + 1, r); // half of array from middle + 1 to highest (index)

            // if l and r are sorted
            self.merge(array, l, m, r); // merge 2 sorted sub-arrays at once
        }
    }

    #[allow(clippy::needless_range_loop)] // clippy fix for "for loop" over vec
    fn merge(&self, array: &Array, l: usize, m: usize, r: usize) {
      // iteration helpers - indexes for sub-arrays
      let mut i:usize = 0;
      let mut j:usize = 0;

      // index of merge sub-array
      let mut k:usize = l;

      let n1:usize = m - l + 1; // highest index in l
      let n2:usize = r - m; // lowest index in r

      // initial helper vectors (arrays)
      let mut l_arr: Vec<u32> = (0..n1 as u32).collect();
      let mut r_arr: Vec<u32> = (0..n2 as u32).collect();

      //copy to helper array for left
      for i in 0..n1 {
          l_arr[i] = array.get(l + i);
      }

      //copy to helper array for right
      for j in 0..n2 {
          r_arr[j] = array.get(m + 1 + j);
      }

      while i < n1 && j < n2 {
          // merge left sub-array to main array
          if l_arr[i] <= r_arr[j] {
              array.set_color(k, [0.0, 1.0, 0.0, 0.8]);
              array.set(k, l_arr[i]);
              array.wait(5);
              array.reset_color(k);
              i += 1;
          } else {
              // merge right sub-array to main array
              array.set(k, r_arr[j]);
              array.wait(5);
              array.reset_color(k);
              j += 1;
          }
          k += 1;
      }

      // check if any array elements left in left sub-array
      while i < n1 {
          // make current item of array stand out
          array.set_color(k, [0.0, 1.0, 0.0, 0.8]);
          array.set(k, l_arr[i]);
          array.wait(5);
          array.reset_color(k);
          i += 1;
          k += 1;
      }

     // check if any array elements right in left sub-array
      while j < n2 {
          // make current item of array stand out
          array.set_color(k, [0.0, 1.0, 0.0, 0.8]);
          array.set(k, r_arr[j]);
          array.wait(5);
          array.reset_color(k);
          j += 1;
          k += 1;
      }
    }
}
