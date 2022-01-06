// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_table_output_next_token(
    input: &crate::output::DescribeTableOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_get_statement_result_output_next_token(
    input: &crate::output::GetStatementResultOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_databases_output_next_token(
    input: &crate::output::ListDatabasesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_schemas_output_next_token(
    input: &crate::output::ListSchemasOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_statements_output_next_token(
    input: &crate::output::ListStatementsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_tables_output_next_token(
    input: &crate::output::ListTablesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_table_output_column_list(
    input: crate::output::DescribeTableOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ColumnMetadata>> {
    let input = match input.column_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_get_statement_result_output_records(
    input: crate::output::GetStatementResultOutput,
) -> std::option::Option<std::vec::Vec<std::vec::Vec<crate::model::Field>>> {
    let input = match input.records {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_databases_output_databases(
    input: crate::output::ListDatabasesOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.databases {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_schemas_output_schemas(
    input: crate::output::ListSchemasOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.schemas {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_statements_output_statements(
    input: crate::output::ListStatementsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::StatementData>> {
    let input = match input.statements {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_tables_output_tables(
    input: crate::output::ListTablesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::TableMember>> {
    let input = match input.tables {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
