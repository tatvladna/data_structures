fn stack() {

    // Реализуем тектовый редактор
    let mut text_editor = TextEditor::new();

    text_editor.add_text("Hello");
    text_editor.add_text(" World");
    text_editor.add_text("!");

    println!("Текущий текст: {}", text_editor.get_text());

    text_editor.undo();
    println!("После отмены: {}", text_editor.get_text());

    text_editor.undo();
    println!("После отмены: {}", text_editor.get_text());


    // скобки
    let expr = "{()}[{}]";
    println!("Скобки сбалансированы: {}", check_balanced_brackets(expr));

    let expr = "{(})";
    println!("Скобки сбалансированы: {}", check_balanced_brackets(expr));


}


#[derive(Debug)]
struct TextEditor {
    text: String,
    history: Vec<String> // реализуем стэк через вектор
}

impl TextEditor {
    fn new() -> Self {
        TextEditor {
            text: String::new(),
            history: Vec::new()
        }
    }

    fn add_text(&mut self, new_text: &str) {
        // cjхраняем в историю
        self.history.push(self.text.clone());
        // добавляем новый текст
        self.text.push_str(new_text)
    }

    fn undo(&mut self) {
        if let Some(previous_text) = self.history.pop() {
            self.text = previous_text
        } else {
            println!("Стэк пуст")
        }
    }

    fn get_text(&self) -> &str {
        &self.text
    }
}


fn check_balanced_brackets(expr: &str) -> bool {
    let mut stack = vec![];

    for ch in expr.chars() {
        match ch {
            '{' | '[' | '(' => stack.push(ch),
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            _ => continue,
        }
    }

    stack.is_empty()
}

fn main() {
    stack();
}
