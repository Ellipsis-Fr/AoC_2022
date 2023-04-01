use AoC_2022::text_file_reader::{self, TextFileReader};


fn main() {
    println!("LANCE DEPUIS TEST.RS");
    // Example to use text_file_reader
    let mut text_file_reader = TextFileReader::new("test.txt");
    if let Ok(()) = text_file_reader.read_file_text() {
        println!("{}", text_file_reader.get_content());
    }
    
}