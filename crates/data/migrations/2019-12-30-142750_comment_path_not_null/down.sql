ALTER TABLE comments
DROP COLUMN path; 

ALTER TABLE comments
ADD path TEXT;