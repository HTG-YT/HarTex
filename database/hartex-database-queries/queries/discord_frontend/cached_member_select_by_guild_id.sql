--! cached_member_select_by_guild_id (guild_id) @ (flags, joined_at?, nick?, user_id, guild_id, roles)
SELECT
    *
FROM
    "DiscordFrontend"."Nightly"."CachedMembers"
WHERE
    "guild_id" = @guild_id;
