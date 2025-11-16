use crate::workflow::node_type::{
    AudioNode, DurationNode, EncoderNode, ImagePreprocessNode, InputLoaderNode, OcrNode,
    OutputWriterNode, TransitionNode,
};
use serde::Deserialize;
use serde_ordered_map::serde_ordered_map;

#[derive(Debug, Deserialize)]
pub struct WorkflowFile {
    #[serde(with = "serde_ordered_map")]
    pub node: Vec<(String, NodeType)>,
}

#[derive(Debug)]
pub struct Workflow {
    pub nodes: Vec<(String, NodeType)>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    InputLoader(InputLoaderNode),
    ImagePreprocess(ImagePreprocessNode),
    Ocr(OcrNode),
    Duration(DurationNode),
    Encoder(EncoderNode),
    Transition(TransitionNode),
    Audio(AudioNode),
    OutputWriter(OutputWriterNode),
}
