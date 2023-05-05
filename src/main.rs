use std::io::stdin;

fn main() {

let mut input_use = String::new();
let mut input = String::new();
let mut list = Vec::new();
let stdin = stdin();

loop {  
    input.clear();
    print_info();

    stdin.read_line(&mut input).unwrap();

    let Ok(opt) = input.trim().parse() else {
        println!("Error: Invalid option (must be a number).");
        continue;
    };

    
    match opt{
        1 => showing_list(&list),
        2 => {
            println!("what u wanna add?");
            input_use.clear();        
            
            stdin.read_line(&mut input_use).unwrap();

            list.push(input_use.to_owned());
            showing_list(&list);
        },
        3 => {
            
            println!("Write a position");
            input_use.clear();
            
            stdin.read_line(&mut input_use).unwrap();

            let Ok(pos) = input_use.trim().parse() else{
                println!("Error: (must be a number).");
                continue;
            };
            
            remove_from_list(pos, &mut list);
            showing_list(&list);
        },
        4 => {
            print!("ok, cya!!");
            break 
        },
        _ => print!("invalid option")
    }

}

}

fn print_info() {
    println!(
        "\
| ToDO app |
| 1: Show  |
| 2: Add   |
| 3: Remove|
| 4: Exit  |"
    );
}

fn showing_list(list: &Vec<String>){

    for i in 0..list.len() {
        print!(" pos {i} : {}",list[i])
    }
}

fn remove_from_list(pos : i32, list: &mut Vec<String>){
    if pos < list.len() as i32{
        list.remove(pos as usize);
    }
    else{
        println!("invalid position");
    }
}
