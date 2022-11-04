trait Noticer {
    fn notice(&self, message: &str) -> String;
}

struct Mail {
    address: String,
    subject: String,
}

impl Noticer for Mail {
    // 具体的な通信手段
    fn notice(&self, message: &str) -> String {
        format!("Sent Mail to {} ( {} ) [ message = {} ]", self.address, self.subject, message)
    }
}

struct SMS {
    number: String,
}

impl Noticer for SMS {
    // 具体的な通信手段
    fn notice(&self, message: &str) -> String {
        format!("Sent SMS to {} [ message = {} ]", self.number, message)
    }
}

struct Report<T: Noticer> {
    noticer: T,
}

impl<T: Noticer> Report<T> {
    // 通信手段は交換可能である
    pub fn new(noticer: T) -> Self {
        Self { noticer }
    }

    // 具体的な通信手段は認識しない
    pub fn notice(&self, message: &str) -> String {
        self.noticer.notice(message)
    }
}

#[cfg(test)]
mod tests {
    use crate::behavioral_patterns::strategy::{Mail, Report, SMS};

    #[test]
    fn case1() {
        let mail = Mail { address: "foo@example.com".to_string(), subject: "Payment Report".to_string() };
        let sut = Report::new(mail);
        assert_eq!(sut.notice("success."), "Sent Mail to foo@example.com ( Payment Report ) [ message = success. ]");
    }

    #[test]
    fn case2() {
        let sms = SMS { number: "090-1234-5678".to_string() };
        let sut = Report::new(sms);
        assert_eq!(sut.notice("success."), "Sent SMS to 090-1234-5678 [ message = success. ]");
    }
}
