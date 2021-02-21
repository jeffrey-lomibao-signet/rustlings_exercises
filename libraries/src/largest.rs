#[cfg(test)]
fn print_type_of<T>(_: T) {
  println!("data type is {}", std::any::type_name::<T>())
}

pub fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  if list.is_empty() {
    panic!("List is empty!");
  }

  let mut largest = list[0];
  #[cfg(test)]
  print_type_of(largest);
  for &number in list {
    if number > largest {
      largest = number;
    }
  }
  largest
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic(expected = "List is empty!")]
  fn find_largest_integer_panics_when_given_an_empty_list() {
    let numbers_list = vec![];
    assert_eq!(0, find_largest(&numbers_list));
  }

  #[test]
  fn find_largest_integer_in_list_of_1() {
    let numbers_list = vec![1];
    assert_eq!(1, find_largest(&numbers_list));
  }

  #[test]
  fn find_largest_integer_in_list_of_1_with_value_of_2() {
    let numbers_list = vec![2];
    assert_eq!(2, find_largest(&numbers_list));
  }

  #[test]
  fn find_largest_integer_in_list_of_2() {
    let numbers_list = vec![2, 3];
    assert_eq!(3, find_largest(&numbers_list));
  }

  #[test]
  fn find_largest_integer_in_list_of_2_first_is_largest() {
    let numbers_list = vec![3, 2];
    assert_eq!(3, find_largest(&numbers_list));
  }

  #[test]
  fn find_largest_integer_in_list_of_2_with_equal_values() {
    let numbers_list = vec![2, 2];
    assert_eq!(2, find_largest(&numbers_list));
  }

  #[test]
  fn find_largest_integer_in_list_of_3() {
    let numbers_list = vec![2, 4, 6];
    assert_eq!(6, find_largest(&numbers_list));
  }

  #[test]
  fn find_largest_integer_in_list_of_many() {
    let numbers_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    assert_eq!(6000, find_largest(&numbers_list));
  }

  #[test]
  fn find_largest_number_in_list_of_many_u32() {
    let numbers_list = vec![102u32, 34, 6000, 89, 54, 2, 43, 8];

    assert_eq!(6000, find_largest(&numbers_list));
  }

  #[test]
  fn find_largest_number_in_list_of_many_char() {
    let char_list = vec!['y', 'm', 'a', 'q'];

    assert_eq!('y', find_largest(&char_list));
  }
}
