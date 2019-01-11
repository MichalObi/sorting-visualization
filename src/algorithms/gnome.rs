//! [Gnome sort](https://en.wikipedia.org/wiki/Gnome_sort)

use super::{Algorithm, Array};

/// [Gnome sort](https://en.wikipedia.org/wiki/Gnome_sort)
pub struct GnomeSort;

impl Algorithm for GnomeSort {
  fn sort(&self, array: Array) {
    let len = array.len(); // length or array to sort
    let mut i = 0; // initial index of first array element
    while i < len { // while items in array exist

      // color manipulation for visualization purpose
      array.set_color(i, [0.0, 1.0, 0.0, 0.8]);
      array.wait(5);
      array.reset_color(i);

      // if initial index or current item index greater or equal previous item
      if i == 0 || array.get(i) >= array.get(i - 1) {
        i += 1; // this fragment of array is sorted, so go up to the nest index of array
      } else {
        array.swap(i, i - 1); // swap 2 items position (similar to bubble sort)
        i -= 1; // go one step backwards
      }
    }
  }

  fn name(&self) -> String {
    "Gnome sort".to_string()
  }
}
