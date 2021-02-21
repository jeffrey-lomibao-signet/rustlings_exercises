use libraries::largest;

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let largest = largest::find_largest(&number_list);

  println!("The largest number is {}", largest);

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

  let largest = largest::find_largest(&number_list);
  println!("The largest number is {}", largest);
}
