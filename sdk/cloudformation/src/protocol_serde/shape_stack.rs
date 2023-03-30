// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_stack(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::Stack, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::Stack::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("StackId") /* StackId com.amazonaws.cloudformation#Stack$StackId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_stack_id(var_1);
            }
            ,
            s if s.matches("StackName") /* StackName com.amazonaws.cloudformation#Stack$StackName */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_stack_name(var_2);
            }
            ,
            s if s.matches("ChangeSetId") /* ChangeSetId com.amazonaws.cloudformation#Stack$ChangeSetId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_change_set_id(var_3);
            }
            ,
            s if s.matches("Description") /* Description com.amazonaws.cloudformation#Stack$Description */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_4);
            }
            ,
            s if s.matches("Parameters") /* Parameters com.amazonaws.cloudformation#Stack$Parameters */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_parameters::de_parameters(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_parameters(var_5);
            }
            ,
            s if s.matches("CreationTime") /* CreationTime com.amazonaws.cloudformation#Stack$CreationTime */ =>  {
                let var_6 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudformation#CreationTime`)"))
                        ?
                    )
                ;
                builder = builder.set_creation_time(var_6);
            }
            ,
            s if s.matches("DeletionTime") /* DeletionTime com.amazonaws.cloudformation#Stack$DeletionTime */ =>  {
                let var_7 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudformation#DeletionTime`)"))
                        ?
                    )
                ;
                builder = builder.set_deletion_time(var_7);
            }
            ,
            s if s.matches("LastUpdatedTime") /* LastUpdatedTime com.amazonaws.cloudformation#Stack$LastUpdatedTime */ =>  {
                let var_8 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudformation#LastUpdatedTime`)"))
                        ?
                    )
                ;
                builder = builder.set_last_updated_time(var_8);
            }
            ,
            s if s.matches("RollbackConfiguration") /* RollbackConfiguration com.amazonaws.cloudformation#Stack$RollbackConfiguration */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_rollback_configuration::de_rollback_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_rollback_configuration(var_9);
            }
            ,
            s if s.matches("StackStatus") /* StackStatus com.amazonaws.cloudformation#Stack$StackStatus */ =>  {
                let var_10 =
                    Some(
                        Result::<crate::model::StackStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::StackStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_stack_status(var_10);
            }
            ,
            s if s.matches("StackStatusReason") /* StackStatusReason com.amazonaws.cloudformation#Stack$StackStatusReason */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_stack_status_reason(var_11);
            }
            ,
            s if s.matches("DisableRollback") /* DisableRollback com.amazonaws.cloudformation#Stack$DisableRollback */ =>  {
                let var_12 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudformation#DisableRollback`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_disable_rollback(var_12);
            }
            ,
            s if s.matches("NotificationARNs") /* NotificationARNs com.amazonaws.cloudformation#Stack$NotificationARNs */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_notification_ar_ns::de_notification_ar_ns(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_notification_ar_ns(var_13);
            }
            ,
            s if s.matches("TimeoutInMinutes") /* TimeoutInMinutes com.amazonaws.cloudformation#Stack$TimeoutInMinutes */ =>  {
                let var_14 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudformation#TimeoutMinutes`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_timeout_in_minutes(var_14);
            }
            ,
            s if s.matches("Capabilities") /* Capabilities com.amazonaws.cloudformation#Stack$Capabilities */ =>  {
                let var_15 =
                    Some(
                        crate::protocol_serde::shape_capabilities::de_capabilities(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_capabilities(var_15);
            }
            ,
            s if s.matches("Outputs") /* Outputs com.amazonaws.cloudformation#Stack$Outputs */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_outputs::de_outputs(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_outputs(var_16);
            }
            ,
            s if s.matches("RoleARN") /* RoleARN com.amazonaws.cloudformation#Stack$RoleARN */ =>  {
                let var_17 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_role_arn(var_17);
            }
            ,
            s if s.matches("Tags") /* Tags com.amazonaws.cloudformation#Stack$Tags */ =>  {
                let var_18 =
                    Some(
                        crate::protocol_serde::shape_tags::de_tags(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_18);
            }
            ,
            s if s.matches("EnableTerminationProtection") /* EnableTerminationProtection com.amazonaws.cloudformation#Stack$EnableTerminationProtection */ =>  {
                let var_19 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudformation#EnableTerminationProtection`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_enable_termination_protection(var_19);
            }
            ,
            s if s.matches("ParentId") /* ParentId com.amazonaws.cloudformation#Stack$ParentId */ =>  {
                let var_20 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_parent_id(var_20);
            }
            ,
            s if s.matches("RootId") /* RootId com.amazonaws.cloudformation#Stack$RootId */ =>  {
                let var_21 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_root_id(var_21);
            }
            ,
            s if s.matches("DriftInformation") /* DriftInformation com.amazonaws.cloudformation#Stack$DriftInformation */ =>  {
                let var_22 =
                    Some(
                        crate::protocol_serde::shape_stack_drift_information::de_stack_drift_information(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_drift_information(var_22);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

