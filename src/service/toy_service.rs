#[derive(Debug)]
pub struct Barco {

    pub hundido : bool,
    pub casillas : Vec<i16>
}

impl Barco{

    pub fn explotar(&mut self) -> &bool {
        
        match self {
            // , .. to match any value for the remaining fields.
            Barco { hundido : false, .. } => {self.hundido = true },
            _ => {},
        }
           
        return &self.hundido;
    }
}


