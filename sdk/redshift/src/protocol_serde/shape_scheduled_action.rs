// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_scheduled_action(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ScheduledAction, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ScheduledAction::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ScheduledActionName") /* ScheduledActionName com.amazonaws.redshift#ScheduledAction$ScheduledActionName */ =>  {
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
            s if s.matches("TargetAction") /* TargetAction com.amazonaws.redshift#ScheduledAction$TargetAction */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_scheduled_action_type::de_scheduled_action_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_target_action(var_2);
            }
            ,
            s if s.matches("Schedule") /* Schedule com.amazonaws.redshift#ScheduledAction$Schedule */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_schedule(var_3);
            }
            ,
            s if s.matches("IamRole") /* IamRole com.amazonaws.redshift#ScheduledAction$IamRole */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_iam_role(var_4);
            }
            ,
            s if s.matches("ScheduledActionDescription") /* ScheduledActionDescription com.amazonaws.redshift#ScheduledAction$ScheduledActionDescription */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_scheduled_action_description(var_5);
            }
            ,
            s if s.matches("State") /* State com.amazonaws.redshift#ScheduledAction$State */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::model::ScheduledActionState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::ScheduledActionState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_6);
            }
            ,
            s if s.matches("NextInvocations") /* NextInvocations com.amazonaws.redshift#ScheduledAction$NextInvocations */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_scheduled_action_time_list::de_scheduled_action_time_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_next_invocations(var_7);
            }
            ,
            s if s.matches("StartTime") /* StartTime com.amazonaws.redshift#ScheduledAction$StartTime */ =>  {
                let var_8 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.redshift#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_start_time(var_8);
            }
            ,
            s if s.matches("EndTime") /* EndTime com.amazonaws.redshift#ScheduledAction$EndTime */ =>  {
                let var_9 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.redshift#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_end_time(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

