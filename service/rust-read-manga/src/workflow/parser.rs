use crate::workflow::models::{Workflow, WorkflowFile};
use anyhow::Result;
use std::fs;
use toml::Value;

fn merge_values(base: &mut Value, override_val: &Value) {
    match (&mut *base, override_val) {
        (Value::Table(base_table), Value::Table(override_table)) => {
            for (key, override_value) in override_table {
                if let Some(base_value) = base_table.get_mut(key) {
                    merge_values(base_value, override_value);
                } else {
                    base_table.insert(key.clone(), override_value.clone());
                }
            }
        }
        (base_val, _) => *base_val = override_val.clone(),
    }
}

pub fn parse_workflow(file_path: &str) -> Result<Workflow> {
    let config_path = "config.toml";
    let mut base_config = fs::read_to_string(config_path)
        .and_then(|content| toml::from_str(&content).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
        .unwrap_or_else(|_| Value::Table(toml::map::Map::new()));

    let file_content = fs::read_to_string(file_path)?;
    let workflow_config: Value = toml::from_str(&file_content)?;

    merge_values(&mut base_config, &workflow_config);

    let workflow_file: WorkflowFile = base_config.try_into()?;

    let workflow = Workflow {
        nodes: workflow_file.node,
    };

    Ok(workflow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::models::NodeType;

    #[test]
    fn test_merge_workflow() {
        let base_toml = r#"
            [node.encoder]
            codec = "h265"
            fps = 30
        "#;

        let override_toml = r#"
            [node.encoder]
            codec = "h264"
            crf = 25
        "#;

        let mut base_value: Value = toml::from_str(base_toml).unwrap();
        let override_value: Value = toml::from_str(override_toml).unwrap();

        merge_values(&mut base_value, &override_value);

        let merged: WorkflowFile = base_value.try_into().unwrap();
        let encoder_node = &merged.node.get("encoder").unwrap();

        if let NodeType::Encoder(encoder) = encoder_node {
            assert_eq!(encoder.codec, crate::workflow::node_type::VideoCodec::H264);
            assert_eq!(encoder.fps, 30);
            assert_eq!(encoder.crf, Some(25));
        } else {
            panic!("Expected EncoderNode");
        }
    }
}