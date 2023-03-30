// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateMatchmakingConfiguration`](crate::client::fluent_builders::UpdateMatchmakingConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_name): <p>A unique identifier for the matchmaking configuration to update. You can use either the configuration name or ARN value. </p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_description): <p>A description for the matchmaking configuration.</p>
    ///   - [`game_session_queue_arns(Vec<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::game_session_queue_arns) / [`set_game_session_queue_arns(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_game_session_queue_arns): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to a GameLift game session queue resource and uniquely identifies it. ARNs are unique across all Regions. Format is <code>arn:aws:gamelift:   <region>    ::gamesessionqueue/    <queue name></queue>   </region></code>. Queues can be located in any Region. Queues are used to start new GameLift-hosted game sessions for matches that are created with this matchmaking configuration. If <code>FlexMatchMode</code> is set to <code>STANDALONE</code>, do not set this parameter.</p>
    ///   - [`request_timeout_seconds(i32)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::request_timeout_seconds) / [`set_request_timeout_seconds(Option<i32>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_request_timeout_seconds): <p>The maximum duration, in seconds, that a matchmaking ticket can remain in process before timing out. Requests that fail due to timing out can be resubmitted as needed.</p>
    ///   - [`acceptance_timeout_seconds(i32)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::acceptance_timeout_seconds) / [`set_acceptance_timeout_seconds(Option<i32>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_acceptance_timeout_seconds): <p>The length of time (in seconds) to wait for players to accept a proposed match, if acceptance is required.</p>
    ///   - [`acceptance_required(bool)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::acceptance_required) / [`set_acceptance_required(Option<bool>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_acceptance_required): <p>A flag that indicates whether a match that was created with this configuration must be accepted by the matched players. To require acceptance, set to TRUE. With this option enabled, matchmaking tickets use the status <code>REQUIRES_ACCEPTANCE</code> to indicate when a completed potential match is waiting for player acceptance. </p>
    ///   - [`rule_set_name(impl Into<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::rule_set_name) / [`set_rule_set_name(Option<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_rule_set_name): <p>A unique identifier for the matchmaking rule set to use with this configuration. You can use either the rule set name or ARN value. A matchmaking configuration can only use rule sets that are defined in the same Region.</p>
    ///   - [`notification_target(impl Into<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::notification_target) / [`set_notification_target(Option<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_notification_target): <p>An SNS topic ARN that is set up to receive matchmaking notifications. See <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-notification.html"> Setting up notifications for matchmaking</a> for more information.</p>
    ///   - [`additional_player_count(i32)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::additional_player_count) / [`set_additional_player_count(Option<i32>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_additional_player_count): <p>The number of player slots in a match to keep open for future players. For example, if the configuration's rule set specifies a match for a single 12-person team, and the additional player count is set to 2, only 10 players are selected for the match. This parameter is not used if <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    ///   - [`custom_event_data(impl Into<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::custom_event_data) / [`set_custom_event_data(Option<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_custom_event_data): <p>Information to add to all events related to the matchmaking configuration. </p>
    ///   - [`game_properties(Vec<GameProperty>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::game_properties) / [`set_game_properties(Option<Vec<GameProperty>>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_game_properties): <p>A set of custom properties for a game session, formatted as key:value pairs. These properties are passed to a game server process with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>). This information is added to the new <code>GameSession</code> object that is created for a successful match. This parameter is not used if <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    ///   - [`game_session_data(impl Into<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::game_session_data) / [`set_game_session_data(Option<String>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_game_session_data): <p>A set of custom game session properties, formatted as a single string value. This data is passed to a game server process with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>). This information is added to the game session that is created for a successful match. This parameter is not used if <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    ///   - [`backfill_mode(BackfillMode)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::backfill_mode) / [`set_backfill_mode(Option<BackfillMode>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_backfill_mode): <p>The method that is used to backfill game sessions created with this matchmaking configuration. Specify MANUAL when your game manages backfill requests manually or does not use the match backfill feature. Specify AUTOMATIC to have GameLift create a match backfill request whenever a game session has one or more open slots. Learn more about manual and automatic backfill in <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-backfill.html">Backfill Existing Games with FlexMatch</a>. Automatic backfill is not available when <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    ///   - [`flex_match_mode(FlexMatchMode)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::flex_match_mode) / [`set_flex_match_mode(Option<FlexMatchMode>)`](crate::client::fluent_builders::UpdateMatchmakingConfiguration::set_flex_match_mode): <p>Indicates whether this matchmaking configuration is being used with GameLift hosting or as a standalone matchmaking solution. </p>  <ul>   <li> <p> <b>STANDALONE</b> - FlexMatch forms matches and returns match information, including players and team assignments, in a <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-events.html#match-events-matchmakingsucceeded"> MatchmakingSucceeded</a> event.</p> </li>   <li> <p> <b>WITH_QUEUE</b> - FlexMatch forms matches and uses the specified GameLift queue to start a game session for the match. </p> </li>  </ul>
                            /// - On success, responds with [`UpdateMatchmakingConfigurationOutput`](crate::output::UpdateMatchmakingConfigurationOutput) with field(s):
    ///   - [`configuration(Option<MatchmakingConfiguration>)`](crate::output::UpdateMatchmakingConfigurationOutput::configuration): <p>The updated matchmaking configuration.</p>
                            /// - On failure, responds with [`SdkError<UpdateMatchmakingConfigurationError>`](crate::error::UpdateMatchmakingConfigurationError)
    pub fn update_matchmaking_configuration(&self) -> crate::client::fluent_builders::UpdateMatchmakingConfiguration {
                                crate::client::fluent_builders::UpdateMatchmakingConfiguration::new(self.handle.clone())
                            }
}

