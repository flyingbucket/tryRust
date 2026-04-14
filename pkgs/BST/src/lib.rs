#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree<T> {
    Empty,
    Node {
        left: Box<Tree<T>>,
        value: T,
        right: Box<Tree<T>>,
    },
}

impl<T: Ord> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Tree::Empty
    }

    pub fn insert(self, new_val: T) -> Self {
        match self {
            Tree::Empty => Tree::Node {
                left: Box::new(Tree::Empty),
                value: new_val,
                right: Box::new(Tree::Empty),
            },
            Tree::Node { left, value, right } => {
                if new_val < value {
                    Tree::Node {
                        left: Box::new((*left).insert(new_val)),
                        value,
                        right,
                    }
                } else if new_val > value {
                    Tree::Node {
                        left,
                        value,
                        right: Box::new((*right).insert(new_val)),
                    }
                } else {
                    Tree::Node { left, value, right }
                }
            }
        }
    }

    pub fn insert_mut(&mut self, new_val: T) -> bool {
        match self {
            Tree::Empty => {
                *self = Tree::Node {
                    left: Box::new(Tree::Empty),
                    value: new_val,
                    right: Box::new(Tree::Empty),
                };
                true // 插入成功
            }
            Tree::Node { value, left, right } => {
                if new_val < *value {
                    left.insert_mut(new_val)
                } else if new_val > *value {
                    right.insert_mut(new_val)
                } else {
                    false // 值已存在，什么都不做
                }
            }
        }
    }
    // 查找
    pub fn contains(&self, target: &T) -> bool {
        match self {
            Tree::Empty => false,
            Tree::Node { left, value, right } => {
                if target < value {
                    left.contains(target)
                } else if target > value {
                    right.contains(target)
                } else {
                    true
                }
            }
        }
    }
}
