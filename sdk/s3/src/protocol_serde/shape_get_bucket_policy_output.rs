// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_policy_payload(body: &[u8]) -> std::result::Result<std::option::Option<std::string::String>, crate::error::GetBucketPolicyError> {
    (!body.is_empty()).then(||{
        let body_str = std::str::from_utf8(body).map_err(crate::error::GetBucketPolicyError::unhandled)?;
        Ok(body_str.to_string())
    }).transpose()
}

