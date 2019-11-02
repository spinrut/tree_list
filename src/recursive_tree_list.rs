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

/* TODO Convert to Red-Black version:
 * pop_front
 * pop_back
 * remove
 */
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

    fn rot_r(mut node: Box<TreeNode<T>>, mut left: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        node.left = left.right.take();

        // Update node.num_to_left. Values for tmp and other nodes do not change.
        node.num_to_left -= left.num_to_left + 1;

        left.right = Some(node);

        // flipColors(), except two of the colors are already correct
        left.left.as_mut().unwrap().color = Color::Black;
        left
    }

    fn rot_l(mut node: Box<TreeNode<T>>, mut right: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        node.right = right.left.take();

        // Update tmp.num_to_left. Values for 'node' and other nodes do not change.
        right.num_to_left += node.num_to_left + 1;

        // Link reverses direction, so node and tmp swap colors
        std::mem::swap(&mut right.color, &mut node.color);

        right.left = Some(node);
        right
    }

    fn node_color(node: &Link<T>) -> Color {
        match node {
            None => Color::Black,
            Some(ref x) => x.color
        }
    }

    fn push_front_aux(node: Link<T>, val: T) -> Box<TreeNode<T>> {
        match node {
            None => Box::new(TreeNode::new(val)),
            Some(mut x) => {
                x.num_to_left += 1;
                let new_left = Self::push_front_aux(x.left.take(), val);

                if new_left.color == Color::Red && Self::node_color(&new_left.left) == Color::Red {
                    Self::rot_r(x, new_left)
                } else {
                    x.left = Some(new_left);
                    x
                }
            }
        }
    }

    pub fn push_front(&mut self, val: T) {
        self.size += 1;
        let mut new_root = Self::push_front_aux(self.root.take(), val);
        new_root.color = Color::Black;
        self.root = Some(new_root);
    }

    fn pop_front_aux(mut node: Box<TreeNode<T>>) -> (Link<T>, Option<T>) {
        match node.left {
            None => (node.right, Some(node.val)),
            Some(next) => {
                node.num_to_left -= 1;
                let (left, res) = Self::pop_front_aux(next);
                node.left = left;
                (Some(node), res)
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let (root, res) = match self.root.take() {
            None => (None, None),
            Some(node) => {
                self.size -= 1;
                Self::pop_front_aux(node)
            }
        };

        self.root = root;
        res
    }

    fn push_back_aux(node: Link<T>, val: T) -> Box<TreeNode<T>> {
        match node {
            None => Box::new(TreeNode::new(val)),
            Some(mut x) => {
                let mut new_right = Self::push_back_aux(x.right.take(), val);
                match new_right.color {
                    Color::Red => {
                        if Self::node_color(&x.left) == Color::Red {
                            x.left.as_mut().unwrap().color = Color::Black;
                            new_right.color = Color::Black;
                            x.color = Color::Red;
                            x.right = Some(new_right);
                            x
                        } else {
                            Self::rot_l(x, new_right)
                        }
                    },
                    Color::Black => {
                        x.right = Some(new_right);
                        x
                    }
                }
            }
        }
    }

    pub fn push_back(&mut self, val: T) {
        self.size += 1;
        let mut new_root = Self::push_back_aux(self.root.take(), val);
        new_root.color = Color::Black;
        self.root = Some(new_root);
    }

    fn pop_back_aux(mut node: Box<TreeNode<T>>) -> (Link<T>, Option<T>) {
        match node.right {
            None => (node.left, Some(node.val)),
            Some(next) => {
                let (right, res) = Self::pop_back_aux(next);
                node.right = right;
                (Some(node), res)
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let (root, res) = match self.root.take() {
            None => (None, None),
            Some(node) => {
                self.size -= 1;
                Self::pop_back_aux(node)
            }
        };

        self.root = root;
        res
    }

    fn insert_aux(node: Link<T>, index: usize, val: T) -> Box<TreeNode<T>> {
        match node {
            None => Box::new(TreeNode::new(val)),
            Some(mut x) => {
                if index <= x.num_to_left {
                    x.num_to_left += 1;
                    let new_left = Self::insert_aux(x.left.take(), index, val);
                    if new_left.color == Color::Red && Self::node_color(&new_left.left) == Color::Red {
                        Self::rot_r(x, new_left)
                    } else {
                        x.left = Some(new_left);
                        x
                    }
                } else {
                    let mut new_right = Self::insert_aux(x.right.take(), index - x.num_to_left - 1, val);
                    match new_right.color {
                        Color::Red => {
                            if Self::node_color(&x.left) == Color::Red {
                                x.left.as_mut().unwrap().color = Color::Black;
                                new_right.color = Color::Black;
                                x.color = Color::Red;
                                x.right = Some(new_right);
                                x
                            } else {
                                Self::rot_l(x, new_right)
                            }
                        }
                        Color::Black => {
                            x.right = Some(new_right);
                            x
                        }
                    }
                }
            }
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
        match index.cmp(&node.num_to_left) {
            Ordering::Less => {
                node.num_to_left -= 1;
                let (left, res) = Self::remove_aux(node.left.unwrap(), index);
                node.left = left;
                (Some(node), res)
            }
            Ordering::Greater => {
                index -= node.num_to_left + 1;
                let (right, res) = Self::remove_aux(node.right.unwrap(), index);
                node.right = right;
                (Some(node), res)
            }
            Ordering::Equal => match (node.left.take(), node.right.take()) {
                (None, None) => (None, node.val),
                (left, None) => (left, node.val),
                (None, right) => (right, node.val),
                (left, Some(right)) => {
                    node.left = left;
                    let (right, succ) = Self::pop_front_aux(right);
                    node.right = right;
                    let res = std::mem::replace(&mut node.val, succ.unwrap());
                    (Some(node), res)
                }
            },
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.size {
            panic!("Index out of bounds!");
        } else {
            self.size -= 1;
            let (root, res) = Self::remove_aux(self.root.take().unwrap(), index);
            self.root = root;
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
    fn test_remove_node_with_children() {
        let mut tree: RecursiveTreeList<char> = RecursiveTreeList::new();

        tree.push_back('a');
        tree.push_front('b');
        tree.push_back('c');
        tree.insert(2, 'd');
        tree.push_back('e');
        tree.insert(3, 'f');
        assert_eq!(tree.len(), 6);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            ['b', 'a', 'd', 'f', 'c', 'e'].to_vec()
        );
        assert_eq!(tree.remove(1), 'a');

        assert_eq!(tree.len(), 5);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            ['b', 'd', 'f', 'c', 'e'].to_vec()
        );
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
