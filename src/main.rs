mod text_file_reader;
use text_file_reader::TextFileReader;


fn main() {
    // Example to use text_file_reader
    let mut text_file_reader = TextFileReader::new("test.txt");
    if let Ok(()) = text_file_reader.read_file_text() {
        println!("{}", text_file_reader.get_content());
    }
    
}
