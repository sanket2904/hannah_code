use crossterm::{
    style::{Color,ResetColor, SetForegroundColor},
    ExecutableCommand,
};

#[derive(Debug,PartialEq)]
pub enum PrintCommand {
    AICall, 
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = std::io::stdout();
        let statement_color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("Agent {}: ", agent_pos);
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);
        stdout.execute(ResetColor).unwrap();
    } 
}



pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = std::io::stdout();
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);
    stdout.execute(ResetColor).unwrap();
    let mut user_response = String::new();
    std::io::stdin().read_line(&mut user_response).expect("Failed to read line");
    return user_response.trim().to_string();
} 


pub fn confirm_safe_to_proceed() -> bool {
    let mut stdout: std::io::Stdout = std::io::stdout();
    loop {
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        println!("Are you sure you want to proceed? (y/n)");
        stdout.execute(ResetColor).unwrap();
        let mut user_response = String::new();
        std::io::stdin().read_line(&mut user_response).expect("Failed to read line");
        let user_response = user_response.trim().to_string();
        if user_response == "y" {
            return true;
        } else if user_response == "n" {
            return false;
        }
    }
}