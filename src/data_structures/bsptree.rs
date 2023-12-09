use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

/// Type alias for a reference counting pointer to a Node.
type BSPTreeNode = Rc<RefCell<Node>>;

type NodeData = u32;

/// Defines the four ways in which you can move focus around in the BSPTree.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

impl MoveDirection {
    pub fn apply_move(&self, rect: Rectangle) -> (i32, i32) {
        match self {
            Self::Left => (rect.x as i32 - 1, rect.y as i32),
            Self::Right => (rect.x as i32 + rect.w as i32 + 1, rect.y as i32),
            Self::Up => (rect.x as i32, rect.y as i32 - 1),
            Self::Down => (rect.x as i32, rect.y as i32 + rect.h as i32 + 1),
        }
    }
}
/// The way in which a Node in the BSP Tree will be split.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

impl SplitDirection {
    pub fn split(&self, rect: Rectangle) -> (Rectangle, Rectangle) {
        match self {
            Self::Horizontal => (
                Rectangle::new(rect.x, rect.y, rect.w, rect.h / 2),
                Rectangle::new(rect.x, rect.y + rect.h / 2, rect.w, rect.h / 2),
            ),
            Self::Vertical => (
                Rectangle::new(rect.x, rect.y, rect.w / 2, rect.h),
                Rectangle::new(rect.x + rect.w / 2, rect.y, rect.w / 2, rect.h),
            ),
        }
    }
}

/// Structure representing a simple rectangle.
/// The `x` and `y` fields represent the **top-left** corner of the rectangle.
/// The `w` and `h` fields represent the width and height of the Rectangle starting from the top
/// left point.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rectangle {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    pub fn is_inside(&self, x_: i32, y_: i32) -> bool {
        (x_ as u32 >= self.x && x_ as u32 <= self.x + self.w)
            && (y_ as u32 >= self.y && y_ as u32 <= self.y + self.h)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}); ({}, {})", self.x, self.y, self.w, self.h)
    }
}

/// A Binary Space Partitioning Tree is a type of binary tree with with all nodes having either two
/// or zero descendants. The BSP Tree can be used to divide a rectangle into chunks, each the
/// half the size of the previous chunk.
#[derive(Clone, Debug, PartialEq)]
pub struct BSPTree {
    root: Option<BSPTreeNode>,
    focused: Option<BSPTreeNode>,
    size: Rectangle,
}

impl BSPTree {
    /// Create a new BSPTree with a given `size`.
    pub fn new(size: Rectangle) -> Self {
        Self {
            root: None,
            focused: None,
            size,
        }
    }

    /// Insert a new node as into the tree at the currently focused node.
    pub fn insert(&mut self, data: NodeData) {
        if self.root.is_none() {
            let mut n = Node::new(self.size, SplitDirection::Vertical, data);
            n.focused = true;
            let node = Rc::new(RefCell::new(n));

            self.root = Some(node.clone());
            self.focused = Some(node.clone());
            return;
        }

        let tmp = self.focused.as_mut().unwrap().clone();
        let mut focused = tmp.borrow_mut();

        let (lsize, rsize) = focused.split.split(focused.rect);
        focused.leaf = false;
        focused.focused = false;

        let prev_data = focused.data.unwrap();
        focused.data = None;

        let mut left = Node::new(lsize, focused.split, prev_data);
        let mut right = Node::new(rsize, focused.split, data);

        left.parent = Some(self.focused.as_ref().unwrap().clone());
        right.parent = Some(self.focused.as_ref().unwrap().clone());
        right.focused = true;
        right.right_child = true;

        let new_focused = Rc::new(RefCell::new(right));

        focused.left = Some(Rc::new(RefCell::new(left)));
        focused.right = Some(new_focused.clone());

        drop(focused);

        self.focused = Some(new_focused);
    }

