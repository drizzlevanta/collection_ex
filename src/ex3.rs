use std::{collections::HashMap, io, process};

// Using a hash map and vectors, create a text interface to allow a user to add employee names to
// a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or
// all people in the company by department, sorted alphabetically.
const ACTIONS: [&'static str; 3] = ["add", "query", "exit"];
// const ADD_PROMPT: &str =
//     format!("To add an employee, type \" {ACTIONS[0]} <employee> to <department>\"");

pub fn demo() {
    take_inputs();
}

#[derive(Debug)]
struct Company {
    staff: HashMap<String, String>,
}

impl Company {
    //TODO error handling
    fn add(&mut self, inputs: Vec<&str>) {
        // let inputs: Vec<&str> = s.trim().split_whitespace().collect();
        self.staff
            .insert(inputs[1].to_owned(), inputs[3].to_owned());
    }

    fn staff_in_dept(&self, dept_name: &str) -> Vec<&String> {
        let iter = self.staff.iter();
        iter.filter(|(_, v)| v.as_str() == dept_name)
            .map(|(k, _)| k)
            .collect::<Vec<_>>()
    }

    fn all_staff(&self) -> Vec<(&String, &String)> {
        //TODO try binaryheap
        // let heap: BinaryHeap<_> = self.staff.iter().collect();
        // heap.into_sorted_vec()

        //BTreeMap sorts its element by key be default
        // let btree_map: BTreeMap<&String, &String> =
        //     self.staff.iter().map(|(k, v)| (v, k)).collect();
        // btree_map

        //sort Hashmap by converting to a vector first
        let mut hash_vec: Vec<(&String, &String)> = self.staff.iter().collect();
        hash_vec.sort_by(|a, b| a.1.cmp(b.1));
        hash_vec
    }
}

fn print_hint() {
    println!(
        "To add an employee, type \"{} <employee> to <department>\"",
        ACTIONS[0]
    );
    println!(
        "To query employees, type \"{}\" optionally followed by department name",
        ACTIONS[1]
    );
    println!("To exit, type \"{}\"", ACTIONS[2]);
}

fn take_inputs() {
    let mut company = Company {
        staff: HashMap::new(),
    };
    loop {
        print_hint();
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // let action = input.split_whitespace().next().unwrap();

        let input: Vec<_> = input.split_whitespace().collect();

        // let actions = ["add", "query", "exit"];
        // let add_action = "add";
        match input.as_slice() {
            ["add", ..] => {
                company.add(input);
            }
            ["query"] => {
                //TODO format printout
                println!("Listing all staff: {:?}", company.all_staff());
            }

            ["exit"] => {
                process::exit(0);
            }
            ["query", dept, ..] => {
                println!(
                    "Listing all staff in department {}: {:?}",
                    dept,
                    company.staff_in_dept(dept)
                );
            }
            _ => {
                println!("Unsupported command!");
            }
        }
        // match action.to_lowercase().as_str() {
        //     "add" => {
        //         company.add(&input);
        //     }
        //     "query" => {
        //         //TODO format printout
        //         println!("Listing all staff: {:?}", company.all_staff());
        //     }

        //     "exit" => {
        //         process::exit(0);
        //     }
        //     _ => {
        //         println!("Unsupported command!");
        //     }
        // }
    }
}
