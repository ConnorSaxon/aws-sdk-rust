// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_point_in_time_recovery_specification(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PointInTimeRecoverySpecification) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.point_in_time_recovery_enabled {
        object.key("PointInTimeRecoveryEnabled").boolean(*var_1);
    }
    Ok(())
}

