use rust_read_manga::workflow::{engine::WorkflowEngine, parser::parse_workflow_file};

#[test]
fn test_parse_and_run_workflow() {
    let workflow = parse_workflow_file("tests/test_workflow.toml").unwrap();
    let engine = WorkflowEngine::new(workflow);
    engine.run().unwrap();
}
