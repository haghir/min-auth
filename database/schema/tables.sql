CREATE TABLE `services` (
    `name`          VARCHAR(191)        NOT NULL PRIMARY KEY
,   `label`         VARCHAR(191)        NOT NULL
);

CREATE TABLE `users` (
    `id`            CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `username`      VARCHAR(191)        UNIQUE   -- sets null if the user get to be removed.
,   `email`         VARCHAR(191)        NOT NULL
,   `salt`          VARCHAR(191)        NOT NULL
,   `pwhash`        CHAR(64)            NOT NULL
,   `pubkey`        CHAR(40)            NOT NULL -- the latest public key fingerprint.
,   `su`            BOOLEAN             NOT NULL FALSE
,   `created_by`    CHAR(24)            NOT NULL
,   `created_at`    DATETIME            NOT NULL DEFAULT CURRENT_TIMESTAMP
,   `updated_by`    CHAR(24)            NOT NULL
,   `updated_at`    DATETIME            NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
,   `deleted_by`    CHAR(24)
,   `deleted_at`    DATETIME
);

CREATE TABLE `service_users` (
    `id`            CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `service_name`  VARCHAR(191)        NOT NULL
,   `user_id`       CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL
,   `created_by`    CHAR(24)            NOT NULL
,   `created_at`    DATETIME            NOT NULL DEFAULT CURRENT_TIMESTAMP
,   FOREIGN KEY (`service_name`) REFERENCES `services` (`name`) ON DELETE CASCADE ON UPDATE RESTRICT
,   FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT
,   UNIQUE (`service_id`, `user_id`)
);

CREATE TABLE `requests` (
    `id`            CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `issuer_id`     CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL
,   `type`          VARCHAR(191)        NOT NULL
,   `status`        TINYINT             NOT NULL -- 0: new, 1: in progress, 2: succeeded, <0: error code
,   `proc_id`       CHAR(24)            CHARSET ascii COLLATE ascii_bin
,   `description`   TEXT
,   `created_by`    CHAR(24)            NOT NULL
,   `created_at`    DATETIME            NOT NULL DEFAULT CURRENT_TIMESTAMP
,   `updated_by`    CHAR(24)            NOT NULL
,   `updated_at`    DATETIME            NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
,   INDEX (`proc_id`)
);

CREATE TABLE `request_tickets` (
    `id`            CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `rnd`           INTEGER UNSIGNED    NOT NULL
,   `created_by`    CHAR(24)            NOT NULL
,   `created_at`    DATETIME            NOT NULL DEFAULT CURRENT_TIMESTAMP
,   FOREIGN KEY (`id`) REFERENCES `requests` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);

CREATE TABLE `new_user_requests` (
    `id`            CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `username`      VARCHAR(191)        UNIQUE   -- sets null if the user get to be removed.
,   `email`         VARCHAR(191)        NOT NULL
,   `pubkey`        BLOB                NOT NULL -- the latest public key.
,   `created_by`    CHAR(24)            NOT NULL
,   `created_at`    DATETIME            NOT NULL DEFAULT CURRENT_TIMESTAMP
,   FOREIGN KEY (`id`) REFERENCES `requests` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);

CREATE TABLE `changing_pubkey_requests` (
    `id`            CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `user_id`       CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL
,   `pubkey`        BLOB                NOT NULL -- the latest public key.
,   `created_by`    CHAR(24)            NOT NULL
,   `created_at`    DATETIME            NOT NULL DEFAULT CURRENT_TIMESTAMP
,   FOREIGN KEY (`id`) REFERENCES `requests` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);

CREATE TABLE `password_reset_requests` (
    `id`            CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `user_id`       CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL
,   `created_by`    CHAR(24)            NOT NULL
,   `created_at`    DATETIME            NOT NULL DEFAULT CURRENT_TIMESTAMP
,   FOREIGN KEY (`id`) REFERENCES `requests` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);

CREATE TABLE `sending_email_requests` (
    `id`            CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `user_id`       CHAR(24)            CHARSET ascii COLLATE ascii_bin NOT NULL
,   `subject`       TEXT                NOT NULL
,   `body`          LONGTEXT            NOT NULL
,   `created_by`    CHAR(24)            NOT NULL
,   `created_at`    DATETIME            NOT NULL DEFAULT CURRENT_TIMESTAMP
,   FOREIGN KEY (`id`) REFERENCES `requests` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);
