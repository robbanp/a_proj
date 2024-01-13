-- Add migration script here
alter table merchants
  alter id add generated always as identity;