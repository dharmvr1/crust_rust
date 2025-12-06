use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}
impl File {
    pub fn new(name: &str) -> Self {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
    pub fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    pub fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Closed => write!(f, "OPEN"),
            FileState::Open => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}
fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f1 = File::new_with_data("input.txt", &f3_data);
    println!("{}", f1);

    let mut buffer = vec![];
    f1 = open(f1).unwrap();
    let f1_length = f1.read(&mut buffer).unwrap();
    f1 = close(f1).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f1);

    println!("{} is bytes {} long", &f1.name, f1_length);
    println!("{}", text);
}
