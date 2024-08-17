#[derive(Debug)]
pub enum Symbol {
    Initial,
    Id(String),
    Terminal(Terminal)
}

#[derive(Debug)]
pub enum Terminal {
    Program,
    Semicolon
}