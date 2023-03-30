// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_stack_instance(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::StackInstance, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::StackInstance::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("StackSetId") /* StackSetId com.amazonaws.cloudformation#StackInstance$StackSetId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_stack_set_id(var_1);
            }
            ,
            s if s.matches("Region") /* Region com.amazonaws.cloudformation#StackInstance$Region */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_region(var_2);
            }
            ,
            s if s.matches("Account") /* Account com.amazonaws.cloudformation#StackInstance$Account */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_account(var_3);
            }
            ,
            s if s.matches("StackId") /* StackId com.amazonaws.cloudformation#StackInstance$StackId */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_stack_id(var_4);
            }
            ,
            s if s.matches("ParameterOverrides") /* ParameterOverrides com.amazonaws.cloudformation#StackInstance$ParameterOverrides */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_parameters::de_parameters(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_parameter_overrides(var_5);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.cloudformation#StackInstance$Status */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::model::StackInstanceStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::StackInstanceStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_6);
            }
            ,
            s if s.matches("StackInstanceStatus") /* StackInstanceStatus com.amazonaws.cloudformation#StackInstance$StackInstanceStatus */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_stack_instance_comprehensive_status::de_stack_instance_comprehensive_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_stack_instance_status(var_7);
            }
            ,
            s if s.matches("StatusReason") /* StatusReason com.amazonaws.cloudformation#StackInstance$StatusReason */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status_reason(var_8);
            }
            ,
            s if s.matches("OrganizationalUnitId") /* OrganizationalUnitId com.amazonaws.cloudformation#StackInstance$OrganizationalUnitId */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_organizational_unit_id(var_9);
            }
            ,
            s if s.matches("DriftStatus") /* DriftStatus com.amazonaws.cloudformation#StackInstance$DriftStatus */ =>  {
                let var_10 =
                    Some(
                        Result::<crate::model::StackDriftStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::StackDriftStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_drift_status(var_10);
            }
            ,
            s if s.matches("LastDriftCheckTimestamp") /* LastDriftCheckTimestamp com.amazonaws.cloudformation#StackInstance$LastDriftCheckTimestamp */ =>  {
                let var_11 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudformation#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_last_drift_check_timestamp(var_11);
            }
            ,
            s if s.matches("LastOperationId") /* LastOperationId com.amazonaws.cloudformation#StackInstance$LastOperationId */ =>  {
                let var_12 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_last_operation_id(var_12);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

