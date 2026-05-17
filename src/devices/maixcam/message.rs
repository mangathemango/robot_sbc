use crate::devices::{
    maixcam::circle::MaixcamCircle, 
    utils::SerialMessage
};

#[derive(Debug, Clone)]
pub enum MaixcamMessage {
    CircleData(Vec<MaixcamCircle>),
}
