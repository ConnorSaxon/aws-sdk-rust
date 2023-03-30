// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_third_party_job_failure_result_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutThirdPartyJobFailureResultInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.job_id {
        object.key("jobId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_token {
        object.key("clientToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.failure_details {
        #[allow(unused_mut)]
        let mut object_4 = object.key("failureDetails").start_object();
        crate::protocol_serde::shape_failure_details::ser_failure_details(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

