struct FilterCondition<T> {
    condition: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        item == &self.condition
    }
}

fn custom_filter<T>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq,
    T: Clone,
{
    collection
        .iter()
        .filter(|item| condition.is_match(item))
        .cloned()
        .collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let filter_condition = FilterCondition { condition: 3 };

    let filtered_numbers = custom_filter(&numbers, &filter_condition);

    println!("Filtered Numbers: {:?}", filtered_numbers);
}
