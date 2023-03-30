// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_salesforce_destination_properties(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SalesforceDestinationProperties) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.object {
        object.key("object").string(var_1.as_str());
    }
    if let Some(var_2) = &input.id_field_names {
        let mut array_3 = object.key("idFieldNames").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.error_handling_config {
        #[allow(unused_mut)]
        let mut object_6 = object.key("errorHandlingConfig").start_object();
        crate::protocol_serde::shape_error_handling_config::ser_error_handling_config(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.write_operation_type {
        object.key("writeOperationType").string(var_7.as_str());
    }
    if let Some(var_8) = &input.data_transfer_api {
        object.key("dataTransferApi").string(var_8.as_str());
    }
    Ok(())
}

pub(crate) fn de_salesforce_destination_properties<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::SalesforceDestinationProperties>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::salesforce_destination_properties::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "object" => {
                                builder = builder.set_object(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "idFieldNames" => {
                                builder = builder.set_id_field_names(
                                    crate::protocol_serde::shape_id_field_name_list::de_id_field_name_list(tokens)?
                                );
                            }
                            "errorHandlingConfig" => {
                                builder = builder.set_error_handling_config(
                                    crate::protocol_serde::shape_error_handling_config::de_error_handling_config(tokens)?
                                );
                            }
                            "writeOperationType" => {
                                builder = builder.set_write_operation_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::WriteOperationType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "dataTransferApi" => {
                                builder = builder.set_data_transfer_api(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::SalesforceDataTransferApi::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

