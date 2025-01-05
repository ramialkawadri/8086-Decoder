use std::fmt::Display;

#[derive(Clone)]
pub struct Flags {
    pub zf: bool,
    pub sf: bool,
}

impl Flags {
    pub fn update_from_number(&mut self, number: i16) {
        self.zf = number == 0;
        self.sf = number < 0;
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from("");

        if self.zf {
            string += "Z";
        }
        if self.sf {
            string += "S";
        }

        write!(f, "{}", string)
    }
}
