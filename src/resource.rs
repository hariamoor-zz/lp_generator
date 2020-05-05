use serde::Deserialize;
use std::fmt::Debug;

// Uses the serde-rs (de)serialization framework to
// auto-generate code that parses the input XML file
// into the form specified by the given types.
// This doesn't use cyclic references for edges,
// but this can later be resolved with an
// associated hashtable.

#[derive(Debug, Deserialize, PartialEq)]
pub struct BasicNode {
    #[serde(rename = "nodename")]
    pub name: String,
    pub cost: f64,
    pub quality: f64,
    #[serde(rename = "and", default)]
    pub edges: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct KnobLayer {
    #[serde(rename = "basicnode")]
    pub basic_nodes: Vec<BasicNode>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "knob")]
pub struct Knob {
    #[serde(rename = "knobname")]
    pub knob_name: String,
    #[serde(rename = "knoblayer")]
    pub layers: Vec<KnobLayer>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "resource")]
pub struct Resource {
    #[serde(rename = "knob")]
    pub knobs: Vec<Knob>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::from_str;

    // Tests are constructed from
    // sample XML files provided
    // in the project.

    #[test]
    fn test_single() {
        let s = r##"
            <resource>
                <knob>
                    <knobname>S1</knobname>
                    <knoblayer>
                        <basicnode>
                            <nodename>S1_0</nodename>
                            <cost>10</cost>
                            <quality>10</quality>
                        </basicnode>
                    </knoblayer>
                </knob>
            </resource>
        "##;

        let should_be = Resource {
            knobs: vec![Knob {
                knob_name: "S1".to_string(),
                layers: vec![KnobLayer {
                    basic_nodes: vec![BasicNode {
                        name: "S1_0".to_string(),
                        cost: 10.0,
                        quality: 10.0,
                        edges: Vec::new(),
                    }],
                }],
            }],
        };

        let value: Resource = from_str(s).unwrap();
        assert_eq!(value, should_be);
    }

    #[test]
    fn test_large() {
        let s = r##"
            <resource>
                <knob>
                    <knobname>S2</knobname>
                    <knoblayer>
                        <basicnode>
                            <nodename>S2_0</nodename>
                            <cost>10</cost>
                            <quality>10</quality>
                            <and>S1_0</and>
                            <and>S1_1</and>
                            <and>S1_2</and>
                        </basicnode>
                    </knoblayer>
                    <knoblayer>
                        <basicnode>
                            <nodename>S2_1</nodename>
                            <cost>20</cost>
                            <quality>20</quality>
                            <and>S1_2</and>
                            <and>S1_3</and>
                        </basicnode>
                    </knoblayer>
                </knob>
            </resource>
        "##;

        let should_be = Resource {
            knobs: vec![Knob {
                knob_name: "S2".to_string(),
                layers: vec![
                    KnobLayer {
                        basic_nodes: vec![BasicNode {
                            name: "S2_0".to_string(),
                            cost: 10.0,
                            quality: 10.0,
                            edges: vec!["S1_0".to_string(), "S1_1".to_string(), "S1_2".to_string()],
                        }],
                    },
                    KnobLayer {
                        basic_nodes: vec![BasicNode {
                            name: "S2_1".to_string(),
                            cost: 20.0,
                            quality: 20.0,
                            edges: vec!["S1_2".to_string(), "S1_3".to_string()],
                        }],
                    },
                ],
            }],
        };

        let value: Resource = from_str(s).unwrap();
        assert_eq!(value, should_be);
    }
}
