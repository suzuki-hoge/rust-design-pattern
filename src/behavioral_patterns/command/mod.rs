// コマンドの利用者
struct Calculator {
    // Command や T ではなく dyn Command にすることで、1 つ 1 つを異なる Command の実装にできる
    // ただし dyn Command は実行時までサイズが定まらないため、Box で固定長にする
    commands: Vec<Box<dyn Command>>,
}

impl Calculator {
    fn exec_all(&self, operand: i64) -> i64 {
        self.commands.iter().fold(operand, |acc, command| command.exec(acc))
    }

    fn undo_all(&self, operand: i64) -> i64 {
        self.commands.iter().rev().fold(operand, |acc, command| command.undo(acc))
    }
}

// コマンド
trait Command {
    fn exec(&self, operand: i64) -> i64;
    fn undo(&self, operand: i64) -> i64;
}

struct AddCommand {
    operand: i64,
}

impl Command for AddCommand {
    fn exec(&self, operand: i64) -> i64 {
        self.operand + operand
    }

    fn undo(&self, operand: i64) -> i64 {
        operand - self.operand
    }
}

struct NegateCommand;

impl Command for NegateCommand {
    fn exec(&self, operand: i64) -> i64 {
        operand * -1
    }

    fn undo(&self, operand: i64) -> i64 {
        self.exec(operand)
    }
}

#[cfg(test)]
mod tests {
    use crate::behavioral_patterns::command::{AddCommand, Calculator, Command, NegateCommand};

    #[test]
    fn case1() {
        let command1 = AddCommand { operand: 3 };
        let command2 = NegateCommand;
        let command3 = AddCommand { operand: 8 };
        let commands: Vec<Box<dyn Command>> = vec![Box::new(command1), Box::new(command2), Box::new(command3)];
        let sut = Calculator { commands };

        assert_eq!(sut.exec_all(0), 5);
        assert_eq!(sut.undo_all(5), 0);

        // コマンドがオブジェクトであることで、キューイングや Undo が可能になる
    }
}
