// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ms_smooth_additional_manifest(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MsSmoothAdditionalManifest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.manifest_name_modifier {
        object.key("manifestNameModifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.selected_outputs {
        let mut array_3 = object.key("selectedOutputs").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub(crate) fn de_ms_smooth_additional_manifest<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::MsSmoothAdditionalManifest>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::ms_smooth_additional_manifest::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "manifestNameModifier" => {
                                builder = builder.set_manifest_name_modifier(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "selectedOutputs" => {
                                builder = builder.set_selected_outputs(
                                    crate::protocol_serde::shape___list_of__string_min1::de___list_of__string_min1(tokens)?
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

