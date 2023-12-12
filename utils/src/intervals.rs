use num_traits::PrimInt;
use std::cmp;
use std::collections::BTreeMap;
use std::iter::Iterator;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Interval<T: PrimInt>(pub T, pub T);

impl<T> Interval<T>
where T: PrimInt
{
    pub fn merge(self, other: Self) -> Self {
        let s = cmp::min(self.0, other.0);
        let e = cmp::max(self.1, other.1);

        Self(s, e)
    }

    pub fn intersection(self, other: Self) -> Self {
        let s = cmp::max(self.0, other.0);
        let e = cmp::min(self.1, other.1);

        Self(s, e)
    }

    pub fn overlaps(self, other: Self) -> bool {
        let s = cmp::max(self.0, other.0);
        let e = cmp::min(self.1, other.1);

        s < e
    }

    pub fn diff(self, other: Self) -> (Option<Self>, Option<Self>) {
        let left = if self.0 < other.0 {
            Some(Self(self.0, cmp::min(self.1, other.0)))
        } else {
            None
        };

        let right = if self.1 > other.1 {
            Some(Self(cmp::max(self.0, other.1), self.1))
        } else {
            None
        };

        (left, right)
    }

    // Note: cannot implement IntoIterator or similar b/c Step trait is unavailable on stable
    pub fn range(self) -> Range<T> {
        Range { start: self.0, end: self.1 }
    }
}

pub struct IntervalTree<T: PrimInt>(BTreeMap<T, Interval<T>>);

//pub struct Iter<'a, T>
//where T: 'a + PrimInt
//{
//    stack: Vec<&'a Node<T>>,
//    curr: Option<&'a Node<T>>,
//}
//
//impl<'a, T> Iterator for Iter<'a, T>
//where T: PrimInt
//{
//    type Item = &'a Interval<T>;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        while let Some(curr) = &self.curr {
//            self.stack.push(curr);
//            self.curr = curr.left.as_ref().map(Box::borrow);
//        }
//
//        self.stack.pop().map(|curr| {
//            self.curr = curr.right.as_ref().map(Box::borrow);
//            &curr.value
//        })
//    }
//}

