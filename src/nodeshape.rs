use sophia::api::graph::{MutableGraph, Graph};
use sophia::api::term::{BnodeId, IriRef};
use sophia::inmem::graph::LightGraph;
use sophia::jsonld::JsonLdSerializer;
use sophia::turtle::serializer::turtle::TurtleSerializer;
use sophia::xml::serializer::RdfXmlSerializer;
use sophia::api::serializer::{TripleSerializer, QuadSerializer};
use sophia::api::prelude::Stringifier;
use uuid::Uuid;

pub struct NodeShape {
    shape_uri: IriRef<String>,
    taget_class: IriRef<String>,
    properties: Vec<Property>
}

impl NodeShape {
    pub fn new(shape_uri: IriRef<String>, target_class: IriRef<String>, properties: Vec<Property>) -> Self {
        NodeShape { shape_uri: shape_uri, taget_class: target_class, properties: properties }
    }


    pub fn to_shacl_string(&self, serializer: Serializer) -> Result<String, Box<dyn std::error::Error>> {
        let mut graph = LightGraph::new();

        graph.insert(
            &self.shape_uri, 
            IriRef::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_owned()), 
            IriRef::new_unchecked("http://www.w3.org/ns/shacl#NodeShape".to_string()))?;

        graph.insert(
            &self.shape_uri, 
            IriRef::new_unchecked("http://www.w3.org/ns/shacl#targetClass".to_owned()), 
            &self.taget_class)?;

        for property in self.properties.iter() {
            let prop_bn = BnodeId::new_unchecked(Uuid::new_v4().to_string());
            graph.insert(
                &self.shape_uri, 
                IriRef::new_unchecked("http://www.w3.org/ns/shacl#property".to_owned()), 
                &prop_bn)?;

            graph.insert(
                &prop_bn, 
                IriRef::new_unchecked("http://www.w3.org/ns/shacl#path".to_owned()), 
                &property.path)?;

            match &property.ptype {
                PropertyType::Class(classes) => {
                    for class in classes.iter() {
                        graph.insert(
                            &prop_bn,
                            IriRef::new_unchecked("http://www.w3.org/ns/shacl#class".to_owned()), 
                            class)?;
                    }
                },
                PropertyType::MaxCount(_max_count) => {},
                PropertyType::MinCount(min_count) => {
                    graph.insert(
                        &prop_bn, 
                        IriRef::new_unchecked("http://www.w3.org/ns/shacl#minCount".to_owned()), 
                        *min_count)?;
                }
                PropertyType::DataType(_data_type) => {}
            }
        };

        match serializer {
            Serializer::Jsonld => {
                self.serialize_jsonld(graph)
            },
            Serializer::Ttl => {
                self.serialize_ttl(graph)
            },
            Serializer::Xml => {
                self.serialize_xml(graph)
            }
        }
    }

    fn serialize_jsonld(&self, graph: LightGraph) -> Result<String, Box<dyn std::error::Error>> {
        let mut serializer = JsonLdSerializer::new_stringifier();
        let text = serializer
            .serialize_dataset(&graph.as_dataset())?
            .to_string();
        Ok(text)
    }

    fn serialize_ttl(&self, graph: LightGraph) -> Result<String, Box<dyn std::error::Error>> {
        let mut serializer = TurtleSerializer::new_stringifier();
        let text = serializer
            .serialize_graph(&graph)?
            .to_string();
        Ok(text)
    }

    fn serialize_xml(&self, graph: LightGraph) -> Result<String, Box<dyn std::error::Error>> {
        let mut serializer = RdfXmlSerializer::new_stringifier();
        let text = serializer
            .serialize_graph(&graph)?
            .to_string();
        Ok(text)
    }
}

enum PropertyType {
    MaxCount(usize),
    MinCount(usize),
    Class(Vec<IriRef<String>>),
    DataType(DataTypes)
}

enum DataTypes {
    Integer,
    String
}

pub struct Property {
    path: IriRef<String>,
    ptype: PropertyType
}

impl Property {
    pub fn new_class_target(path: IriRef<String>, class_target: Vec<&str>) -> Self {
        Property { 
            path: path, 
            ptype: PropertyType::Class(
                class_target.iter()
                    .map(|t| IriRef::new(t.to_string()).unwrap())
                    .collect()
            ) 
        }
    }

    pub fn new_min_count(path: IriRef<String>, min_count: usize) -> Self {
        Property {
             path: path,
             ptype: PropertyType::MinCount(min_count)
        }
    }
}

pub enum Serializer {
    Ttl,
    Jsonld,
    Xml
}