mod error;
pub mod nodeshape;

#[cfg(test)]
mod tests {
    use sophia::iri::IriRef;
    use crate::nodeshape::{NodeShape, Property, Serializer};

    #[test]
    //For now test only used to check string output from valid input in terminal. Real tests to come.
    fn a_test() { 
        let fonds01 = NodeShape::new(
            IriRef::new("http://example.com/shapes/FondsShape".to_owned()).unwrap(),
            IriRef::new("https://www.ica.org/standards/RiC/ontology#RecordSet".to_string()).unwrap(),
            vec![
                Property::new(IriRef::new("http://www.w3.org/2000/01/rdf-schema#label".to_owned()).unwrap())
            ]
        );
        let ttl = fonds01.to_shacl_string(Serializer::Jsonld).unwrap(); 
        print!("Results:\n{}", ttl);    
        assert_eq!(1, 2)
    }
}
