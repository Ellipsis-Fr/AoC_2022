use AoC_2022::text_file_reader::TextFileReader;
use instruction::Instructions;

mod instruction;

fn main() {
    println!("Puzzle du 10/12 Partie 2");
    
    let puzzle = get_puzzle();
    let crt_img = get_crt_img(puzzle);
    print_crt_img(crt_img);
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("10_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_crt_img(puzzle: Vec<String>) -> Vec<String> {
    let mut crt_img = vec![];
    let mut crt_line = String::from("");

    let mut x = 1;

    for command in puzzle {
        let instruction = Instructions::new(command);

        match instruction {
            Instructions::Noop => edit_crt_line(&mut crt_line, x),
            Instructions::AddX(v) => {
                for _ in 0..2 {
                    edit_crt_line(&mut crt_line, x);
                    may_init_new_line(&mut crt_img, &mut crt_line);
                }
                x += v;
            }
        }

        may_init_new_line( &mut crt_img, &mut crt_line);
    }

    crt_img
}

fn may_init_new_line(crt_img: &mut Vec<String>, crt_line: &mut String) {
    if !crt_line.is_empty() && crt_line.len() % 40 == 0 {
        crt_img.push(crt_line.clone());
        *crt_line = String::from("");
    }
}

fn edit_crt_line(crt_line: &mut String, x: i32) {
    let crt_actual_position = crt_line.len() as i32;

    if x - 1 == crt_actual_position || x == crt_actual_position || x + 1 == crt_actual_position {
        crt_line.push('#');
    } else {
        crt_line.push('.');
    }
}

fn print_crt_img(crt_img: Vec<String>) {
    crt_img.iter().for_each(|img| println!("{img}"));
}