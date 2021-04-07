use std::fmt;

#[derive(Debug)]
pub enum Statusx {
    Native(tonic::Code),
    UserNotFound,
    TelOrPasswordIncorrect,
    UnameExist,
}

impl Statusx {
    pub fn assemble_details(&self) {
        println!("This is message impl");
    }

    pub fn message(&self) -> &'static str {
        match self {
            Statusx::UserNotFound => "user not found",
            Statusx::TelOrPasswordIncorrect => "tel or password incorrect",
            Statusx::UnameExist => "user name exist",
            Statusx::Native(code) => code.description(),
        }
    }
}

impl From<Statusx> for tonic::Status {
    fn from(s: Statusx) -> Self {
        match s {
            Statusx::UserNotFound => tonic::Status::not_found(s.message()),
            Statusx::TelOrPasswordIncorrect => tonic::Status::not_found(s.message()),
            Statusx::UnameExist => tonic::Status::not_found(s.message()),
            Statusx::Native(code) => tonic::Status::new(code, code.description()),
        }
    }
}
