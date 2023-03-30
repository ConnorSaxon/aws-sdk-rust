// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_replication_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateReplicationJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.replication_job_id {
        object.key("replicationJobId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.frequency {
        object.key("frequency").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.next_replication_run_start_time {
        object.key("nextReplicationRunStartTime").date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.license_type {
        object.key("licenseType").string(var_4.as_str());
    }
    if let Some(var_5) = &input.role_name {
        object.key("roleName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.description {
        object.key("description").string(var_6.as_str());
    }
    if let Some(var_7) = &input.number_of_recent_amis_to_keep {
        object.key("numberOfRecentAmisToKeep").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    if let Some(var_8) = &input.encrypted {
        object.key("encrypted").boolean(*var_8);
    }
    if let Some(var_9) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_9.as_str());
    }
    Ok(())
}

