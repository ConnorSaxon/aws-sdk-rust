// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_provisioned_product_plan_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeProvisionedProductPlanInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.accept_language {
        object.key("AcceptLanguage").string(var_1.as_str());
    }
    if let Some(var_2) = &input.plan_id {
        object.key("PlanId").string(var_2.as_str());
    }
    if input.page_size != 0 {
        object.key("PageSize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.page_size).into()));
    }
    if let Some(var_3) = &input.page_token {
        object.key("PageToken").string(var_3.as_str());
    }
    Ok(())
}

