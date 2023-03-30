// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_continuous_deployment_policy_config_http_payload(payload: &std::option::Option<crate::model::ContinuousDeploymentPolicyConfig>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::protocol_serde::rest_xml_unset_payload()
    )};
    Ok(
        crate::protocol_serde::shape_update_continuous_deployment_policy_input::ser_continuous_deployment_policy_config_payload(payload)?
    )
}

pub fn ser_continuous_deployment_policy_config_payload(input: &crate::model::ContinuousDeploymentPolicyConfig) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
     {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
                                #[allow(unused_mut)]
                                let mut root = writer.start_el("ContinuousDeploymentPolicyConfig").write_ns("http://cloudfront.amazonaws.com/doc/2020-05-31/", None);
        crate::protocol_serde::shape_continuous_deployment_policy_config::ser_continuous_deployment_policy_config(input, root)?
    }
    Ok(out.into_bytes())
}

