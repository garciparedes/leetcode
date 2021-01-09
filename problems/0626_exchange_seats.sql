SELECT
    IF(
        s.id MOD 2 = 0, 
        s.id - 1, 
        IF(s.id < (SELECT COUNT(*) FROM seat), s.id + 1, s.id)
    ) as id, 
    s.student
FROM seat AS s
ORDER BY id ASC;