impl<T> IntervalTree<T>
where T: PrimInt
{
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn singleton(int: Interval<T>) -> Self {
        Self(BTreeMap::from([(int.1, int)]))
    }

    pub fn from<const N: usize>(ints: [Interval<T>; N]) -> Self {
        let ints = ints.iter().map(|&int| (int.1, int));
        Self(BTreeMap::from_iter(ints))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, new: Interval<T>) {
        let keys = self.overlap_keys(new);

        let new = keys.iter().fold(new, |acc, key| {
            let val = self.0.remove(&key).unwrap();
            acc.merge(val)
        });

        self.insert_raw(new);
    }

    pub fn remove(&mut self, rem: Interval<T>) {
        let keys = self.overlap_keys(rem);

        for key in keys {
            let val = self.0.remove(&key).unwrap();
            let (left, right) = val.diff(rem);

            if let Some(left) = left {
                self.insert_raw(left);
            }
            if let Some(right) = right {
                self.insert_raw(right);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Interval<T>> {
        self.0.values()
    }

    pub fn pop_first(&mut self) -> Option<Interval<T>> {
        self.0.pop_first().map(|(_, v)| v)
    }

    pub fn pop_last(&mut self) -> Option<Interval<T>> {
        self.0.pop_last().map(|(_, v)| v)
    }

    fn overlap_keys(&mut self, int: Interval<T>) -> Vec<T> {
        self.0.range(int.0..)
            .take_while(|(_, val)| val.overlaps(int))
            .map(|(e, _)| e)
            .copied()
            .collect()
    }

    fn insert_raw(&mut self, new: Interval<T>) {
        self.0.insert(new.1, new);
    }
}

//struct Node<T: PrimInt> {
//    value: Interval<T>,
//    left: Option<Box<Self>>,
//    right: Option<Box<Self>>,
//}
//
//impl<T> Node<T>
//where T: PrimInt
//{
//    fn leaf(value: Interval<T>) -> Self {
//        Self { value, left: None, right: None }
//    }
//
//    fn branch(value: Interval<T>, left: Option<Self>, right: Option<Self>) -> Self {
//        let left = left.map(Box::new);
//        let right = right.map(Box::new);
//        Self { value, left, right }
//    }
//
//    fn set_left(&mut self, new: Interval<T>) {
//        self.left = Some(Box::new(Self::leaf(new)));
//    }
//
//    fn set_right(&mut self, new: Interval<T>) {
//        self.right = Some(Box::new(Self::leaf(new)));
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_merge_left() {
        let i1 = Interval(0_u32, 4_u32);
        let i2 = Interval(3_u32, 5_u32);
        assert_eq!(Interval(0, 5), i1.merge(i2));
        assert_eq!(Interval(0, 5), i2.merge(i1));
    }

    #[test]
    fn interval_merge_right() {
        let i1 = Interval(0_u32, 4_u32);
        let i2 = Interval(2_u32, 6_u32);
        assert_eq!(Interval(0, 6), i1.merge(i2));
        assert_eq!(Interval(0, 6), i2.merge(i1));
    }

    #[test]
    fn interval_merge_subset() {
        let i1 = Interval(0_u32, 4_u32);
        let i2 = Interval(1_u32, 3_u32);
        assert_eq!(i1, i1.merge(i2));
        assert_eq!(i1, i2.merge(i1));
    }

    #[test]
    fn interval_diff_onesided() {
        let i1 = Interval(1_u32, 4_u32);
        let i2 = Interval(3_u32, 6_u32);
        assert_eq!((Some(Interval(1, 3)), None), i1.diff(i2));
        assert_eq!((None, Some(Interval(4, 6))), i2.diff(i1));
    }

    #[test]
    fn interval_diff_twosided() {
        let i1 = Interval(3_u32, 7_u32);
        let i2 = Interval(1_u32, 8_u32);
        assert_eq!((None, None), i1.diff(i2));
        assert_eq!((Some(Interval(1, 3)), Some(Interval(7, 8))), i2.diff(i1));
    }

    fn interval_tree_insert_overlap(i1: Interval<u32>, i2: Interval<u32>) {
        let mut tr = IntervalTree::new();
        tr.insert(i1);
        tr.insert(i2);

        assert_eq!(Some(i1.merge(i2)), tr.pop_first());
    }

    #[test]
    fn interval_tree_insert_overlap_left() {
        interval_tree_insert_overlap(Interval(2, 4), Interval(0, 3));
    }

    #[test]
    fn interval_tree_insert_overlap_right() {
        interval_tree_insert_overlap(Interval(0, 3), Interval(2, 4));
    }

    #[test]
    fn interval_tree_insert_overlap_both() {
        interval_tree_insert_overlap(Interval(3, 5), Interval(1, 9));
        interval_tree_insert_overlap(Interval(1, 9), Interval(3, 5));
    }

    #[test]
    fn interval_tree_remove_overlap() {
        let mut tr: IntervalTree<u32> = IntervalTree::from([
            Interval(0, 3),
            Interval(5, 7),
            Interval(9, 12),
        ]);

        tr.remove(Interval(2, 10));
        assert_eq!(Some(Interval(0, 2)), tr.pop_first());
        assert_eq!(Some(Interval(10, 12)), tr.pop_first());
    }

    #[test]
    fn interval_tree_insert_disjoint() {
        let i1 = Interval(7_u32, 10_u32);
        let i2 = Interval(3_u32, 5_u32);

        let mut tr = IntervalTree::new();
        tr.insert(i1);
        tr.insert(i2);

        assert_eq!(Some(i2), tr.pop_first());
        assert_eq!(Some(i1), tr.pop_first());
    }

    #[test]
    fn interval_tree_iter() {
        let i1 = Interval(0_u32, 3_u32);
        let i2 = Interval(4_u32, 7_u32);
        let i3 = Interval(7_u32, 10_u32);

        let mut tr = IntervalTree::new();
        tr.insert(i1);
        tr.insert(i2);
        tr.insert(i3);

        let is: Vec<Interval<_>> = tr.iter().copied().collect();
        assert_eq!(vec![i1, i2, i3], is);
    }
}
