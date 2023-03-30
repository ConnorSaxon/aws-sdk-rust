// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SearchGameSessions`](crate::client::fluent_builders::SearchGameSessions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::SearchGameSessions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_id(impl Into<String>)`](crate::client::fluent_builders::SearchGameSessions::fleet_id) / [`set_fleet_id(Option<String>)`](crate::client::fluent_builders::SearchGameSessions::set_fleet_id): <p>A unique identifier for the fleet to search for active game sessions. You can use either the fleet ID or ARN value. Each request must reference either a fleet ID or alias ID, but not both.</p>
    ///   - [`alias_id(impl Into<String>)`](crate::client::fluent_builders::SearchGameSessions::alias_id) / [`set_alias_id(Option<String>)`](crate::client::fluent_builders::SearchGameSessions::set_alias_id): <p>A unique identifier for the alias associated with the fleet to search for active game sessions. You can use either the alias ID or ARN value. Each request must reference either a fleet ID or alias ID, but not both.</p>
    ///   - [`location(impl Into<String>)`](crate::client::fluent_builders::SearchGameSessions::location) / [`set_location(Option<String>)`](crate::client::fluent_builders::SearchGameSessions::set_location): <p>A fleet location to search for game sessions. You can specify a fleet's home Region or a remote location. Use the Amazon Web Services Region code format, such as <code>us-west-2</code>. </p>
    ///   - [`filter_expression(impl Into<String>)`](crate::client::fluent_builders::SearchGameSessions::filter_expression) / [`set_filter_expression(Option<String>)`](crate::client::fluent_builders::SearchGameSessions::set_filter_expression): <p>String containing the search criteria for the session search. If no filter expression is included, the request returns results for all game sessions in the fleet that are in <code>ACTIVE</code> status.</p>  <p>A filter expression can contain one or multiple conditions. Each condition consists of the following:</p>  <ul>   <li> <p> <b>Operand</b> -- Name of a game session attribute. Valid values are <code>gameSessionName</code>, <code>gameSessionId</code>, <code>gameSessionProperties</code>, <code>maximumSessions</code>, <code>creationTimeMillis</code>, <code>playerSessionCount</code>, <code>hasAvailablePlayerSessions</code>.</p> </li>   <li> <p> <b>Comparator</b> -- Valid comparators are: <code>=</code>, <code>&lt;&gt;</code>, <code>&lt;</code>, <code>&gt;</code>, <code>&lt;=</code>, <code>&gt;=</code>. </p> </li>   <li> <p> <b>Value</b> -- Value to be searched for. Values may be numbers, boolean values (true/false) or strings depending on the operand. String values are case sensitive and must be enclosed in single quotes. Special characters must be escaped. Boolean and string values can only be used with the comparators <code>=</code> and <code>&lt;&gt;</code>. For example, the following filter expression searches on <code>gameSessionName</code>: "<code>FilterExpression": "gameSessionName = 'Matt\\'s Awesome Game 1'"</code>. </p> </li>  </ul>  <p>To chain multiple conditions in a single expression, use the logical keywords <code>AND</code>, <code>OR</code>, and <code>NOT</code> and parentheses as needed. For example: <code>x AND y AND NOT z</code>, <code>NOT (x OR y)</code>.</p>  <p>Session search evaluates conditions from left to right using the following precedence rules:</p>  <ol>   <li> <p> <code>=</code>, <code>&lt;&gt;</code>, <code>&lt;</code>, <code>&gt;</code>, <code>&lt;=</code>, <code>&gt;=</code> </p> </li>   <li> <p>Parentheses</p> </li>   <li> <p>NOT</p> </li>   <li> <p>AND</p> </li>   <li> <p>OR</p> </li>  </ol>  <p>For example, this filter expression retrieves game sessions hosting at least ten players that have an open player slot: <code>"maximumSessions&gt;=10 AND hasAvailablePlayerSessions=true"</code>. </p>
    ///   - [`sort_expression(impl Into<String>)`](crate::client::fluent_builders::SearchGameSessions::sort_expression) / [`set_sort_expression(Option<String>)`](crate::client::fluent_builders::SearchGameSessions::set_sort_expression): <p>Instructions on how to sort the search results. If no sort expression is included, the request returns results in random order. A sort expression consists of the following elements:</p>  <ul>   <li> <p> <b>Operand</b> -- Name of a game session attribute. Valid values are <code>gameSessionName</code>, <code>gameSessionId</code>, <code>gameSessionProperties</code>, <code>maximumSessions</code>, <code>creationTimeMillis</code>, <code>playerSessionCount</code>, <code>hasAvailablePlayerSessions</code>.</p> </li>   <li> <p> <b>Order</b> -- Valid sort orders are <code>ASC</code> (ascending) and <code>DESC</code> (descending).</p> </li>  </ul>  <p>For example, this sort expression returns the oldest active sessions first: <code>"SortExpression": "creationTimeMillis ASC"</code>. Results with a null value for the sort operand are returned at the end of the list.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::SearchGameSessions::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::SearchGameSessions::set_limit): <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. The maximum number of results returned is 20, even if this value is not set or is set higher than 20. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::SearchGameSessions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::SearchGameSessions::set_next_token): <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
                            /// - On success, responds with [`SearchGameSessionsOutput`](crate::output::SearchGameSessionsOutput) with field(s):
    ///   - [`game_sessions(Option<Vec<GameSession>>)`](crate::output::SearchGameSessionsOutput::game_sessions): <p>A collection of objects containing game session properties for each session that matches the request.</p>
    ///   - [`next_token(Option<String>)`](crate::output::SearchGameSessionsOutput::next_token): <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
                            /// - On failure, responds with [`SdkError<SearchGameSessionsError>`](crate::error::SearchGameSessionsError)
    pub fn search_game_sessions(&self) -> crate::client::fluent_builders::SearchGameSessions {
                                crate::client::fluent_builders::SearchGameSessions::new(self.handle.clone())
                            }
}

