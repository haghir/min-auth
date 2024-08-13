CREATE OR REPLACE VIEW credentials AS
SELECT
    CONCAT(a.username, '@', b.service_name) AS `id`
    a.salt                                  AS `salt`
    a.pwhash                                AS `pwhash`
FROM
    users a
    INNER JOIN service_users b ON b.user_id = a.id
WHERE
    a.username IS NOT NULL
;
