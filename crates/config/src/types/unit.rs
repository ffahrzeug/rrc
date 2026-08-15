use crate::types::execspec::ExecSpec;

pub struct Unit {
    pub service: core::service::Service,
    pub exec: ExecSpec,
}