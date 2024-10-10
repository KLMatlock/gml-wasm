//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use gml_wasm::parse_gml;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn hello_gml() {
    let data = r#"
    graph [            
    id 4           
    node [         
        id 0       
    ]              
    node [         
        id 1       
    ]              
    edge [         
        source 1   
        target 0   
        label "Edge"
    ]              
    ]"#;
    let res = parse_gml(data).unwrap();
    let nodes = res.nodes();
    let edges = res.edges();
    assert!(res.directed.is_none());
    assert_eq!(res.id, Some(4));
    assert_eq!(res.label(), None);
    assert_eq!(nodes.len(), 2);
    assert_eq!(edges.len(), 1);
    assert_eq!(nodes[0].id, 0);
    assert_eq!(nodes[1].id, 1);
    assert_eq!(edges[0].source, 1);
    assert_eq!(edges[0].target, 0);
    assert_eq!(edges[0].label(), Some("Edge".to_string()));
}

#[wasm_bindgen_test]
fn invalid_gml() {
    let data = r#"asdf"#;
    let res = parse_gml(data);
    assert!(res.is_err());
}
