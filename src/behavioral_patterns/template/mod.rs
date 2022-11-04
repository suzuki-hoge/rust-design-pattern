use itertools::Itertools;

trait TextTemplate {
    // 処理フローを定義する
    fn run(&self) -> String {
        vec![self.title(), self.list_start(), self.list_items(), self.list_end()].join("")
    }

    // 実装は定義しない
    fn title(&self) -> String;
    fn list_start(&self) -> String;
    fn list_items(&self) -> String;
    fn list_end(&self) -> String;
}

struct SimpleText {
    title: String,
    items: Vec<String>,
}

impl TextTemplate for SimpleText {
    // 処理フローは定義しないが、実装を定義する
    // fixme: このスコープから run() を見えなくするには？
    fn title(&self) -> String {
        format!("{}: ", self.title)
    }
    fn list_start(&self) -> String {
        format!("")
    }
    fn list_items(&self) -> String {
        self.items.iter().map(|item| format!("{}", item)).collect_vec().join(", ")
    }
    fn list_end(&self) -> String {
        format!("")
    }
}

struct Html {
    title: String,
    items: Vec<String>,
}

impl TextTemplate for Html {
    // 処理フローは定義しないが、実装を定義する
    // fixme: このスコープから run() を見えなくするには？
    fn title(&self) -> String {
        format!("<h1>{}</h1>", self.title)
    }
    fn list_start(&self) -> String {
        format!("<ul>")
    }
    fn list_items(&self) -> String {
        self.items.iter().map(|item| format!("<li>{}</li>", item)).collect_vec().join("")
    }
    fn list_end(&self) -> String {
        format!("</ul>")
    }
}

#[cfg(test)]
mod tests {
    use crate::behavioral_patterns::template::{Html, SimpleText, TextTemplate};

    #[test]
    fn case1() {
        let sut = SimpleText { title: "My items".to_string(), items: vec!["foo".to_string(), "bar".to_string()] };
        assert_eq!(sut.run(), "My items: foo, bar");
        // fixme: このスコープから title() を見えなくするには？
    }

    #[test]
    fn case2() {
        let sut = Html { title: "My items".to_string(), items: vec!["foo".to_string(), "bar".to_string()] };
        assert_eq!(sut.run(), "<h1>My items</h1><ul><li>foo</li><li>bar</li></ul>");
        // fixme: このスコープから title() を見えなくするには？
    }
}
