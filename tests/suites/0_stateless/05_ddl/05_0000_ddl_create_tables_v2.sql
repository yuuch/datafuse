set enable_planner_v2 = 1;

DROP TABLE IF EXISTS t;
DROP TABLE IF EXISTS t2;
DROP TABLE IF EXISTS t3;
DROP TABLE IF EXISTS t4;
DROP TABLE IF EXISTS column_comment_test;

CREATE TABLE t(c1 int) ENGINE = Null;

-- As part of the functionality of time travel, system.tables now
-- - contains dropped-tables
-- - has an extra column `dropped_on`
-- it is NOT backward compatible.
--
-- The following case is moved to 20_005_system_tables_with_dropped
--SELECT COUNT(1) from system.tables where name = 't' and database = 'default' and dropped_on = 'NULL';

CREATE TABLE IF NOT EXISTS t(c1 int) ENGINE = Null;
CREATE TABLE t(c1 int) ENGINE = Null; -- {ErrorCode 2302}


create table t2(a int,b int) engine=Memory;
insert into t2 values(1,1),(2,2);
select a+b from t2;

create table t2(a int,b int) engine=Memory; -- {ErrorCode 2302}
create table t2(a int,b int) engine=Memory; -- {ErrorCode 2302}

create table t3(a int,b int) engine=Memory CLUSTER BY(a); -- {ErrorCode 2703}


create table t3(`a` int) ENGINE = Null;
create table t4(a int) ENGINE = Null;

DROP TABLE IF EXISTS t;
DROP TABLE IF EXISTS t2;
DROP TABLE IF EXISTS t3;
DROP TABLE IF EXISTS t4;

-- prepare test databases for testing 'create table like' and 'as select' statements.
CREATE DATABASE db1;
CREATE DATABASE db2;
CREATE TABLE db1.test1(a INT, b INT null) ENGINE=memory;
INSERT INTO db1.test1 VALUES (1, 2), (2, 3), (3, 4);

SELECT '====BEGIN TEST CREATE TABLE LIKE STATEMENT====';
-- test 'create table like' statement, expect db2.test2 has the same schema with db1.test1.
CREATE TABLE db2.test2 LIKE db1.test1 ENGINE=fuse;
INSERT INTO db2.test2 VALUES (3, 5);
SELECT a+b FROM db2.test2;
-- check the schema of db2.test2, it should be the same as db1.test1, column 'a' is not nullable.
DESCRIBE db2.test2;
SELECT '====END TEST CREATE TABLE LIKE STATEMENT====';

SELECT '====BEGIN TEST CREATE TABLE AS SELECT STATEMENT====';
--test 'create table as select' statement, expect db2.test3 has the data from db1.test1 with casting
CREATE TABLE db2.test3(a Varchar null, y Varchar null) ENGINE=fuse AS SELECT * FROM db1.test1;
DESCRIBE db2.test3;
SELECT a FROM db2.test3;
CREATE TABLE db2.test4(a Varchar null, y Varchar null) ENGINE=fuse AS SELECT b, a FROM db1.test1;
DESCRIBE db2.test4;
SELECT a FROM db2.test4;
CREATE TABLE db2.test5(a Varchar null, y Varchar null) ENGINE=fuse AS SELECT b FROM db1.test1;
SELECT a FROM db2.test5;
SELECT '====END TEST CREATE TABLE AS SELECT STATEMENT====';


SELECT '====TIMESTAMP====';
create table db2.test6(id Int8, created timestamp  DEFAULT today() + 3);


SELECT '====CREATE ALL DATA TYPE TABLE====';
create table db2.test7(tiny TINYINT, tiny_unsigned TINYINT UNSIGNED, smallint SMALLINT, smallint_unsigned SMALLINT UNSIGNED, int INT, int_unsigned INT UNSIGNED, bigint BIGINT, bigint_unsigned BIGINT UNSIGNED,float FLOAT, double DOUBLE, date DATE, datetime DATETIME, ts TIMESTAMP, str VARCHAR default '3', bool BOOLEAN, arr ARRAY, obj OBJECT, variant VARIANT);
desc db2.test7;


SELECT '====CREATE TRANSIENT TABLE====';
create transient table db2.test8(tiny TINYINT, tiny_unsigned TINYINT UNSIGNED, smallint SMALLINT, smallint_unsigned SMALLINT UNSIGNED, int INT, int_unsigned INT UNSIGNED, bigint BIGINT, bigint_unsigned BIGINT UNSIGNED,float FLOAT, double DOUBLE, date DATE, datetime DATETIME, ts TIMESTAMP, str VARCHAR default '3', bool BOOLEAN, arr ARRAY, obj OBJECT, variant VARIANT);
desc db2.test8;

-- clean up test databases
DROP DATABASE db1;
DROP DATABASE db2;

CREATE TABLE system.test; -- {ErrorCode 1006}

-- create table with column comment
-- SELECT '====CREATE TABLE WITH COLUMN COMMENT====';
-- CREATE TABLE column_comment_test (a INT COMMENT 'comment for a', b FLOAT NULL DEFAULT 0 COMMENT 'comment for b');
-- DROP TABLE column_comment_test;
