create table actix_users (
    uuid uuid primary key,
    username varchar(30) NOT NULL,
    password varchar(150) NOT NULL
)