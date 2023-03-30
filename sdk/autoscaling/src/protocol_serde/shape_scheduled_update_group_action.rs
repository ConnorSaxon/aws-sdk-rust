// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_scheduled_update_group_action(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ScheduledUpdateGroupAction, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ScheduledUpdateGroupAction::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AutoScalingGroupName") /* AutoScalingGroupName com.amazonaws.autoscaling#ScheduledUpdateGroupAction$AutoScalingGroupName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_auto_scaling_group_name(var_1);
            }
            ,
            s if s.matches("ScheduledActionName") /* ScheduledActionName com.amazonaws.autoscaling#ScheduledUpdateGroupAction$ScheduledActionName */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_scheduled_action_name(var_2);
            }
            ,
            s if s.matches("ScheduledActionARN") /* ScheduledActionARN com.amazonaws.autoscaling#ScheduledUpdateGroupAction$ScheduledActionARN */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_scheduled_action_arn(var_3);
            }
            ,
            s if s.matches("Time") /* Time com.amazonaws.autoscaling#ScheduledUpdateGroupAction$Time */ =>  {
                let var_4 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.autoscaling#TimestampType`)"))
                        ?
                    )
                ;
                builder = builder.set_time(var_4);
            }
            ,
            s if s.matches("StartTime") /* StartTime com.amazonaws.autoscaling#ScheduledUpdateGroupAction$StartTime */ =>  {
                let var_5 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.autoscaling#TimestampType`)"))
                        ?
                    )
                ;
                builder = builder.set_start_time(var_5);
            }
            ,
            s if s.matches("EndTime") /* EndTime com.amazonaws.autoscaling#ScheduledUpdateGroupAction$EndTime */ =>  {
                let var_6 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.autoscaling#TimestampType`)"))
                        ?
                    )
                ;
                builder = builder.set_end_time(var_6);
            }
            ,
            s if s.matches("Recurrence") /* Recurrence com.amazonaws.autoscaling#ScheduledUpdateGroupAction$Recurrence */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_recurrence(var_7);
            }
            ,
            s if s.matches("MinSize") /* MinSize com.amazonaws.autoscaling#ScheduledUpdateGroupAction$MinSize */ =>  {
                let var_8 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#AutoScalingGroupMinSize`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_min_size(var_8);
            }
            ,
            s if s.matches("MaxSize") /* MaxSize com.amazonaws.autoscaling#ScheduledUpdateGroupAction$MaxSize */ =>  {
                let var_9 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#AutoScalingGroupMaxSize`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_size(var_9);
            }
            ,
            s if s.matches("DesiredCapacity") /* DesiredCapacity com.amazonaws.autoscaling#ScheduledUpdateGroupAction$DesiredCapacity */ =>  {
                let var_10 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#AutoScalingGroupDesiredCapacity`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_desired_capacity(var_10);
            }
            ,
            s if s.matches("TimeZone") /* TimeZone com.amazonaws.autoscaling#ScheduledUpdateGroupAction$TimeZone */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_time_zone(var_11);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

