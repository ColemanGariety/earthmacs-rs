pub struct Buffer {
    lines: Vec<String>,
    x: i32,
    y: i32,
    mode: String,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            lines: vec![],
            x: 0,
            y: 0,
            mode: "normal".to_string(),
        }
    }

    pub fn insertLine(&mut self, line: String, index: usize) {
       self.lines.insert(index, line);
    }

    pub fn appendLine(&mut self, line: String) {
       self.lines.push(line);
    }

    pub fn removeLine(&mut self, index: usize) {
       self.lines.remove(index);
    }

    pub fn moveDown(&mut self) {
        self.y = self.y + 1;
    }

    // private
    
    fn remTabs(line: String) -> String {
        line.replace("\t", "    ")
    }
}
