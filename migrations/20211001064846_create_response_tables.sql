-- Add migration script here

/* Contents of a response to a request
* Can hold both schemas and data types e.g.:
* {
*   name = int,
*   artist = {
*      ...
*   },
* }
*/
create table daysquare.response_schema(
    id uuid primary key,
    description text not null
);

/* Data type response. can be a vector
* name = xxxx OR name = [ xxxx xxxx xxxx ]
*/
create table daysquare.response_data(
    id uuid primary key,
    response_schema_id uuid not null references daysquare.response_schema(id),
    data_type_id uuid not null references daysquare.data_type(id),
    identifier text not null,
    is_vec boolean not null,

    unique(response_schema_id, identifier)
);

/* Response schema data type. Has a parent schema
* and a child schema (cannot be the same)
* TODO: some type of check against loops
* can be a vector of the schema e.g.:
* name = {
*   ...
* }
*/
create table daysquare.response_schema_data(
    id uuid primary key,
    parent_response_schema_id uuid not null references daysquare.response_schema(
        id
    ),
    child_response_schema_id uuid not null references daysquare.response_schema(
        id
    ),
    identifier text not null,
    is_vec boolean not null,

    unique(parent_response_schema_id, identifier)
);
