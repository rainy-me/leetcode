# Write your MySQL query statement below
select
    c.Name as Customers
from
    Customers c
where not exists (
    select
        1
    from
        Orders o
    where o.CustomerId = c.id
)