use core::fmt;

pub struct Mail{
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl Mail{
    pub fn new(from: &str, to: &str, subject: &str, body: &str)->Self{
        Self{
            from: from.to_string(),
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
        }
    }

    pub fn parse_from(content: &str) -> Option<Mail> {
        let mut mail = Mail::new("", "", "", "");
        for item in content.split("\n"){
            if item.starts_with("from: "){
                mail.from = item[6..].to_string()
            }
            if item.starts_with("subject: "){
                mail.subject = item[9..].to_string()
            }
            if item.starts_with("body: "){
                mail.body = item[6..].to_string()
            }
        }
        if mail.from.is_empty() || mail.body.is_empty(){
            return None;
        }
        Some(mail)
    }

    pub fn parse_to(content: &str) -> Option<Mail> {
        let mut mail = Mail::new("", "", "", "");
        for item in content.split("\n"){
            if item.starts_with("from: "){
                mail.from = item[6..].to_string()
            }
            if item.starts_with("to: "){
                mail.to = item[4..].to_string()
            }
            if item.starts_with("subject: "){
                mail.subject = item[9..].to_string()
            }
            if item.starts_with("body: "){
                mail.body = item[6..].to_string()
            }
        }
        if mail.to.is_empty() || mail.body.is_empty(){
            return None;
        }
        Some(mail)
    }
}

impl fmt::Display for Mail{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "from: {}\nto: {}\nsubject: {}\nbody: {}", self.from,
        self.to, self.subject, self.body)
    }
}
