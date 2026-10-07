use crate::{Finding, NyvraxError, ScanContext};

pub trait NyvraxRule: Send + Sync {
    fn id(&self) -> &'static str;

    fn name(&self) -> &'static str;

    fn scan(&self, context: &ScanContext) -> Result<Vec<Finding>, NyvraxError>;
}
