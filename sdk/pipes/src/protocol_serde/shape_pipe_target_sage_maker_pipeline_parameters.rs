// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_pipe_target_sage_maker_pipeline_parameters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PipeTargetSageMakerPipelineParameters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pipeline_parameter_list {
        let mut array_2 = object.key("PipelineParameterList").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_sage_maker_pipeline_parameter::ser_sage_maker_pipeline_parameter(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub(crate) fn de_pipe_target_sage_maker_pipeline_parameters<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PipeTargetSageMakerPipelineParameters>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::pipe_target_sage_maker_pipeline_parameters::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "PipelineParameterList" => {
                                builder = builder.set_pipeline_parameter_list(
                                    crate::protocol_serde::shape_sage_maker_pipeline_parameter_list::de_sage_maker_pipeline_parameter_list(tokens)?
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

