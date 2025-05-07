use sophia::api::term::IriRef;   

pub struct Property {
    pub path: IriRef<String>,
    pub target: Option<TargetType>,
    pub min_count: Option<usize>,
    pub max_count: Option<usize>
}

impl Property {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Property { 
            path: IriRef::new(path.to_string())?,
            target: None,
            min_count: None,
            max_count: None
        })
    }

    pub fn add_class_target(&mut self, class_targets: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let mut targets = Vec::new();
        for target in class_targets.iter() {
            let iri = IriRef::new(target.to_string())?;
            targets.push(iri)
        }
        self.target = Some(TargetType::Class(targets));
        Ok(())
    }
    

    pub fn add_min_count(&mut self, min_count: usize) {
        self.min_count = Some(min_count);
    }

    pub fn add_max_count(&mut self, max_count: usize) {
        self.max_count = Some(max_count);
    }

    pub fn add_datatype_string(&mut self) {
        self.target = Some(TargetType::DataType(DataType::String));
    }

    pub fn add_datatype_int(&mut self) {
        self.target = Some(TargetType::DataType(DataType::Integer));
    }
}

pub enum DataType {
    Integer,
    String
}

pub enum TargetType {
    Class(Vec<IriRef<String>>),
    DataType(DataType)
}
