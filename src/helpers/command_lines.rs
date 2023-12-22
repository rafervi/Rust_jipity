use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};


#[derive(PartialEq, Debug)]

pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout = stdout();

        //Deice on the print color
        let statement_color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };
        //Print the agent statement
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {agent_pos}");

        //Make selected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{agent_statement}");

        //Reset color
        stdout.execute(ResetColor).unwrap();
        println!("{agent_statement}");
    }
}
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    //Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{question}");

    //Reset the color
    stdout.execute(ResetColor).unwrap();

    //Reset user input
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    //Trim whitespace and return

    return user_response.trim().to_string();
}

//Get user response the code is safe to execute

pub fn confirm_safe_code() -> bool{
    let mut stdout = stdout();
    loop {
        // Print the question in specified color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        println!("WARNING: You are about to run code written entirely by AI");
        println!("Review your code and confirm you wish to continue");

        // Reset Color
        stdout.execute(ResetColor).unwrap();
        // Present Options with different colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good");
        stdout.execute(SetForegroundColor(Color::DarkGreen)).unwrap();
        println!("[1] Stop this project");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        // Read user input
        let mut human_response = String::new();
        stdin()
            .read_line(&mut human_response)
            .expect("Failed to read response");

        // Trim whitespace and convert to lowercase
        let human_response = human_response.trim().to_lowercase();

        // Match response
        match human_response.as_str (){
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input. Select '1' or '2' ")
            }

        }

    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_print_agent_msg() {
        PrintCommand::AICall
            .print_agent_message("Managing Agent: ", "Testing testing, processing something")
    }
}
