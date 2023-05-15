CREATE TABLE `credentials` (
    id     VARCHAR(191) NOT NULL PRIMARY KEY
,   salt   VARCHAR(191) NOT NULL
,   pwhash CHAR(64) NOT NULL
);
