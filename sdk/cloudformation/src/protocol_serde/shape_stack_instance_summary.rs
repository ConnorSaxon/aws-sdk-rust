// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_stack_instance_summary(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::StackInstanceSummary, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::StackInstanceSummary::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("StackSetId") /* StackSetId com.amazonaws.cloudformation#StackInstanceSummary$StackSetId */ =>  {
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
            s if s.matches("Region") /* Region com.amazonaws.cloudformation#StackInstanceSummary$Region */ =>  {
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
            s if s.matches("Account") /* Account com.amazonaws.cloudformation#StackInstanceSummary$Account */ =>  {
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
            s if s.matches("StackId") /* StackId com.amazonaws.cloudformation#StackInstanceSummary$StackId */ =>  {
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
            s if s.matches("Status") /* Status com.amazonaws.cloudformation#StackInstanceSummary$Status */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::model::StackInstanceStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::StackInstanceStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_5);
            }
            ,
            s if s.matches("StatusReason") /* StatusReason com.amazonaws.cloudformation#StackInstanceSummary$StatusReason */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status_reason(var_6);
            }
            ,
            s if s.matches("StackInstanceStatus") /* StackInstanceStatus com.amazonaws.cloudformation#StackInstanceSummary$StackInstanceStatus */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_stack_instance_comprehensive_status::de_stack_instance_comprehensive_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_stack_instance_status(var_7);
            }
            ,
            s if s.matches("OrganizationalUnitId") /* OrganizationalUnitId com.amazonaws.cloudformation#StackInstanceSummary$OrganizationalUnitId */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_organizational_unit_id(var_8);
            }
            ,
            s if s.matches("DriftStatus") /* DriftStatus com.amazonaws.cloudformation#StackInstanceSummary$DriftStatus */ =>  {
                let var_9 =
                    Some(
                        Result::<crate::model::StackDriftStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::StackDriftStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_drift_status(var_9);
            }
            ,
            s if s.matches("LastDriftCheckTimestamp") /* LastDriftCheckTimestamp com.amazonaws.cloudformation#StackInstanceSummary$LastDriftCheckTimestamp */ =>  {
                let var_10 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudformation#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_last_drift_check_timestamp(var_10);
            }
            ,
            s if s.matches("LastOperationId") /* LastOperationId com.amazonaws.cloudformation#StackInstanceSummary$LastOperationId */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_last_operation_id(var_11);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

