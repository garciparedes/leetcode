UPDATE salary 
    SET sex = CASE 
        WHEN sex = 'f' THEN 'm' 
        ELSE 'f' 
    END;      

