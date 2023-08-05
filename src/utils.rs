pub fn pop_front<T>(vec: &mut Vec<T>) -> Option<T> {
    vec.rotate_left(1);
    vec.pop()
}
