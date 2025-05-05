pub mod property;
pub mod nodeshape;

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::nodeshape::{NodeShape, Serializer};
    use crate::property::{Property, DataType};

    #[test]
    //For now test only used to check string output from valid input in terminal. Real tests to come.
    fn a_test() { 
        let mut label = Property::new(
            "http://www.w3.org/2000/01/rdf-schema#label"
        ).unwrap();

        label.add_min_count(1);
        label.add_data_type(DataType::String);

        let fonds01 = NodeShape::new(
            "http://example.com/shapes/FondsShape",
            "https://www.ica.org/standards/RiC/ontology#RecordSet",
            vec![label]
        ).unwrap();
        let ttl = fonds01.to_shacl_string(Serializer::Jsonld).unwrap(); 
        print!("Results:\n{}", ttl);    
        assert_eq!(1, 2)
    }
}
