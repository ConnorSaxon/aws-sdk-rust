// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_portfolio_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreatePortfolioInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.accept_language {
        object.key("AcceptLanguage").string(var_1.as_str());
    }
    if let Some(var_2) = &input.display_name {
        object.key("DisplayName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.provider_name {
        object.key("ProviderName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_9.as_str());
    }
    Ok(())
}

