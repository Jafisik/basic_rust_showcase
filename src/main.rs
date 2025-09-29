use std::io;
use tokio::sync::mpsc;

mod todo;
mod calc;
mod timer;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut input = String::new();


    while input.trim() != "4" {
        input.clear();
        println!("\n1. ToDo list\n2. Kalkulačka\n3. Ovládání časovače\n4. Konec");
        io::stdin().read_line(&mut input).expect("Špatný input");

        match input.trim() {
            "1" => {
                if let Err(e) = todo::todo_main() {
                    println!("{}", e);
                }
            }
            "2" => {
                println!("Zadej příklad:");
                let mut input1 = String::new();
                io::stdin().read_line(&mut input1).expect("Chyba při čtení");
                if let Err(e) = calc::calculator(input1.trim()) {
                    println!("{}", e);
                }
            }
            "3" => {
                println!("Vítej ve hře, kde budeš hádat uplynulou dobu");

                println!("Zadej čas v sekundách, který chceš hádat: ");
                let mut cmd_input = String::new();
                io::stdin().read_line(&mut cmd_input).unwrap();
                let seconds: u64 = match cmd_input.trim().parse(){
                    Ok(s) => s,
                    Err(_) => {
                        println!("Není číslo");
                        continue;
                    }
                    
                };
                let (tx, rx) = mpsc::channel(10);
                tokio::spawn(timer::timer(rx,  seconds));

                println!("Zmáčkni enter jakmile si myslíš, že je čas na nule");
                let mut cmd_input = String::new();
                io::stdin().read_line(&mut cmd_input).unwrap();

                tx.send(timer::Command::Stop).await.unwrap();
            }
            "4" => {
                println!("Aplikace se vypíná");
            }
            _ => {
                println!("Neplatná volba");
            }
        }
    }

    Ok(())
}
