-- Add migration script here
CREATE TABLE IF NOT EXISTS merchants (
   id int primary key GENERATED ALWAYS AS IDENTITY,
   name text not null,
   created_at TIMESTAMPTZ DEFAULT current_timestamp,
   updated_at TIMESTAMPTZ DEFAULT current_timestamp,
   metadata jsonb,
   status text not null
);

select trigger_updated_at('merchants');
