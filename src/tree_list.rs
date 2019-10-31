use std::cmp::Ordering;
type Link<T> = Option<Box<TreeNode<T>>>;

#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    num_to_left: usize,
    left: Link<T>,
    right: Link<T>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Self {
        TreeNode {
            val,
            num_to_left: 0,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct TreeList<T> {
    root: Link<T>,
    size: usize,
}

impl<T> TreeList<T> {
    pub fn new() -> Self {
        TreeList {
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

    pub fn push_front(&mut self, val: T) {
        self.size += 1;

        let mut curr = &mut self.root;
        while let Some(node) = curr {
            node.num_to_left += 1;
            curr = &mut node.left;
        }

        *curr = Some(Box::new(TreeNode::new(val)));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match &mut self.root {
            None => None,
            curr => {
                self.size -= 1;
                let mut curr = curr;

                while curr.as_mut().unwrap().left.is_some() {
                    curr.as_mut().unwrap().num_to_left -= 1;
                    curr = &mut curr.as_mut().unwrap().left;
                }

                let mut tmp = curr.take().unwrap();
                *curr = tmp.right.take();
                Some(tmp.val)
            }
        }
    }

    pub fn push_back(&mut self, val: T) {
        self.size += 1;

        let mut curr = &mut self.root;
        while let Some(node) = curr {
            curr = &mut node.right;
        }

        *curr = Some(Box::new(TreeNode::new(val)));
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.root {
            None => None,
            curr => {
                self.size -= 1;
                let mut curr = curr;

                while curr.as_mut().unwrap().right.is_some() {
                    curr = &mut curr.as_mut().unwrap().right;
                }

                let mut tmp = curr.take().unwrap();
                *curr = tmp.left.take();
                Some(tmp.val)
            }
        }
    }

    pub fn insert(&mut self, index: usize, val: T) {
        if index > self.size {
            panic!("Index out of bounds!");
        } else {
            self.size += 1;

            let mut index = index;
            let mut curr = &mut self.root;
            while let Some(node) = curr {
                if index <= node.num_to_left {
                    node.num_to_left += 1;
                    curr = &mut node.left;
                } else {
                    index -= node.num_to_left + 1;
                    curr = &mut node.right;
                }
            }

            *curr = Some(Box::new(TreeNode::new(val)));
        }
    }

    pub fn remove(&mut self, mut index: usize) -> T {
        if index >= self.size {
            panic!("Index out of bounds!");
        } else {
            self.size -= 1;

            let mut curr = &mut self.root;
            loop {
                let num_to_left = curr.as_mut().unwrap().num_to_left;
                match index.cmp(&num_to_left) {
                    Ordering::Less => {
                        curr.as_mut().unwrap().num_to_left -= 1;
                        curr = &mut curr.as_mut().unwrap().left;
                    }
                    Ordering::Greater => {
                        index -= num_to_left + 1;
                        curr = &mut curr.as_mut().unwrap().right;
                    }
                    Ordering::Equal => break,
                }
            }

            if curr.as_mut().unwrap().left.is_none() {
                let mut tmp = curr.take().unwrap();
                *curr = tmp.right.take();
                tmp.val
            } else if curr.as_mut().unwrap().right.is_none() {
                let mut tmp = curr.take().unwrap();
                *curr = tmp.left.take();
                tmp.val
            } else {
                let curr = curr.as_mut().unwrap();
                let mut to_delete = &mut curr.right;
                while to_delete.as_mut().unwrap().left.is_some() {
                    to_delete.as_mut().unwrap().num_to_left -= 1;
                    to_delete = &mut to_delete.as_mut().unwrap().left;
                }

                let mut tmp = to_delete.take();
                *to_delete = tmp.as_mut().unwrap().right.take();
                std::mem::replace(&mut curr.val, tmp.unwrap().val)
            }
        }
    }

    pub fn clear(&mut self) {
        self.size = 0;
        let mut curr = self.root.take();
        while let Some(mut node) = curr {
            match node.left.take() {
                None => {
                    curr = node.right.take();
                }
                Some(mut left) => {
                    if node.right.is_some() {
                        node.left = left.right.take();
                        left.right = Some(node);
                        curr = Some(left);
                    } else {
                        curr = Some(left);
                    }
                }
            }
        }
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

impl<T> Drop for TreeList<T> {
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
    use super::TreeList;

    #[test]
    fn test_empty_tree() {
        let tree: TreeList<char> = TreeList::new();

        assert_eq!(tree.len(), 0);
        assert_eq!(tree.iter().copied().collect::<Vec<char>>(), [].to_vec());
    }

    #[test]
    fn test_add_one() {
        let mut tree: TreeList<char> = TreeList::new();

        tree.push_back('a');

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.iter().copied().collect::<Vec<char>>(), ['a'].to_vec());
    }

    #[test]
    fn test_add_three() {
        let mut tree: TreeList<char> = TreeList::new();

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
        let mut tree: TreeList<char> = TreeList::new();

        tree.push_front('a');

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.iter().copied().collect::<Vec<char>>(), ['a'].to_vec());
    }

    #[test]
    fn test_add_front_three() {
        let mut tree: TreeList<char> = TreeList::new();

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
        let mut tree: TreeList<char> = TreeList::new();

        tree.insert(0, 'a');

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.iter().copied().collect::<Vec<char>>(), ['a'].to_vec());
    }

    #[test]
    fn test_insert_ordered() {
        let mut tree: TreeList<char> = TreeList::new();

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
        let mut tree: TreeList<char> = TreeList::new();

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
        let mut tree: TreeList<char> = TreeList::new();

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
        let mut tree: TreeList<char> = TreeList::new();

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
        let mut tree: TreeList<char> = TreeList::new();

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
        let mut tree: TreeList<char> = TreeList::new();

        tree.insert(0, 'a');
        tree.insert(0, 'b');
        tree.insert(2, 'c');
        tree.insert(1, 'd');

        assert_eq!(tree.remove(1), 'd');
        assert_eq!(tree.remove(1), 'a');

        assert_eq!(tree.len(), 2);
        assert_eq!(
            tree.iter().copied().collect::<Vec<char>>(),
            ['b', 'c'].to_vec()
        );
    }

    #[test]
    fn test_get() {
        let mut tree: TreeList<char> = TreeList::new();

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
        let mut tree: TreeList<char> = TreeList::new();

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
}
