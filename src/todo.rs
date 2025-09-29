use std::io::{self, Read, Write};
use std::fs::File;

struct Task {
    id: i32,
    description: String,
    done: bool,
}

fn task_print(tasks: &Vec<Task>){
    for task in tasks{
        let done = if task.done == true {"Ano"} else {"Ne"};
        println!("Id: {}\tPopis: {}\tSplněno: {}", task.id, task.description, done);
    }
}

fn tasks_to_file(path: &str ,tasks: &Vec<Task>) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;

    for task in tasks {
        writeln!(file, "{}\t{}\t{}", task.id, task.description, task.done)?;
    }

    Ok(())
}

pub fn todo_main() -> io::Result<()> {
    let mut input = String::new();
    let mut tasks: Vec<Task> = Vec::new();

    let file_path = "todo_list.txt";
    if !std::path::Path::new(file_path).exists() {
        println!("Soubor neexistuje, vytvářím nový");
        File::create(file_path)?;
    }

    let mut file = File::open(file_path)?;
    let mut contents = String::new(); 
    file.read_to_string(&mut contents)?;

    for line in contents.lines() {
        let task_info: Vec<&str> = line.split('\t').collect();

        if task_info.len() >= 3 {
            if let (Ok(task_id), Ok(task_done)) = (
                task_info[0].trim().parse::<i32>(),
                task_info[2].trim().parse::<bool>(),
            ) {
                let task_desc = task_info[1].trim().to_string();

                let task = Task {
                    id: task_id,
                    description: task_desc,
                    done: task_done,
                };

                tasks.push(task);
            }
        }
    }

    while input.trim() != "6" {
        input.clear();
        println!(" _________________________");
        println!("| Vítejte v todo manageru |");
        println!("| 1. Zobrazit úkoly       |");
        println!("| 2. Přidat úkol          |");
        println!("| 3. Přepnout stav úkolu  |");
        println!("| 4. Oddělat úkol         |");
        println!("| 5. Seřadit úkoly        |");
        println!("| 6. Zavřít aplikaci      |");
        println!("|_________________________|");

        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                task_print(&tasks);
            }
            "2" => {
                let ids: Vec<i32> = tasks.iter().map(|task| task.id).collect();
                println!("Zadej popis úkolu: ");

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let description = input.trim().to_string();
                
                for id in 1.. {
                    if !ids.contains(&id) {
                        let task = Task { id, description, done: false };
                        tasks.push(task);
                        break;
                    }
                }
            }
            "3" => {
                println!("Zadej id nebo název úkolu přepnutí stavu: ");
                let mut input: String = String::new();
                let temp = io::stdin().read_line(&mut input);
                if temp.is_err(){
                    println!("{}", temp.is_err());
                }
                let target: &str = input.trim();

                let mut found = false;

                if let Ok(target_id) = target.parse::<i32>() {
                    for task in tasks.iter_mut() {
                        if task.id == target_id {
                            task.done = !task.done;
                            found = true;
                            break;
                        }
                    }
                } else {
                    for task in tasks.iter_mut() {
                        if task.description.eq_ignore_ascii_case(target) {
                            task.done = !task.done;
                            found = true;
                            break;
                        }
                    }
                }

                if found {
                    println!("Úspěšné přepnutí");
                } else{
                    println!("Úkol nebyl nalezen")
                }

            }
            "4" => {
                println!("Zadej id nebo název úkolu přepnutí stavu: ");
                let mut input: String = String::new();
                let temp = io::stdin().read_line(&mut input);
                if temp.is_err(){
                    println!("{}", temp.is_err());
                }
                let target: &str = input.trim();

                let mut found = false;

                if let Ok(target_id) = target.parse::<i32>() {
                    for task in tasks.iter_mut() {
                        if task.id == target_id {
                            tasks.retain(|task| task.id != target_id);
                            found = true;
                            break;
                        }
                    }
                } else {
                    for task in tasks.iter_mut() {
                        if task.description.eq_ignore_ascii_case(target) {
                            tasks.retain(|task| !task.description.eq_ignore_ascii_case(target));
                            found = true;
                            break;
                        }
                    }
                }

                if found {
                    println!("Úspěšné smazání");
                } else{
                    println!("Úkol nebyl nalezen")
                }
            }
            "5" => {
                println!("Zadejte:\n
                    1 pro řadění podle id\n
                    2 pro řadění podle id pozpátku\n
                    3 pro řadění podle popisu alfabeticky\n
                    4 pro řadění podle popisu alfabeticky pozpátku");
                let mut text_input: String = String::new();
                io::stdin().read_line(&mut text_input).expect("Nelze přečíst");
                let input: &str = text_input.trim();
                match input.trim(){
                    "1" => {
                        tasks.sort_by_key(|task| task.id);
                    }
                    "2" => {
                        tasks.sort_by(|a, b| b.id.cmp(&a.id));
                    }
                    "3" => {
                        tasks.sort_by_key(|task| task.description.to_lowercase());
                    }
                    "4" => {
                        tasks.sort_by_key(|task| std::cmp::Reverse(task.description.to_lowercase()));
                    }
                    _ => {
                        println!("Špatný vstup");
                    }
                } 
            }
            "6" => {
                println!("Aplikace se zavírá");
            }
            _ => {
                println!("Špatný výběr, zkus znova\n");
            }
        }
        let a = tasks_to_file(file_path, &tasks);
        if a.is_err(){
            println!("Chyba");
        }
    }

    Ok(())
}