    /// Delete the currently focused node.
    /// Focus is set to the node which fills the space of the deleted node.
    pub fn delete_focused(&mut self) {
        if self.focused.is_none() {
            return;
        }

        // If we try to remove the root node, we just return.
        if self.focused.as_ref().unwrap().borrow().parent.is_none() {
            self.root = None;
            self.focused = None;
            return;
        }

        let focused = self.focused.as_ref().unwrap().clone();
        let rect = focused.borrow().rect;

        if let Some(parent) = focused.borrow().parent.clone() {
            let sibling = if focused.borrow().right_child {
                parent.borrow().left.as_ref().unwrap().clone()
            } else {
                parent.borrow().right.as_ref().unwrap().clone()
            };

            // set the correct parameters on the `sibling`
            {
                let mut s = sibling.borrow_mut();
                s.parent = parent.borrow().parent.clone();
                s.rect = parent.borrow().rect;
                s.right_child = parent.borrow().right_child;

                // set the correct child of the parent of the parent.
                if let Some(par) = s.parent.as_ref() {
                    if s.right_child {
                        par.borrow_mut().right = Some(sibling.clone());
                    } else {
                        par.borrow_mut().left = Some(sibling.clone());
                    }
                }
            }

            // make the sibling the new parent
            parent.replace(sibling.borrow().clone());

            // update the size of the subtrees
            let p = parent.borrow_mut();
            let (ls, rs) = p.split.split(p.rect);
            if let Some(l) = p.left.clone() {
                l.borrow_mut().update(ls);
            }

            if let Some(r) = p.right.clone() {
                r.borrow_mut().update(rs);
            }

            drop(p);
        };

        self.focus_coords(rect.x as i32, rect.y as i32);
    }

    /// Find a node corresponding to the given coordinates.
    pub fn get_node(&self, x: i32, y: i32) -> Option<BSPTreeNode> {
        self.root.as_ref()?;

        let mut node = self.root.clone();
        let mut new_node;
        while let Some(n) = node.as_ref() {
            new_node = None;
            if let Some(left) = n.borrow().left.as_ref() {
                if left.borrow().rect.is_inside(x, y) {
                    new_node = Some(left.clone());
                }
            }
            if let Some(right) = n.borrow().right.as_ref() {
                if right.borrow().rect.is_inside(x, y) {
                    new_node = Some(right.clone());
                }
            }

            if new_node.is_none() {
                break;
            }

            node = new_node.clone();
        }

        if let Some(n) = node.clone() {
            if !n.borrow().leaf {
                return None;
            }
        }

        node
    }

    /// Try to move focus in the given `direction`. If there is nowhere to move, the focus stays
    /// the same.
    pub fn move_focus(&mut self, direction: MoveDirection) {
        if self.root.is_none() {
            return;
        }

        let (x, y) = direction.apply_move(self.focused.as_ref().unwrap().borrow().rect);
        self.focus_coords(x, y);
    }

    /// Try to focus a node on the given coordinates, if the coordinates are invalid, nothing
    /// happens.
    pub fn focus_coords(&mut self, x: i32, y: i32) {
        if let Some(node) = self.get_node(x, y) {
            self.focused.as_mut().unwrap().borrow_mut().focused = false;
            node.borrow_mut().focused = true;
            self.focused = Some(node);
        }
    }

    /// Set the `SplitDirection` of the currently focused Node.
    pub fn set_split(&self, split: SplitDirection) {
        if let Some(f) = self.focused.as_ref() {
            f.borrow_mut().split = split
        }
    }

    /// Toggle the `SplitDirection` of the currently focused Node.
    pub fn toggle_split(&self) {
        if let Some(f) = self.focused.as_ref() {
            let split = f.borrow().split;

            match split {
                SplitDirection::Vertical => f.borrow_mut().split = SplitDirection::Horizontal,
                SplitDirection::Horizontal => f.borrow_mut().split = SplitDirection::Vertical,
            }
        }
    }

