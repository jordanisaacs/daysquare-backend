-- Add migration script here

/* Table to hold data primitives as enum.
* e.g. string, int, time, uuid, etc.
* Each data_type is a data_primitive
*/
create table daysquare.data_primitive(
    id uuid primary key,
    primitive text not null,

    unique(primitive)
);

/* Table to represent a data type as enum.
* e.g. spotify artist id, google api token, const, etc.
* All queries, paths, and headers are a data_type
* Optional regex check
*/
create table daysquare.data_type(
    id uuid primary key,
    data_primitive_id uuid not null references daysquare.data_primitive(id),
    label text not null,

    unique(data_primitive_id, label)
);
