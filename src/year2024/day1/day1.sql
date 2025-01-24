#!/usr/bin/env sqlite3 -batch -cmd .quit -init
-- [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

.bail on
.headers off

-- load puzzle input
create table data (left integer, x, y, right integer);
.separator " "
.import "input.txt" data
create table left as select left from data order by left;
create table right as select right from data order by right;
drop table data;

-- compute part 1
select sum(abs(left-right)) from left inner join right on left.rowid==right.rowid;

-- compute part 2
create table counter as select right,count(*) as nb from right group by right;
select sum(left*nb) from left,counter where counter.right==left.left;

.quit
