use poise::{serenity_prelude::FullEvent, FrameworkContext};

use crate::{mommy, Data, Error};

mod autoreply;
mod board;

pub async fn handle(
    ctx: &poise::serenity_prelude::Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        //FullEvent::CommandPermissionsUpdate { permission } => todo!(),
        //FullEvent::AutoModRuleCreate { rule } => todo!(),
        //FullEvent::AutoModRuleUpdate { rule } => todo!(),
        //FullEvent::AutoModRuleDelete { rule } => todo!(),
        //FullEvent::AutoModActionExecution { execution } => todo!(),
        //FullEvent::CacheReady { guilds } => todo!(),
        //FullEvent::ShardsReady { total_shards } => todo!(),
        //FullEvent::ChannelCreate { channel } => todo!(),
        //FullEvent::CategoryCreate { category } => todo!(),
        //FullEvent::CategoryDelete { category } => todo!(),
        //FullEvent::ChannelDelete { channel, messages } => todo!(),
        //FullEvent::ChannelPinsUpdate { pin } => todo!(),
        //FullEvent::ChannelUpdate { old, new } => todo!(),
        //FullEvent::GuildAuditLogEntryCreate { entry, guild_id } => todo!(),
        //FullEvent::GuildBanAddition {
        //    guild_id,
        //    banned_user,
        //} => todo!(),
        //FullEvent::GuildBanRemoval {
        //    guild_id,
        //    unbanned_user,
        //} => todo!(),
        //FullEvent::GuildCreate { guild, is_new } => todo!(),
        //FullEvent::GuildDelete { incomplete, full } => todo!(),
        //FullEvent::GuildEmojisUpdate {
        //    guild_id,
        //    current_state,
        //} => todo!(),
        //FullEvent::GuildIntegrationsUpdate { guild_id } => todo!(),
        //FullEvent::GuildMemberAddition { new_member } => todo!(),
        //FullEvent::GuildMemberRemoval {
        //    guild_id,
        //    user,
        //    member_data_if_available,
        //} => todo!(),
        //FullEvent::GuildMemberUpdate {
        //    old_if_available,
        //    new,
        //    event,
        //} => todo!(),
        //FullEvent::GuildMembersChunk { chunk } => todo!(),
        //FullEvent::GuildRoleCreate { new } => todo!(),
        //FullEvent::GuildRoleDelete {
        //    guild_id,
        //    removed_role_id,
        //    removed_role_data_if_available,
        //} => todo!(),
        //FullEvent::GuildRoleUpdate {
        //    old_data_if_available,
        //    new,
        //} => todo!(),
        //FullEvent::GuildStickersUpdate {
        //    guild_id,
        //    current_state,
        //} => todo!(),
        //FullEvent::GuildUpdate {
        //    old_data_if_available,
        //    new_data,
        //} => todo!(),
        //FullEvent::InviteCreate { data } => todo!(),
        //FullEvent::InviteDelete { data } => todo!(),
        FullEvent::Message { new_message } => {
            mommy::message(ctx, data, new_message).await?;
            autoreply::handle(ctx, data, new_message).await?
        }
        //FullEvent::MessageDelete {
        //    channel_id,
        //    deleted_message_id,
        //    guild_id,
        //} => todo!(),
        //FullEvent::MessageDeleteBulk {
        //    channel_id,
        //    multiple_deleted_messages_ids,
        //    guild_id,
        //} => todo!(),
        //FullEvent::MessageUpdate {
        //    old_if_available,
        //    new,
        //    event,
        //} => todo!(),
        FullEvent::ReactionAdd { add_reaction } => board::handle(ctx, data, add_reaction).await?,
        FullEvent::ReactionRemove { removed_reaction } => {
            board::handle(ctx, data, removed_reaction).await?
        }
        //FullEvent::ReactionRemoveAll {
        //    channel_id,
        //    removed_from_message_id,
        //} => todo!(),
        //FullEvent::ReactionRemoveEmoji { removed_reactions } => todo!(),
        //FullEvent::PresenceReplace { presences } => todo!(),
        //FullEvent::PresenceUpdate { new_data } => todo!(),
        //FullEvent::Ready { data_about_bot } => todo!(),
        //FullEvent::Resume { event } => todo!(),
        //FullEvent::ShardStageUpdate { event } => todo!(),
        //FullEvent::TypingStart { event } => todo!(),
        //FullEvent::UserUpdate { old_data, new } => todo!(),
        //FullEvent::VoiceServerUpdate { event } => todo!(),
        //FullEvent::VoiceStateUpdate { old, new } => todo!(),
        //FullEvent::VoiceChannelStatusUpdate {
        //    old,
        //    status,
        //    id,
        //    guild_id,
        //} => todo!(),
        //FullEvent::WebhookUpdate {
        //    guild_id,
        //    belongs_to_channel_id,
        //} => todo!(),
        //FullEvent::InteractionCreate { interaction } => todo!(),
        //FullEvent::IntegrationCreate { integration } => todo!(),
        //FullEvent::IntegrationUpdate { integration } => todo!(),
        //FullEvent::IntegrationDelete {
        //    integration_id,
        //    guild_id,
        //    application_id,
        //} => todo!(),
        //FullEvent::StageInstanceCreate { stage_instance } => todo!(),
        //FullEvent::StageInstanceUpdate { stage_instance } => todo!(),
        //FullEvent::StageInstanceDelete { stage_instance } => todo!(),
        //FullEvent::ThreadCreate { thread } => todo!(),
        //FullEvent::ThreadUpdate { old, new } => todo!(),
        //FullEvent::ThreadDelete {
        //    thread,
        //    full_thread_data,
        //} => todo!(),
        //FullEvent::ThreadListSync { thread_list_sync } => todo!(),
        //FullEvent::ThreadMemberUpdate { thread_member } => todo!(),
        //FullEvent::ThreadMembersUpdate {
        //    thread_members_update,
        //} => todo!(),
        //FullEvent::GuildScheduledEventCreate { event } => todo!(),
        //FullEvent::GuildScheduledEventUpdate { event } => todo!(),
        //FullEvent::GuildScheduledEventDelete { event } => todo!(),
        //FullEvent::GuildScheduledEventUserAdd { subscribed } => todo!(),
        //FullEvent::GuildScheduledEventUserRemove { unsubscribed } => todo!(),
        _ => (),
    };

    Ok(())
}
