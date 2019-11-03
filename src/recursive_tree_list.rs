// Left-Leaning Red-Black Tree List. Insertion and deletion algorithms adapted from
// https://www.cs.princeton.edu/~rs/talks/LLRB/LLRB.pdf
use std::cmp::Ordering;
type Link<T> = Option<Box<TreeNode<T>>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Color {
    Black,
    Red
}

#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    color: Color,
    num_to_left: usize,
    left: Link<T>,
    right: Link<T>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Self {
        TreeNode {
            val,
            color: Color::Red,
            num_to_left: 0,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct RecursiveTreeList<T> {
    root: Link<T>,
    size: usize,
}

impl<T> RecursiveTreeList<T> {
    pub fn new() -> Self {
        RecursiveTreeList {
            root: None,
            size: 0,
        }
    }

    pub fn get(&self, mut index: usize) -> Option<&T> {
        if index > self.size {
            None
        } else {
            let mut node = self.root.as_ref().unwrap();

            loop {
                match index.cmp(&node.num_to_left) {
                    Ordering::Less => node = node.left.as_ref().unwrap(),
                    Ordering::Greater => {
                        index -= node.num_to_left + 1;
                        node = node.right.as_ref().unwrap();
                    }
                    Ordering::Equal => break Some(&node.val),
                }
            }
        }
    }

    pub fn get_mut(&mut self, mut index: usize) -> Option<&mut T> {
        if index > self.size {
            None
        } else {
            let mut node = self.root.as_mut().unwrap();

            loop {
                match index.cmp(&node.num_to_left) {
                    Ordering::Less => node = node.left.as_mut().unwrap(),
                    Ordering::Greater => {
                        index -= node.num_to_left + 1;
                        node = node.right.as_mut().unwrap();
                    }
                    Ordering::Equal => break Some(&mut node.val),
                }
            }
        }
    }

    fn rot_r(mut left: Box<TreeNode<T>>, mut node: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        node.num_to_left -= left.num_to_left + 1;
        node.left = left.right.take();
        left.color = node.color;
        node.color = Color::Red;
        left.right = Some(node);
        left
    }

    fn rot_l(mut right: Box<TreeNode<T>>, mut node: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        right.num_to_left += node.num_to_left + 1;
        node.right = right.left.take();
        right.color = node.color;
        node.color = Color::Red;
        right.left = Some(node);
        right
    }

    fn node_color(node: &Link<T>) -> Color {
        match node {
            Some(x) => x.color,
            None => Color::Black
        }
    }

    fn flip_colors_black(node: &mut TreeNode<T>) {
        node.left.as_mut().unwrap().color = Color::Black;
        node.right.as_mut().unwrap().color = Color::Black;
        node.color = Color::Red;
    }

    fn flip_colors_red(node: &mut TreeNode<T>) {
        node.left.as_mut().unwrap().color = Color::Red;
        node.right.as_mut().unwrap().color = Color::Red;
        node.color = Color::Black;
    }

    fn move_red_left(mut node: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        Self::flip_colors_red(node.as_mut());
        if Self::node_color(&node.right.as_ref().unwrap().left) == Color::Red {
            let mut right = node.right.take().unwrap();
            right = Self::rot_r(right.left.take().unwrap(), right);
            node = Self::rot_l(right, node);
            Self::flip_colors_black(&mut node);
        }
        node
    }

    fn move_red_right(mut node: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        Self::flip_colors_red(node.as_mut());
        if Self::node_color(&node.left.as_ref().unwrap().left) == Color::Red {
            node = Self::rot_r(node.left.take().unwrap(), node);
            Self::flip_colors_black(&mut node);
        }
        node
    }

    fn push_front_aux(node: Link<T>, val: T) -> Box<TreeNode<T>> {
        match node {
            Some(mut x) => {
                x.num_to_left += 1;
                let new_left = Self::push_front_aux(x.left.take(), val);
                if new_left.color == Color::Red && Self::node_color(&new_left.left) == Color::Red {
                    x = Self::rot_r(new_left, x);
                    Self::flip_colors_black(x.as_mut());
                } else {
                    x.left = Some(new_left);
                }
                x
            }
            None => Box::new(TreeNode::new(val))
        }
    }

    pub fn push_front(&mut self, val: T) {
        self.size += 1;
        let mut new_root = Self::push_front_aux(self.root.take(), val);
        new_root.color = Color::Black;
        self.root = Some(new_root);
    }

    fn pop_front_aux(mut node: Box<TreeNode<T>>) -> (Link<T>, T) {
        match node.left {
            Some(ref next) => {
                if next.color == Color::Black && Self::node_color(&next.left) == Color::Black {
                    node = Self::move_red_left(node);
                    node.num_to_left -= 1;
                    let (new_left, res) = Self::pop_front_aux(node.left.take().unwrap());
                    node.left = new_left;
                    if node.right.as_ref().unwrap().color == Color::Red {
                        if Self::node_color(&node.left) == Color::Red {
                            Self::flip_colors_black(node.as_mut());
                        } else {
                            node = Self::rot_l(node.right.take().unwrap(), node);
                        }
                    }
                    (Some(node), res)
                } else {
                    node.num_to_left -= 1;
                    let (new_left, res) = Self::pop_front_aux(node.left.take().unwrap());
                    node.left = new_left;
                    (Some(node), res)
                }
            }
            None => (None, node.val)
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.root.take() {
            Some(root) => {
                self.size -= 1;
                let (new_root, res) = Self::pop_front_aux(root);
                self.root = new_root.map(|mut x| {
                    x.color = Color::Black;
                    x
                });
                Some(res)
            }
            None => None
        }
    }

    fn push_back_aux(node: Link<T>, val: T) -> Box<TreeNode<T>> {
        match node {
            Some(mut x) => {
                let new_right = Self::push_back_aux(x.right.take(), val);
                if new_right.color == Color::Red {
                    if Self::node_color(&x.left) == Color::Red {
                        x.right = Some(new_right);
                        Self::flip_colors_black(x.as_mut());
                    } else {
                        x = Self::rot_l(new_right, x)
                    }
                } else {
                    x.right = Some(new_right);
                }
                x
            }
            None => Box::new(TreeNode::new(val))
        }
    }

    pub fn push_back(&mut self, val: T) {
        self.size += 1;
        let mut new_root = Self::push_back_aux(self.root.take(), val);
        new_root.color = Color::Black;
        self.root = Some(new_root);
    }

    fn pop_back_aux(mut node: Box<TreeNode<T>>) -> (Link<T>, T) {
        if Self::node_color(&node.left) == Color::Red {
            node = Self::rot_r(node.left.take().unwrap(), node);
            let (new_right, res) = Self::pop_back_aux(node.right.take().unwrap());
            if Self::node_color(&new_right) == Color::Red {
                node = Self::rot_l(new_right.unwrap(), node);
            } else {
                node.right = new_right;
            }
            (Some(node), res)
        } else {
            match node.right {
                Some(ref next) => {
                    if next.color == Color::Black && Self::node_color(&next.left) == Color::Black {
                        node = Self::move_red_right(node);
                        let (new_right, res) = Self::pop_back_aux(node.right.take().unwrap());
                        if Self::node_color(&new_right) == Color::Red {
                            if node.left.as_ref().unwrap().color == Color::Red {
                                node.right = new_right;
                                Self::flip_colors_black(&mut node);
                            } else {
                                node = Self::rot_l(new_right.unwrap(), node);
                            }
                        } else {
                            node.right = new_right;
                        }
                        (Some(node), res)
                    } else {
                        let (new_right, res) = Self::pop_back_aux(node.right.take().unwrap());
                        if Self::node_color(&new_right) == Color::Red {
                            node = Self::rot_l(new_right.unwrap(), node);
                        } else {
                            node.right = new_right;
                        }
                        (Some(node), res)
                    }
                }
                None => (None, node.val)
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.root.take() {
            Some(root) => {
                self.size -= 1;
                let (new_root, res) = Self::pop_back_aux(root);
                self.root = new_root.map(|mut x| {
                    x.color = Color::Black;
                    x
                });
                Some(res)
            }
            None => None
        }
    }

    fn insert_aux(node: Link<T>, mut index: usize, val: T) -> Box<TreeNode<T>> {
        match node {
            Some(mut x) => {
                if index <= x.num_to_left {
                    x.num_to_left += 1;
                    let new_left = Self::insert_aux(x.left.take(), index, val);
                    if new_left.color == Color::Red && Self::node_color(&new_left.left) == Color::Red {
                        x = Self::rot_r(new_left, x);
                        Self::flip_colors_black(x.as_mut());
                    } else {
                        x.left = Some(new_left);
                    }
                } else {
                    index -= x.num_to_left + 1;
                    let new_right = Self::insert_aux(x.right.take(), index, val);
                    if new_right.color == Color::Red {
                        if Self::node_color(&x.left) == Color::Red {
                            x.right = Some(new_right);
                            Self::flip_colors_black(x.as_mut());
                        } else {
                            x = Self::rot_l(new_right, x);
                        }
                    } else {
                        x.right = Some(new_right);
                    }
                }
                x
            }
            None => Box::new(TreeNode::new(val))
        }
    }

    pub fn insert(&mut self, index: usize, val: T) {
        if index > self.size {
            panic!("Index out of bounds!");
        } else {
            self.size += 1;
            let mut new_root = Self::insert_aux(self.root.take(), index, val);
            new_root.color = Color::Black;
            self.root = Some(new_root);
        }
    }

    fn remove_aux(mut node: Box<TreeNode<T>>, mut index: usize) -> (Link<T>, T) {
        let res: T;
        if index < node.num_to_left {
            let left = node.left.as_ref().unwrap();
            if left.color == Color::Black && Self::node_color(&left.left) == Color::Black {
                node = Self::move_red_left(node);
            }
            node.num_to_left -= 1;
            let (new_left, val) = Self::remove_aux(node.left.take().unwrap(), index);
            node.left = new_left;
            res = val;
        } else {
            if Self::node_color(&node.left) == Color::Red {
                node = Self::rot_r(node.left.take().unwrap(), node);
                index -= node.num_to_left + 1;
                let (new_right, val) = Self::remove_aux(node.right.take().unwrap(), index);
                node.right = new_right;
                res = val;
            } else {
                if index == node.num_to_left && node.right.is_none() {
                    return (None, node.val);
                }
                let right = node.right.as_ref().unwrap();
                if right.color == Color::Black && Self::node_color(&right.left) == Color::Black {
                    node = Self::move_red_right(node);
                }
                if index == node.num_to_left {
                    let (new_right, val) = Self::pop_front_aux(node.right.take().unwrap());
                    node.right = new_right;
                    res = std::mem::replace(&mut node.val, val);
                } else {
                    index -= node.num_to_left + 1;
                    let (new_right, val) = Self::remove_aux(node.right.take().unwrap(), index);
                    node.right = new_right;
                    res = val;
                }
            }
        }

        match Self::node_color(&node.right) {
            Color::Red => {
                if Self::node_color(&node.left) == Color::Red {
                    Self::flip_colors_black(&mut node);
                } else {
                    node = Self::rot_l(node.right.take().unwrap(), node);
                }
            }
            Color::Black => {
                if Self::node_color(&node.left) == Color::Red && Self::node_color(&node.left.as_ref().unwrap().left) == Color::Red {
                    node = Self::rot_r(node.left.take().unwrap(), node);
                }
            }
        }

        (Some(node), res)
    }

    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.size {
            panic!("Index out of bounds!");
        } else {
            self.size -= 1;
            let (new_root, res) = Self::remove_aux(self.root.take().unwrap(), index);
            self.root = new_root.map(|mut x| {
                x.color = Color::Black;
                x
            });
            res
        }
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.root = None;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> Iter<T> {
        let mut curr = &self.root;
        let mut stack: Vec<&TreeNode<T>> = Vec::new();
        while let Some(ref node) = curr {
            stack.push(node);
            curr = &node.left;
        }

        Iter { stack }
    }
}

impl<T> Drop for RecursiveTreeList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

pub struct Iter<'a, T> {
    stack: Vec<&'a TreeNode<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.stack.pop();

        next.map(|node| {
            let mut curr = &node.right;
            while let Some(ref x) = curr {
                self.stack.push(x);
                curr = &x.left;
            }
            &node.val
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RecursiveTreeList;

    #[test]
    fn test_empty_tree() {
        let tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        assert_eq!(tree.len(), 0);
        assert_eq!(tree.iter().copied().collect::<Vec<char>>(), [].to_vec());
    }

    #[test]
    fn test_add_one() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.push_back('a');

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.iter().copied().collect::<Vec<char>>(), ['a'].to_vec());
    }

    #[test]
    fn test_add_three() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.push_back('a');
        tree.push_back('b');
        tree.push_back('c');

        assert_eq!(tree.len(), 3);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            ['a', 'b', 'c'].to_vec()
        );
    }

    #[test]
    fn test_add_front_one() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.push_front('a');

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.iter().copied().collect::<Vec<char>>(), ['a'].to_vec());
    }

    #[test]
    fn test_add_front_three() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.push_front('a');
        tree.push_front('b');
        tree.push_front('c');

        assert_eq!(tree.len(), 3);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            ['c', 'b', 'a'].to_vec()
        );
    }

    #[test]
    fn test_insert_one() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.insert(0, 'a');

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.iter().copied().collect::<Vec<char>>(), ['a'].to_vec());
    }

    #[test]
    fn test_insert_ordered() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.insert(0, 'a');
        tree.insert(1, 'b');
        tree.insert(2, 'c');
        tree.insert(3, 'd');

        assert_eq!(tree.len(), 4);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            ['a', 'b', 'c', 'd'].to_vec()
        );
    }

    #[test]
    fn test_insert_indexed() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.insert(0, 'a');
        tree.insert(0, 'b');
        tree.insert(2, 'c');
        tree.insert(1, 'd');

        assert_eq!(tree.len(), 4);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            ['b', 'd', 'a', 'c'].to_vec()
        );
    }

    #[test]
    fn test_push_pop() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.push_back('a');
        tree.push_back('b');
        tree.push_back('c');
        tree.push_back('d');

        assert_eq!(tree.pop_back(), Some('d'));
        assert_eq!(tree.pop_back(), Some('c'));
        tree.push_back('e');
        tree.push_back('f');

        assert_eq!(tree.len(), 4);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            ['a', 'b', 'e', 'f'].to_vec()
        );
    }

    #[test]
    fn test_insert_pop() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.insert(0, 'a');
        tree.insert(0, 'b');
        assert_eq!(tree.pop_back(), Some('a'));

        tree.insert(1, 'c');
        tree.insert(1, 'd');

        assert_eq!(tree.len(), 3);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            ['b', 'd', 'c'].to_vec()
        );
    }

    #[test]
    fn test_push_remove() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.push_back('a');
        tree.push_back('b');
        tree.push_back('c');
        tree.push_back('d');
        assert_eq!(tree.remove(3), 'd');

        tree.push_back('e');
        tree.push_back('f');
        assert_eq!(tree.remove(2), 'c');

        assert_eq!(tree.len(), 4);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            ['a', 'b', 'e', 'f'].to_vec()
        );
    }

    #[test]
    fn test_insert_remove() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.insert(0, 'a');
        tree.insert(0, 'b');
        tree.insert(2, 'c');
        tree.insert(1, 'd');
        println!("{:?}", tree);

        assert_eq!(tree.remove(1), 'd');
        println!("{:?}", tree);
        assert_eq!(tree.remove(1), 'a');
        println!("{:?}", tree);

        assert_eq!(tree.len(), 2);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            ['b', 'c'].to_vec()
        );
    }

    #[test]
    fn test_get() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.insert(0, 'a');
        tree.insert(0, 'b');
        tree.insert(2, 'c');
        tree.insert(1, 'd');

        assert_eq!(tree.get(0), Some('b').as_ref());
        assert_eq!(tree.get(1), Some('d').as_ref());
        assert_eq!(tree.get(2), Some('a').as_ref());
        assert_eq!(tree.get(3), Some('c').as_ref());
    }

    #[test]
    fn test_clear() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.push_back('a');
        tree.push_front('b');
        tree.push_back('c');
        tree.insert(2, 'd');
        tree.push_back('e');
        tree.insert(3, 'f');

        tree.clear();

        assert_eq!(tree.len(), 0);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            Vec::new()
        );
    }
}
