use crate::workflow::node_type::{
    AudioNode, DurationNode, EncoderNode, ImagePreprocessNode, InputLoaderNode, OcrNode,
    OutputWriterNode, TransitionNode,
};
use indexmap::IndexMap;
use serde::Deserialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct WorkflowFile {
    pub node: IndexMap<String, NodeType>,
}

#[derive(Debug)]
pub struct Workflow {
    pub nodes: IndexMap<String, NodeType>,
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
