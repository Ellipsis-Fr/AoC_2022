pub enum Instructions {
    Noop, AddX(i32)
}

impl Instructions {
    pub fn new(instruction: String) -> Self {
        let instruction_splitted = if instruction.contains(" ") {
            Some(instruction.split_whitespace().collect::<Vec<&str>>())
        } else { None };

        match instruction_splitted {
            None => { return Instructions::Noop },
            Some(v) => {
                return Instructions::AddX(v[1].to_string().parse().unwrap());
            }
        }
    }
}