-- Add migration script here
PRAGMA foreign_keys = OFF;

CREATE TABLE IF NOT EXISTS
            `label_infos_tmp` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `catalog_number` TEXT,
                `label` TEXT REFERENCES `labels_gid_redirect` (`gid`),
                `release` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
INSERT INTO `label_infos_tmp` (`id`, `catalog_number`, `label`, `release`) SELECT `id`, `catalog_number`, `label`, `release` FROM `label_infos`;
DROP TABLE `label_infos`;
ALTER TABLE `label_infos_tmp` RENAME TO `label_infos`;
CREATE INDEX `idx_label_infos_2` ON `label_infos` (`catalog_number`, `release`);
CREATE INDEX `idx_label_infos` ON `label_infos` (`label`, `catalog_number`);

PRAGMA foreign_keys = ON;