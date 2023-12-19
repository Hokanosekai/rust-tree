/// Represents node data.
#[derive(Clone, Debug)]
pub struct NodeData {
  /// The default path of the node.
  pub default_path: String,
  /// The length of the path.
  pub length: u64,
}

#[derive(Clone, Debug)]
pub enum NodeType {
    File(),
    Directory(),
}

#[derive(Clone, Debug)]
pub struct Node {
  pub node_type: NodeType,
  pub data: NodeData,
  pub depth: u64,
  pub path: String,
  pub name: String,
  pub children: Vec<Node>,
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Node {
  /// Creates a new root node directory.
  /// 
  /// # Returns
  /// 
  /// A new root node directory.
  /// 
  /// # Examples
  /// 
  /// ```
  /// use node::*;
  /// 
  /// let root = Node::new_root();
  /// assert_eq!(root.depth, 0);
  /// assert_eq!(root.path, "/");
  /// assert_eq!(root.name, "root");
  /// ```
  pub fn new_root() -> Node {
    Node {
      node_type: NodeType::Directory(),
      data: NodeData {
        default_path: "/".to_string(),
        length: 1,
      },
      depth: 0,
      path: "/".to_string(),
      name: "root".to_string(),
      children: Vec::new(),
    }
  }

  pub fn new_file(data: NodeData, depth: u64, path: String, name: String) -> Node {
    Node {
      node_type: NodeType::File(),
      data,
      depth,
      path,
      name,
      children: Vec::new(),
    }
  }

  pub fn new_directory(data: NodeData, depth: u64, path: String, name: String) -> Node {
    Node {
      node_type: NodeType::Directory(),
      data,
      depth,
      path,
      name,
      children: Vec::new(),
    }
  }

  /// Gets the depth of the node.
  /// 
  /// # Returns
  /// 
  /// The depth of the node.
  /// 
  /// # Examples
  /// 
  /// ```
  /// use node::*;
  /// 
  /// let root = Node::new_root();
  /// assert_eq!(root.depth(), 0);
  /// ```
  pub fn depth(&self) -> u64 {
    self.depth
  }

  /// Displays the node and its children.
  /// 
  /// # Example
  /// 
  /// ```
  /// use node::*;
  /// 
  /// let root = Node::new_root();
  /// root.display();
  /// ```
  /// 
  /// # Output
  /// 
  /// ```
  /// Directory: /
  ///  Children: 0
  /// ```
  /// 
  /// # Example
  /// 
  /// ```
  /// use node::*;
  /// 
  /// let root = Node::new_root();
  /// let mut src = Node::Directory(NodeDirectory::new(
  ///  NodeData {
  ///   default_path: "src/".to_string(),
  ///   length: 4
  ///  },
  ///  1,
  ///  "src/".to_string(),
  ///  "src".to_string()
  /// ));
  /// 
  /// src.add_child(Node::File(NodeFile::new(
  ///  NodeData {
  ///   default_path: "src/main.rs".to_string(),
  ///   length: 11
  ///  },
  ///  2,
  ///  "main.rs".to_string(),
  ///  "main.rs".to_string()
  /// )));
  /// 
  /// root.insert(src);
  /// root.display();
  /// ```
  /// 
  /// # Output
  /// 
  /// ```
  /// Directory: /
  ///  Children: 1
  ///  Directory: src/
  ///   Children: 1
  ///   File: main.rs
  /// ```
  pub fn display(&self) {
    let ds = "  ".repeat(self.depth as usize);
    match self.node_type {
      NodeType::File() => {
        println!("{}File: {}", ds, self.name);
        println!("{} Path: {}", ds, self.path);
        println!("{} Depth: {}", ds, self.depth);
      },
      NodeType::Directory() => {
        println!("{}Directory: {}", ds, self.name);
        println!("{} Children: {}", ds, self.children.len());
        println!("{} Path: {}", ds, self.path);
        println!("{} Depth: {}", ds, self.depth);

        for child in &self.children {
          child.display();
        }
      },
    }
  }

  /// Gets the name of the node.
  /// 
  /// # Returns
  /// 
  /// The name of the node.
  /// 
  /// # Examples
  /// 
  /// ```
  /// use node::*;
  /// 
  /// let root = Node::new_root();
  /// assert_eq!(root.name(), "root");
  /// ```
  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn path(&self) -> String {
    self.path.clone()
  }
}

pub struct Tree {
  pub root: Option<Node>,
}

impl Tree {
  pub fn new() -> Tree {
    Tree {
      root: None,
    }
  }

  pub fn display(&self) {
    match &self.root {
      Some(root) => {
        root.display();
      },
      None => {
        println!("No root node.");
      },
    }
  }

  pub fn insert(&mut self, node: Node) {
    match &mut self.root {
      Some(root) => {
        Tree::insert_recursive(root, node);
      },
      None => {
        self.root = Some(Node::new_root().into());
        self.insert(node);
      },
    }
  }

  pub fn find_by_name(&self, name: String) -> Option<Node> {
    match &self.root {
      Some(root) => {
        Tree::find_by_name_recursive(root, name)
      },
      None => {
        None
      },
    }
  }

  pub fn find_by_path(&self, path: String) -> Option<Node> {
    None
  }

  pub fn find_by_depth(&self, depth: u64) -> Option<Node> {
    None
  }

  fn find_by_name_recursive(node: &Node, name: String) -> Option<Node> {
    println!("Searching for node: {}", name);
    if node.name() == name {
      println!("Found node: {}", node.name());
      return Some(node.clone());
    }

    for child in &node.children {
      let found = Tree::find_by_name_recursive(child, name.clone());
      match found {
        Some(node) => {
          return Some(node);
        },
        None => {
          continue;
        },
      }
    }

    None
  }

  fn insert_recursive(node: &mut Node, child: Node) {
    if child.depth() == node.depth() + 1 {
      node.children.push(child);
      println!("Added child.");
    } else {
      let child_path = child.path();
      let path = child_path.split("/").collect::<Vec<&str>>();
      let to_find = path[node.depth() as usize];
      println!("To find: {}", to_find);

      for c in &mut node.children {
        if c.name() == to_find {
          println!("Found child: {}", c.name());
          Tree::insert_recursive(c, child);
          node.display();
          return;
        }
      }
    }
  }
}