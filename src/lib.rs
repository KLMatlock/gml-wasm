mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use gml_parser::{Edge, GMLObject, Graph, Node};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}


#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmNode {
    pub id: i64,
    label: Option<String>,
}

#[wasm_bindgen]
impl WasmNode {
    #[wasm_bindgen(getter)]
    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }
}


#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmEdge {
    pub source: i64,
    pub target: i64,
    label: Option<String>,
}

#[wasm_bindgen]
impl WasmEdge {
    #[wasm_bindgen(getter)]
    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }
}

#[wasm_bindgen]
pub struct WasmGraph {
    pub directed: Option<bool>,
    pub id: Option<i64>,
    label: Option<String>,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    //attrs: Vec<(String, GMLValue)>,
}

#[wasm_bindgen]
impl WasmGraph {
    #[wasm_bindgen(getter)]
    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    #[wasm_bindgen(getter)]
    pub fn edges(&self) -> Vec<WasmEdge> {
        self.edges.iter().map(|edge| WasmEdge {
            source: edge.source,
            target: edge.target,
            label: edge.label.clone(),
        }).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn nodes(&self) -> Vec<WasmNode> {
        self.nodes.iter().map(|node| WasmNode {
            id: node.id,
            label: node.label.clone(),
        }).collect()
    }

    #[wasm_bindgen]
    pub fn from_gml_string(data: &str) -> Result<WasmGraph, JsError> {
        parse_gml(data)
    }
}


#[wasm_bindgen]
pub fn parse_gml(data: &str) -> Result<WasmGraph, JsError> {
    set_panic_hook();
    let root = GMLObject::from_str(data)?;
    let graph = Graph::from_gml(root)?;
    Ok(WasmGraph {
        directed: graph.directed,
        id: graph.id,
        label: graph.label,
        nodes: graph.nodes,
        edges: graph.edges,
    })
}
