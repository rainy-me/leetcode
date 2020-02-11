select id, 
	max(if(month = 'jan', revenue,null)) as Jan_Revenue,
	max(if(month = 'feb', revenue,null)) as Feb_Revenue,
	max(if(month = 'mar', revenue,null)) as Mar_Revenue,
	max(if(month = 'apr', revenue,null)) as Apr_Revenue,
	max(if(month = 'may', revenue,null)) as May_Revenue,
	max(if(month = 'jun', revenue,null)) as Jun_Revenue,
	max(if(month = 'jul', revenue,null)) as Jul_Revenue,
	max(if(month = 'aug', revenue,null)) as Aug_Revenue,
	max(if(month = 'sep', revenue,null)) as Sep_Revenue,
	max(if(month = 'oct', revenue,null)) as Oct_Revenue,
	max(if(month = 'nov', revenue,null)) as Nov_Revenue,
	max(if(month = 'dec', revenue,null)) as Dec_Revenue
from department
group by id
order by id