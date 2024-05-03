CREATE TABLE `credentials` (
    `id`     VARCHAR(191) NOT NULL PRIMARY KEY
,   `salt`   VARCHAR(191) NOT NULL
,   `pwhash` CHAR(64) NOT NULL
);

CREATE TABLE `groups` (
    `id`          CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `parent_id`   CHAR(24)          CHARSET ascii COLLATE ascii_bin -- root group's parent id is null.
,   `name`        VARCHAR(191)      NOT NULL
,   `created_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP
,   `updated_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
,   FOREIGN KEY (`parent_id`) REFERENCES `groups` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);

CREATE TABLE `users` (
    `id`          CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `group_id`    CHAR(24)          CHARSET ascii COLLATE ascii_bin
,   `username`    VARCHAR(191)      UNIQUE   -- sets null if the user get to be removed.
,   `email`       VARCHAR(191)      NOT NULL
,   `first_name`  VARCHAR(191)      NOT NULL
,   `surname`     VARCHAR(191)
,   `pubkey_fpr`  CHAR(40)          NOT NULL -- the latest public key fingerprint.
,   `pubkey`      BLOB              NOT NULL -- the latest public key.
,   `su`          BOOLEAN           NOT NULL FALSE
,   `created_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP
,   `updated_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
,   `deleted_at`  DATETIME
,   FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);

CREATE TABLE `permissions` (
    `id`          CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `user_id`     CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL
,   `group_id`    CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL
,   `read`        BOOLEAN           NOT NULL DEFAULT FALSE
,   `adduser`     BOOLEAN           NOT NULL DEFAULT FALSE
,   `moduser`     BOOLEAN           NOT NULL DEFAULT FALSE
,   `deluser`     BOOLEAN           NOT NULL DEFAULT FALSE
,   `addgroup`    BOOLEAN           NOT NULL DEFAULT FALSE
,   `modgroup`    BOOLEAN           NOT NULL DEFAULT FALSE
,   `delgroup`    BOOLEAN           NOT NULL DEFAULT FALSE
,   `grant`       BOOLEAN           NOT NULL DEFAULT FALSE
,   `created_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP
,   `updated_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
,   FOREIGN KEY (`user_id`) REFERENCES `groups` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT
,   FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT
,   UNIQUE (`user_id`, `group_id`)
);

CREATE TABLE `requests` (
    `id`          CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `issuer_id`   CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL
,   `type`        VARCHAR(191)      NOT NULL
,   `status`      TINYINT           NOT NULL -- 0: new, 1: in progress, 2: succeeded, <0: error code
,   `proc_id`     CHAR(24)          CHARSET ascii COLLATE ascii_bin
,   `description` TEXT
,   `created_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP
,   `updated_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
,   INDEX (`proc_id`)
);

CREATE TABLE `request_tickets` (
    `id`          CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `rnd`         INTEGER UNSIGNED  NOT NULL
,   `created_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP
,   `updated_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
,   FOREIGN KEY (`id`) REFERENCES `requests` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);

CREATE TABLE `new_user_requests` (
    `id`          CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `group_id`    CHAR(24)          CHARSET ascii COLLATE ascii_bin
,   `username`    VARCHAR(191)      UNIQUE   -- sets null if the user get to be removed.
,   `email`       VARCHAR(191)      NOT NULL
,   `first_name`  VARCHAR(191)      NOT NULL
,   `surname`     VARCHAR(191)
,   `pubkey`      BLOB              NOT NULL -- the latest public key.
,   `created_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP
,   `updated_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
,   FOREIGN KEY (`id`) REFERENCES `requests` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);

CREATE TABLE `changing_pubkey_requests` (
    `id`          CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `user_id`     CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL
,   `pubkey`      BLOB              NOT NULL -- the latest public key.
,   `created_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP
,   `updated_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
,   FOREIGN KEY (`id`) REFERENCES `requests` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);

CREATE TABLE `password_reset_requests` (
    `id`          CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `user_id`     CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL
,   `created_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP
,   `updated_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
,   FOREIGN KEY (`id`) REFERENCES `requests` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);

CREATE TABLE `sending_email_requests` (
    `id`          CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL PRIMARY KEY
,   `user_id`     CHAR(24)          CHARSET ascii COLLATE ascii_bin NOT NULL
,   `subject`     TEXT              NOT NULL
,   `body`        LONGTEXT          NOT NULL
,   `created_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP
,   `updated_at`  DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
,   FOREIGN KEY (`id`) REFERENCES `requests` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
);
