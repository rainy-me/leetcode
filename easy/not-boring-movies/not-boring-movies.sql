# write your mysql query statement below
select
    *
from
    cinema c
where
    mod(c.id, 2) and lower(c.description) not like "%boring%"
order by c.rating desc;