-- Add migration script here
PRAGMA foreign_keys = OFF;

DROP TRIGGER `trigger_after_delete_tracks`;

CREATE TABLE `release_groups` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
    `title` TEXT NOT NULL,
    `mbid` TEXT UNIQUE NOT NULL,
    `primary_type_id` TEXT,
    `first_release_date` INTEGER,
    `disambiguation` TEXT,
    `annotation` TEXT,
    -- Foreign Keys
    `artist_credit` INTEGER REFERENCES `artist_credits` (`id`),
    -- Database Utils
    `full_update_date` INTEGER CHECK(`full_update_date` > 0)
) STRICT;
CREATE TABLE `release_groups_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL,
    `new_id` TEXT REFERENCES `release_groups`(`id`) ON UPDATE CASCADE ON DELETE
    SET NULL,
        `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;


CREATE TABLE `releases_tmp` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `mbid` TEXT UNIQUE NOT NULL,
    `title` TEXT NOT NULL,
    `date` INTEGER,
    `country` TEXT,
    `quality` TEXT,
    `status` TEXT,
    `barcode` TEXT,
    `disambiguation` TEXT,
    `packaging` TEXT,
    `annotation` TEXT,
    `full_update_date` INTEGER,
    -- Foreign Keys
    `artist_credit` INTEGER REFERENCES `artist_credits` (`id`),
    `release_group` INTEGER REFERENCES `release_groups` (`id`)
) STRICT;
INSERT INTO `releases_tmp` (
        `id`,
        `mbid`,
        `title`,
        `date`,
        `country`,
        `quality`,
        `status`,
        `barcode`,
        `disambiguation`,
        `packaging`,
        `annotation`,
        `full_update_date`,
        `artist_credit`
    )
SELECT `id`,
    `mbid`,
    `title`,
    `date`,
    `country`,
    `quality`,
    `status`,
    `barcode`,
    `disambiguation`,
    `packaging`,
    `annotation`,
    `full_update_date`,
    `artist_credit`
FROM `releases`;
DROP TABLE `releases`;
ALTER TABLE `releases_tmp`
    RENAME TO `releases`;
CREATE TABLE `l_artists_release_groups` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
    `type_id` TEXT NOT NULL,
    `relation_type` TEXT NOT NULL,
    `direction` TEXT NOT NULL,
    `begin` INTEGER,
    `end` INTEGER,
    `attributes` TEXT,
    `attribute_ids` TEXT,
    `atribute_values` TEXT,
    `target_type` TEXT,
    `target_credit` TEXT,
    `source_credit` TEXT,
    -- Foreign Keys
    `entity0` INTEGER NOT NULL REFERENCES `artists` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
    `entity1` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
) STRICT;
CREATE TABLE `l_labels_release_groups` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
    `type_id` TEXT NOT NULL,
    `relation_type` TEXT NOT NULL,
    `direction` TEXT NOT NULL,
    `begin` INTEGER,
    `end` INTEGER,
    `attributes` TEXT,
    `attribute_ids` TEXT,
    `atribute_values` TEXT,
    `target_type` TEXT,
    `target_credit` TEXT,
    `source_credit` TEXT,
    -- Foreign Keys
    `entity0` INTEGER NOT NULL REFERENCES `labels` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
    `entity1` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
) STRICT;
CREATE TABLE `l_recordings_release_groups` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `recordings` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;

    CREATE TABLE `l_releases_release_groups` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;

    CREATE TABLE `l_release_groups_release_groups` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_release_groups_works` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `works` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
    CREATE TRIGGER `trigger_after_delete_releases` AFTER DELETE ON `releases` BEGIN
            -- Clean full update date
            UPDATE `release_groups` SET `full_update_date` = NULL WHERE id = OLD.`release_group`;

            -- Remove the artist credit
            DELETE FROM `artist_credits` WHERE id = OLD.artist_credit;
        END;

        CREATE TRIGGER `trigger_after_delete_release_groups` AFTER DELETE ON `release_groups` BEGIN
            -- Clean full update date
            UPDATE `releases` SET `full_update_date` = NULL WHERE `release_group` = OLD.id;

            -- Remove the artist credit
            DELETE FROM `artist_credits` WHERE id = OLD.artist_credit;
        END
;
CREATE TRIGGER `trigger_after_insert_release_groups` AFTER INSERT ON `release_groups` FOR EACH ROW BEGIN
    INSERT INTO release_groups_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;

CREATE TRIGGER `trigger_after_insert_releases`
AFTER
INSERT ON `releases` FOR EACH ROW BEGIN
INSERT INTO releases_gid_redirect
VALUES (new.mbid, new.id, 0) ON CONFLICT DO
UPDATE
SET new_id = new.id;
END;



CREATE TRIGGER `trigger_after_delete_tracks` AFTER DELETE ON `tracks` BEGIN
            -- Invalidate the recording as it doesn't have its tracks anymore
            UPDATE `recordings` SET `full_update_date` = NULL WHERE id = OLD.recording;
            UPDATE `releases` SET `full_update_date` = NULL WHERE id = (
                SELECT releases.id 
                FROM releases
                INNER JOIN medias ON releases.id = medias.`release`
                WHERE medias.id = OLD.media
            );
        END;

PRAGMA foreign_keys = ON;