use std::collections::HashMap;

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    subdirectories: HashMap<String, Directory>,
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            files: Vec::new(),
            subdirectories: HashMap::new(),
        }
    }

    fn add_file(&mut self, name: String, size: u64) {
        self.files.push(File { name, size });
    }

    fn add_directory(&mut self, name: String) {
        self.subdirectories.insert(name.clone(), Directory::new(name));
    }
}


#[derive(Debug)]
struct DomNode {
    tag: String,
    children: Vec<DomNode>,
}

impl DomNode {
    fn new(tag: String) -> Self {
        DomNode {
            tag,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: DomNode) {
        self.children.push(child);
    }
}

fn main() {
    let mut root = Directory::new("root".to_string());

    root.add_file("file1.txt".to_string(), 100);
    root.add_file("file2.txt".to_string(), 200);

    root.add_directory("documents".to_string());
    if let Some(documents) = root.subdirectories.get_mut("documents") {
        documents.add_file("doc1.docx".to_string(), 300);
    }

    println!("{:#?}", root);

    // DOM
    let mut html = DomNode::new("html".to_string());

    let mut body = DomNode::new("body".to_string());
    body.add_child(DomNode::new("h1".to_string()));
    body.add_child(DomNode::new("p".to_string()));

    html.add_child(body);

    println!("{:#?}", html);
}