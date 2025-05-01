use super::tree::Tree;

pub struct Interpreter {
    tree: Tree,
}

impl Interpreter {
    pub fn new(tree: Tree) -> Self {
        Self { tree }
    }

    pub fn evaluate(&mut self) {
        todo!("{:?}", self.tree)
    }
}
