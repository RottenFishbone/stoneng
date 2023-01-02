use crate::model::Rect;

#[allow(dead_code)]
pub struct Quadtree<T> {
    bounds: Rect,

    contents: Vec<T>,
    children: Vec<Quadtree<T>>,

    depth: usize,

    max_depth: usize,
    max_children: usize,
}
impl<T> Quadtree<T> {
    /// Builds a new quadtree root
    pub fn new(bounds: Rect, max_depth: usize, max_children: usize) -> Self {
        Self::new_at(bounds, max_depth, max_children, 0)
    }

    /// Builds a new quadtree node at the specified depth level.
    ///
    /// Generally this is only used internally when a quadtree is expanding itself.
    pub fn new_at(bounds: Rect, max_depth: usize, max_children: usize, depth: usize) -> Self {
        let contents = Vec::<T>::new();
        let children = Vec::<Quadtree<T>>::new();

        Self { bounds, contents, children, max_depth, max_children, depth }
    }

    /// Pushes an item into the Quadtree, using the bounds to discern placement.
    ///
    /// In the event that `max_children` is exceeded, the quadtree will expand its 
    /// at that node, moving children into lower nodes as needed.
    pub fn insert(&mut self, item: T, bounds: Rect) {
        todo!()
    }

    /// Finds all elements within a quadtree that `bounds` could collide with.
    ///
    /// This walks the tree using `bounds` to determine the path, collecting 
    /// the contents at each node into a vector. In the event `bounds` spans multiple
    /// children, all childrens' contents will be added.
    pub fn retrieve(&mut self, bounds: Rect) -> Vec<T> {
        todo!()
    }
}
