# tree_list
## A tree-based list with logarithmic time insertion
### Explanation
Insertion of a single element into index `i` of an array list is an `O(n)` operation, because every element from the `i`th onward must be shifted backward to make room for the new element. Deletion is likewise `O(n)`. Thus, random insertion of `n` elements into an array list is an `O(n^2)` operation, which is undesirable.

A TreeList is a tree-based list. Unlike a BST, where each node stores a key, a value, and pointers to its children, a TreeList node stores the value, pointers to children, and the size of the node's left subtree. The TreeList can thereby store a sequence of ordered elements that can be inserted into and deleted from without needing to update the index of every element. Like a BST, insertion and deletion into a balanced BST can both be accomplished in `O(log n)` time. This makes the TreeList a more effective choice than an array list when performing a large number of insertions and deletions. In exchange, the TreeList loses the cache locality and constant-time random access properties of the array list.

As with ropes, each node in a TreeList records the size of its left subtree. A TreeList differs from a rope in that a TreeList stores values in all nodes, whereas a rope stores values exclusively in the leaves. A TreeList is also intended to store values of any type, rather than just strings.

### Operations
Operating on a TreeList is mostly similar to operating on a BST.
- Get: Given index `i`, perform binary search as you would with a BST. Whenever you descend to a node's right subtree, subtract that node's `size_of_left_subtree + 1`.
- Insertion: Similar to BST insertion. When descending to a node's left subtree, increment its `size_of_left_subtree`.
- Deletion: Similar to BST deletion. When descending to a node's left subtree, decrement its `size_of_left_subtree`.
- Rotation: Similar to BST rotation. Updates to `size_of_left_subtree` for each node involved can be computed from the relevant node's current `size_of_left_subtree` parameters, as well as the size of the former root's subtree.
