// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_on_demand_replication_run_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartOnDemandReplicationRunInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.replication_job_id {
        object.key("replicationJobId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    Ok(())
}

