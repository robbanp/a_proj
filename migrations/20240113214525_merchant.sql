-- Add migration script here
CREATE TABLE IF NOT EXISTS merchants (
   id int not null PRIMARY KEY,
   name varchar(255) not null,
   created_at timestamp DEFAULT current_timestamp,
   updated_at timestamp DEFAULT current_timestamp,
   metadata jsonb,
   status varchar(255)
);