    /// Print the BSP Tree.
    ///
    /// The `print_type` can be:
    /// - `0` - print in the `pre-order` order
    /// - `1` - print in the `in-order` order
    /// - `any other` - print in the `post-order` order
    pub fn print(&self, print_type: i32) {
        if let Some(r) = self.root.as_ref() {
            match print_type {
                0 => r.borrow().print_pre(0),
                1 => r.borrow().print_in(0),
                _ => r.borrow().print_post(0),
            }
        }
    }

    pub fn walk(&self) -> Vec<BSPTreeNode> {
        let mut vec = vec![];

        if let Some(r) = self.root.as_ref() {
            vec.push(r.clone());
            r.borrow().walk(&mut vec);
        }
        vec
    }
}

/// A Node in the BSP Tree has a reference to it's parent Node, and to it's two children nodes. It
/// also has a `rect` field which has the size of the space it represents. The `metadata` field is
/// used to store any arbitrary metadata for the Node. The `split` field indicates how the area
/// should be split when adding children.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    rect: Rectangle,
    left: Option<BSPTreeNode>,
    right: Option<BSPTreeNode>,
    parent: Option<BSPTreeNode>,
    split: SplitDirection,
    leaf: bool,
    data: Option<NodeData>,
    focused: bool,
    right_child: bool,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "value:{:?} size:{} focus:{} right_child:{}",
            self.data, self.rect, self.focused, self.right_child
        )
    }
}

impl Node {
    /// Create a new node with Rectangle `rect`..
    pub fn new(rect: Rectangle, split: SplitDirection, data: NodeData) -> Self {
        Self {
            rect,
            left: None,
            right: None,
            parent: None,
            split,
            leaf: true,
            data: Some(data),
            focused: false,
            right_child: false,
        }
    }

    /// Return true if the node is currently focused.
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Get the data stored in the node.
    pub fn get_data(&self) -> Option<NodeData> {
        self.data
    }

    /// Get the size of the node.
    pub fn get_rect(&self) -> Rectangle {
        self.rect
    }

    /// Recursively walk both sides of the subtree starting from this node.
    pub fn walk(&self, v: &mut Vec<BSPTreeNode>) {
        if let Some(l) = self.left.clone() {
            v.push(l.clone());
            l.borrow().walk(v);
        }
        if let Some(r) = self.right.clone() {
            v.push(r.clone());
            r.borrow().walk(v);
        }
    }

    /// Update the size of the current node as well as it's children.
    pub fn update(&mut self, rect: Rectangle) {
        self.rect = rect;
        let (lrect, rrect) = self.split.split(rect);

        if let Some(l) = self.left.clone() {
            l.borrow_mut().update(lrect);
        }
        if let Some(r) = self.right.clone() {
            r.borrow_mut().update(rrect);
        }
    }

    fn print_pre(&self, indent: usize) {
        println!(
            "{}{self}",
            " ".chars().cycle().take(indent).collect::<String>(),
        );

        if let Some(left) = self.left.clone() {
            left.borrow().print_pre(indent + 4);
        }

        if let Some(right) = self.right.clone() {
            right.borrow().print_pre(indent + 4)
        }
    }

    fn print_in(&self, indent: usize) {
        if let Some(left) = self.left.clone() {
            left.borrow().print_in(indent + 4);
        }

        println!(
            "{}{self}",
            " ".chars().cycle().take(indent).collect::<String>(),
        );

        if let Some(right) = self.right.clone() {
            right.borrow().print_in(indent + 4)
        }
    }

    fn print_post(&self, indent: usize) {
        if let Some(left) = self.left.clone() {
            left.borrow().print_post(indent + 4);
        }

        if let Some(right) = self.right.clone() {
            right.borrow().print_post(indent + 4)
        }

        println!(
            "{}{self}",
            " ".chars().cycle().take(indent).collect::<String>(),
        );
    }
}
