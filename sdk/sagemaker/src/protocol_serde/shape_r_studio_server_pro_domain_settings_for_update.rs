// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_r_studio_server_pro_domain_settings_for_update(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RStudioServerProDomainSettingsForUpdate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.domain_execution_role_arn {
        object.key("DomainExecutionRoleArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.default_resource_spec {
        #[allow(unused_mut)]
        let mut object_3 = object.key("DefaultResourceSpec").start_object();
        crate::protocol_serde::shape_resource_spec::ser_resource_spec(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.r_studio_connect_url {
        object.key("RStudioConnectUrl").string(var_4.as_str());
    }
    if let Some(var_5) = &input.r_studio_package_manager_url {
        object.key("RStudioPackageManagerUrl").string(var_5.as_str());
    }
    Ok(())
}

