use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

use csv::Reader;
use dotenvy::dotenv;
use llm_chain::{executor, parameters, prompt, step::Step};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let exec = executor!()?;

    let file = File::open("data.csv")?;
    let mut reader = Reader::from_reader(file);

    let mut csv_data = String::new();
    for result in reader.records() {
        let record = result?;
        csv_data.push_str(&record.iter().collect::<Vec<_>>().join(","));
        csv_data.push('\n');
    }

    loop {
        println!("Enter your prompt (or 'quit' to exit):");
        io::stdout().flush()?;

        let mut user_prompt = String::new();
        io::stdin().read_line(&mut user_prompt)?;
        user_prompt = user_prompt.trim().to_string();

        if user_prompt.to_lowercase() == "quit" {
            break;
        }
        let prompt_string = format!(
            "You are a data analyst tasked with analyzing a CSV file containing information about individuals, including their name, age, occupation, city, favorite sport, and annual income. Your goal is to provide clear and concise answers to the given questions based on the data provided.

        Question: {}\n\nCSV Data:\n{}",
            user_prompt, csv_data
        );

        let step = Step::for_prompt_template(prompt!("{}", &prompt_string));

        let res = step.run(&parameters!(), &exec).await?;
        println!("{}", res.to_immediate().await?.as_content());
    }

    Ok(())
}
