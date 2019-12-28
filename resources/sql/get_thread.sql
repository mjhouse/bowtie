WITH RECURSIVE metas AS (
	SELECT 
		parent, 
		child, 
		depth 
	FROM comments_meta WHERE 
		parent = 34
	UNION ALL
	SELECT 
		c.parent,
		c.child,
		c.depth 
	FROM comments_meta c 
	INNER JOIN metas m 
	ON m.child = c.parent
)
SELECT 
	DISTINCT c.* 
FROM 
	comments c
INNER JOIN metas 
ON 
	c.id = metas.parent OR 
	c.id = metas.child
ORDER BY 
	c.parent  ASC,
	c.created ASC;