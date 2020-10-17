# Write your MySQL query statement below
SELECT r.Email
FROM (
    SELECT Email, COUNT(*) AS cases 
    FROM Person 
    GROUP BY Email
) r
WHERE r.cases > 1;
