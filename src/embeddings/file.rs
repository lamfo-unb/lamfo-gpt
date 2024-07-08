#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub contents: String,
    pub sentences: Vec<String>
}

enum FileState {
    None,
    CodeBlock,
    Sentence,
    Comments,
}

impl File {
    pub fn new(path: String, contents: String) -> Self {
        Self {
            path,
            contents,
            sentences: Vec::new()
        }
    }

    pub fn parse(&mut self) {
        let mut contents = Vec::new();
        let mut state = FileState::None;
        let mut sentence = String::new();

        for line in self.contents.lines() {
            match state {
                FileState::None => {
                    if line.starts_with("```") {
                        state = FileState::CodeBlock;
                        sentence = String::new();
                        sentence.push_str(line);
                        sentence.push('\n');
                    } else if line.starts_with("---") {
                        state = FileState::Comments;
                    } else if !line.starts_with('#') && !line.is_empty() {
                        state = FileState::Sentence;
                        sentence = String::new();
                        sentence.push_str(line);
                        sentence.push('\n');
                    }
                }
                FileState::CodeBlock => {
                    sentence.push_str(line);
                    if line.starts_with("```") {
                        contents.push(sentence);
                        sentence = String::new();
                        state = FileState::None;
                    }
                }
                FileState::Comments => {
                    if line.starts_with("---") {
                        state = FileState::None;
                    }
                }
                FileState::Sentence => {
                    if line.is_empty() {
                        state = FileState::None;
                        contents.push(sentence);
                        sentence = String::new();
                    } else {
                        sentence.push_str(line);
                        sentence.push('\n');
                    }
                }
            }
        }

        self.sentences = contents;
    }
}