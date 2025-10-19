pub fn map<I, O, F: FnMut(I) -> O>(input: Vec<I>, mut function: F) -> Vec<O> {
    let mut mapped = Vec::with_capacity(input.len());
    for value in input {
        mapped.push(function(value));
    }
    mapped
}
