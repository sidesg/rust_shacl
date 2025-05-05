use sophia::api::graph::{MutableGraph, Graph};
use sophia::api::term::{BnodeId, IriRef};
use sophia::inmem::graph::LightGraph;
use sophia::jsonld::JsonLdSerializer;
use sophia::turtle::serializer::turtle::TurtleSerializer;
use sophia::xml::serializer::RdfXmlSerializer;
use sophia::api::serializer::{TripleSerializer, QuadSerializer};
use sophia::api::prelude::Stringifier;
use uuid::Uuid;

use crate::property::{DataType, Property, TargetType};

pub struct NodeShape {
    shape_uri: IriRef<String>,
    taget_class: IriRef<String>,
    properties: Vec<Property>
}

impl NodeShape {
    pub fn new(shape_uri: &str, target_class: &str, properties: Vec<Property>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(NodeShape { 
            shape_uri: IriRef::new(shape_uri.to_string())?, 
            taget_class: IriRef::new(target_class.to_string())?, 
            properties: properties 
        })
    }


    pub fn to_shacl_string(&self, serializer: Serializer) -> Result<String, Box<dyn std::error::Error>> {
        let mut graph = LightGraph::new();

        graph.insert(
            &self.shape_uri, 
            IriRef::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), 
            IriRef::new_unchecked("http://www.w3.org/ns/shacl#NodeShape"))?;

        graph.insert(
            &self.shape_uri, 
            IriRef::new_unchecked("http://www.w3.org/ns/shacl#targetClass"), 
            &self.taget_class)?;

        for property in self.properties.iter() {
            let prop_bn = BnodeId::new_unchecked(Uuid::new_v4().to_string());
            graph.insert(
                &self.shape_uri, 
                IriRef::new_unchecked("http://www.w3.org/ns/shacl#property"), 
                &prop_bn)?;
            
            graph.insert(
                &prop_bn, 
                IriRef::new_unchecked("http://www.w3.org/ns/shacl#path"), 
                &property.path)?;

            if let Some(target) = &property.target {
                match target {
                    TargetType::Class(classes) => {
                        for class in classes.iter() {
                            graph.insert(
                                &prop_bn,
                                IriRef::new_unchecked("http://www.w3.org/ns/shacl#class"), 
                                class)?;
                        }
                    },
                    TargetType::DataType(datatype) => {
                        match datatype {
                            DataType::Integer => {
                                graph.insert(
                                    &prop_bn, 
                                    IriRef::new_unchecked("http://www.w3.org/ns/shacl#datatype"), 
                                    IriRef::new_unchecked("http://www.w3.org/2001/XMLSchema#integer")
                                )?;
                            },
                            DataType::String => {
                                graph.insert(
                                    &prop_bn, 
                                    IriRef::new_unchecked("http://www.w3.org/ns/shacl#datatype"), 
                                    IriRef::new_unchecked("http://www.w3.org/2001/XMLSchema#string")
                                )?;
                            }
                        }
                    }
                }

                if let Some(min_count) = property.min_count {
                    graph.insert(
                        &prop_bn, 
                        IriRef::new_unchecked("http://www.w3.org/ns/shacl#minCount"), 
                        min_count)?;
                }

                if let Some(max_count) = property.max_count {
                    graph.insert(
                        &prop_bn, 
                        IriRef::new_unchecked("http://www.w3.org/ns/shacl#maxCount"), 
                        max_count)?;
                }
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

pub enum Serializer {
    Ttl,
    Jsonld,
    Xml
}