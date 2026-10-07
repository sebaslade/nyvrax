use crate::ScanContext;

pub fn detected_frameworks(context: &ScanContext) -> &[String] {
    &context.project.frameworks
}
