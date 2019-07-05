drop table topics;
CREATE TABLE topics
(
    topic_id INTEGER PRIMARY KEY,
    name     TEXT NOT NULL UNIQUE
);

insert into topics (name) values ('C#');

select * from topics;