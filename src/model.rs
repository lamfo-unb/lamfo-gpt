use crate::robert::{Conv, Robert};
use crate::error::Result;

#[derive(Clone)]
pub struct RobertController {
    pub robert: Robert,
    pub conv: Conv,
}

impl RobertController {
    pub async fn new(robert: Robert, conv: Conv) -> Result<Self> {
        Ok(Self { robert, conv })
    }
}