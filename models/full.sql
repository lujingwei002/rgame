
CREATE DATABASE IF NOT EXISTS `gamedb` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
use `gamedb`;

CREATE TABLE `account` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `open_id` varchar(128) NOT NULL COMMENT 'open_id',
    `platform` enum('test', 'quick') DEFAULT 'test' COMMENT '平台',
    `channel` int unsigned DEFAULT 0 COMMENT '渠道',
    `created_at` datetime(3) DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime(3) DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`) USING BTREE,
    UNIQUE KEY `idx_open_id_platform_channel` (`open_id`,`platform`,`channel`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


CREATE TABLE `role` (
    `id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `open_id` varchar(128) NOT NULL COMMENT 'open_id',
    `platform` enum('test', 'quick') DEFAULT 'test' COMMENT '平台',
    `channel` int unsigned DEFAULT 0 COMMENT '渠道',
    `created_at` datetime(3) DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime(3) DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`) USING BTREE,
    UNIQUE KEY `idx_open_id_platform_channel` (`open_id`,`platform`,`channel`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
