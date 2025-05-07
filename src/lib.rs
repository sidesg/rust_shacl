pub mod property;
pub mod nodeshape;

#[cfg(test)]
mod tests {
    use crate::nodeshape::NodeShape;
    use crate::property::Property;

    #[test]
    //For now test only used to check string output from valid input in terminal. Real tests to come.
    fn a_test() { 
        let mut label = Property::new(
            "http://www.w3.org/2000/01/rdf-schema#label"
        ).unwrap();

        label.add_min_count(1);
        label.add_datatype_string();

        let fonds01 = NodeShape::new(
            "http://example.com/shapes/FondsShape",
            "https://www.ica.org/standards/RiC/ontology#RecordSet",
            vec![label]
        ).unwrap();
        let jsonld = fonds01.serialize_jsonld().unwrap(); 
        print!("Jsonld:\n{}\n", jsonld);

        let ttl = fonds01.serialize_ttl().unwrap(); 
        print!("Ttl:\n{}\n", ttl);

        let xml = fonds01.serialize_xml().unwrap(); 
        print!("Xml:\n{}\n", xml);
        
        assert_eq!(1, 2)
    }
}
