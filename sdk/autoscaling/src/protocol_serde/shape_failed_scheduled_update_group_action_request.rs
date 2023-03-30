// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_failed_scheduled_update_group_action_request(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::FailedScheduledUpdateGroupActionRequest, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::FailedScheduledUpdateGroupActionRequest::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ScheduledActionName") /* ScheduledActionName com.amazonaws.autoscaling#FailedScheduledUpdateGroupActionRequest$ScheduledActionName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_scheduled_action_name(var_1);
            }
            ,
            s if s.matches("ErrorCode") /* ErrorCode com.amazonaws.autoscaling#FailedScheduledUpdateGroupActionRequest$ErrorCode */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_error_code(var_2);
            }
            ,
            s if s.matches("ErrorMessage") /* ErrorMessage com.amazonaws.autoscaling#FailedScheduledUpdateGroupActionRequest$ErrorMessage */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_error_message(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

