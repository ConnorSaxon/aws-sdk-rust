// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_inference_output_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InferenceOutputConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.s3_output_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("S3OutputConfiguration").start_object();
        crate::protocol_serde::shape_inference_s3_output_configuration::ser_inference_s3_output_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_inference_output_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::InferenceOutputConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::inference_output_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "S3OutputConfiguration" => {
                                builder = builder.set_s3_output_configuration(
                                    crate::protocol_serde::shape_inference_s3_output_configuration::de_inference_s3_output_configuration(tokens)?
                                );
                            }
                            "KmsKeyId" => {
                                builder = builder.set_kms_key_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
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

