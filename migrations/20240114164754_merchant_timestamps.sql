-- Add migration script here

alter table merchants
  alter created_at TYPE TIMESTAMPTZ;
alter table merchants
  alter updated_at TYPE TIMESTAMPTZ;
