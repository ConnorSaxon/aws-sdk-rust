// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_constraint_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateConstraintInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.accept_language {
        object.key("AcceptLanguage").string(var_1.as_str());
    }
    if let Some(var_2) = &input.portfolio_id {
        object.key("PortfolioId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.product_id {
        object.key("ProductId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.parameters {
        object.key("Parameters").string(var_4.as_str());
    }
    if let Some(var_5) = &input.r#type {
        object.key("Type").string(var_5.as_str());
    }
    if let Some(var_6) = &input.description {
        object.key("Description").string(var_6.as_str());
    }
    if let Some(var_7) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_7.as_str());
    }
    Ok(())
}

