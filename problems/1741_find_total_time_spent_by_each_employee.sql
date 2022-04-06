SELECT event_day as day, emp_id, SUM(out_time - in_time) as total_time
FROM employees
GROUP BY event_day, emp_id;
