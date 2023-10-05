struct FilterCondition<T>{
  value: T,
}

impl <T : PartialEq> FilterCondition <T>{
  fn is_match(&self, item: &T)-> bool{
    &self.value == item
  }
}

fn custom_filter<T>(collection: Vec<T>, filter_condition: &FilterCondition<T>) -> Vec<T> where T: PartialEq + Clone{
  collection
    .into_iter()
    .filter(|item| filter_condition.is_match(item))
    .collect()
}

fn main() {
  let original_vector = vec![1,2,4,7,3,6,9,11,5];

  let filter_condition = FilterCondition{value: 5};

  let filtered_vector = custom_filter(original_vector.clone(), &filter_condition);

  println!("original vector: {:?}", original_vector);
  println!("filtered vector: {:?}", filtered_vector);
}
