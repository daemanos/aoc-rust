use num_traits::PrimInt;
use std::borrow::{Borrow, BorrowMut};
use std::cmp;
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

pub struct IntervalTree<T: PrimInt>(Option<Node<T>>);

pub struct Iter<'a, T>
where T: 'a + PrimInt
{
    stack: Vec<&'a Node<T>>,
    curr: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T>
where T: PrimInt
{
    type Item = &'a Interval<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(curr) = &self.curr {
            self.stack.push(curr);
            self.curr = curr.left.as_ref().map(Box::borrow);
        }

        self.stack.pop().map(|curr| {
            self.curr = curr.right.as_ref().map(Box::borrow);
            &curr.value
        })
    }
}

impl<T> IntervalTree<T>
where T: PrimInt
{
    pub fn new() -> Self {
        Self(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    //pub fn insert(&mut self, new: Interval<T>) {
    //    match &mut self.0 {
    //        None => self.0 = Some(Node::leaf(new)),
    //        Some(root) => {
    //            let mut stack = vec![(root, new)];
    //            while let Some((mut ref curr, new)) = stack.pop() {
    //                let (new_left, new_right) = new.diff(curr.value);

    //                match (new_left, &mut curr.left) {
    //                    (Some(new), Some(child)) => stack.push((child.borrow_mut(), new)),
    //                    (Some(new), None) => curr.set_left(new),
    //                    _ => (),
    //                };
    //            }
    //        }
    //    };
    //}

    pub fn iter(&self) -> Iter<T> {
        let stack = Vec::new();
        let curr = self.0.as_ref();
        Iter { stack, curr }
    }
}

struct Node<T: PrimInt> {
    value: Interval<T>,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl<T> Node<T>
where T: PrimInt
{
    fn leaf(value: Interval<T>) -> Self {
        Self { value, left: None, right: None }
    }

    fn branch(value: Interval<T>, left: Option<Self>, right: Option<Self>) -> Self {
        let left = left.map(Box::new);
        let right = right.map(Box::new);
        Self { value, left, right }
    }

    fn set_left(&mut self, new: Interval<T>) {
        self.left = Some(Box::new(Self::leaf(new)));
    }

    fn set_right(&mut self, new: Interval<T>) {
        self.right = Some(Box::new(Self::leaf(new)));
    }
}

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

    #[test]
    fn interval_tree_iter() {
        let i1 = Interval(0_u32, 3_u32);
        let i2 = Interval(4_u32, 7_u32);
        let i3 = Interval(7_u32, 10_u32);

        let n1 = Node::leaf(i1);
        let n3 = Node::leaf(i3);
        let n2 = Node::branch(i2, Some(n1), Some(n3));
        let tr = IntervalTree(Some(n2));

        let is: Vec<Interval<_>> = tr.iter().copied().collect();
        assert_eq!(vec![i1, i2, i3], is);
    }
}
