use std::{rc::Rc, cell::{RefCell, RefMut}, fmt};

#[derive(Debug)]
pub struct TreeStructure {
    pub size: u32,
    pub folders: Vec<Rc<RefCell<Folder>>>,
}

impl TreeStructure {
    pub fn new() -> Self {
        Self {
            size: 0,
            folders: vec![],
        }
    }

    pub fn add(&mut self, folder: &Rc<RefCell<Folder>>) {
        self.folders.push(Rc::clone(folder));
    }

    /* pub fn get_folder_by_name(&mut self, name: &str) -> Option<RefMut<Folder>> {
        // for folder in &self.folders {
        //     if folder.borrow().name.eq(name) {
        //         return Some(folder.borrow_mut());
        //     }
        // }
        
        // None
        match self.folders.iter().find(|folder| folder.borrow().name.eq(name)) {
            None => None,
            Some(f) => Some(f.borrow_mut())
        }
    } */

    pub fn get_folder_by_name(&mut self, name: &str) -> Option<Rc<RefCell<Folder>>> {
        // for folder in &self.folders {
        //     if folder.borrow().name.eq(name) {
        //         return Some(folder.borrow_mut());
        //     }
        // }
        
        // None
        match self.folders.iter().find(|folder| folder.borrow().name.eq(name)) {
            None => None,
            Some(f) => Some(Rc::clone(f))
        }
    }
}

#[derive(Clone)]
pub struct Folder {
    pub name: String,
    pub size: u32,
    pub subfolders: Vec<Rc<RefCell<Folder>>>,
    pub files: Vec<File>,
    pub parent_folder: Option<Rc<RefCell<Folder>>>
}

impl Folder {
    pub fn new(name: &str, possible_parent: Option<&Rc<RefCell<Folder>>>) -> Self {
        let parent = match possible_parent {
            None => None,
            Some(parent) => Some(Rc::clone(parent))
        };

        Self {
            name: name.to_owned(),
            size: 0,
            subfolders: vec![],
            files: vec![],
            parent_folder: parent
        }
    }

    pub fn get_subfolder_by_name(&self, folder_name: &str) -> Rc<RefCell<Folder>> {
        println!("dossier actuel : {}, sous-dossier recherché : {}", self.name, folder_name);
        Rc::clone(self.subfolders.iter().find(|folder| folder.borrow().name.eq(folder_name)).unwrap())
    }

    pub fn add_subfolder(&mut self, folder: &Rc<RefCell<Folder>>) {
        self.subfolders.push(Rc::clone(folder));
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn folder_size_calculation(&mut self) {
        // Réinit pour chaque nouveau calcul car supposons que de nouveaux fichiers peuvent être ajouté dans les dossiers
        self.size = 0;


        for subfolder in &self.subfolders {
            subfolder.borrow_mut().folder_size_calculation();
            self.size += subfolder.borrow().size;
        }

        for file in &self.files {
            self.size += file.size;
        }
    }

    pub fn calculation_of_total_space_occupied_by_folders_of_less_than_100000_bytes(&self) -> u32 {
        let mut total_space = 0;

        if self.size < 100000 {
            total_space += self.size;
        }
        
        for subfolder in &self.subfolders {
            total_space += subfolder.borrow().calculation_of_total_space_occupied_by_folders_of_less_than_100000_bytes();
        }

        total_space
    }

    pub fn find_folder_to_delete(&self, space_to_find: u32, mut folder_to_delete: Folder) -> Folder {
        let mut actual_folder = self.clone();

        for subfolder in &self.subfolders {
            let current_folder = subfolder.borrow().find_folder_to_delete(space_to_find, folder_to_delete.clone());
            if current_folder.size >= space_to_find && current_folder.size <= folder_to_delete.size {
                actual_folder = current_folder.clone();
                folder_to_delete = current_folder;
            } 
        }

        actual_folder
    }

    // pub fn find_folder_to_delete(&self, space_to_find: u32) -> Folder {
    //     let mut current_folder = self.clone();

    //     for subfolder in &self.subfolders {
    //         if subfolder.borrow().size >= space_to_find && subfolder.borrow().size <= current_folder.size {
    //             current_folder = subfolder.borrow().find_folder_to_delete(space_to_find);
    //         } 
    //     }

    //     current_folder
    // }
}

impl fmt::Debug for Folder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Folder")
         .field("name", &self.name)
         .field("size", &self.size)
         .field("files", &self.files)
         .field("folders", &self.subfolders)
         .finish()
    }
}

#[derive(Clone)]
pub struct File {
    pub name: String,
    pub size: u32,
    pub parent: Rc<RefCell<Folder>>
}

impl File {
    pub fn new(name: &str, size: u32, parent: &Rc<RefCell<Folder>>) -> Self {
        Self {
            name: name.to_owned(),
            size,
            parent: Rc::clone(parent)
        }
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
         .field("name", &self.name)
         .field("size", &self.size)
         .field("parent", &self.parent.borrow().name)
         .finish()
    }
}