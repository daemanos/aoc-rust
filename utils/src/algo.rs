use std::iter;

pub fn fixpoint<T, F>(start: T, succ: F) -> Vec<T>
where T: Eq,
      F: Fn(&T) -> T,
{
    iter::successors(Some(start), |prev| {
        let next = succ(prev);
        if *prev == next {
            None
        } else {
            Some(next)
        }
    }).collect()
}
