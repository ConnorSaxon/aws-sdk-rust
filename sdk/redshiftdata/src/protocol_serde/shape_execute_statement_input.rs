// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_execute_statement_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ExecuteStatementInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.sql {
        object.key("Sql").string(var_1.as_str());
    }
    if let Some(var_2) = &input.cluster_identifier {
        object.key("ClusterIdentifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.secret_arn {
        object.key("SecretArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.db_user {
        object.key("DbUser").string(var_4.as_str());
    }
    if let Some(var_5) = &input.database {
        object.key("Database").string(var_5.as_str());
    }
    if let Some(var_6) = &input.with_event {
        object.key("WithEvent").boolean(*var_6);
    }
    if let Some(var_7) = &input.statement_name {
        object.key("StatementName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.parameters {
        let mut array_9 = object.key("Parameters").start_array();
        for item_10 in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_sql_parameter::ser_sql_parameter(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.workgroup_name {
        object.key("WorkgroupName").string(var_12.as_str());
    }
    if let Some(var_13) = &input.client_token {
        object.key("ClientToken").string(var_13.as_str());
    }
    Ok(())
}

