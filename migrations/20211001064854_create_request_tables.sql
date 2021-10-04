-- Add migration script here

/* Table that represents a request
* Request paths, headers, and queries are stored in many to many tables
* Also points to a response schema. { ... }
*/
create table daysquare.request(
    id uuid primary key,
    api_id uuid not null references daysquare.api(id),
    response_schema_id uuid not null references daysquare.response_schema(id),
    description text not null
);


/* Table to represent path data
* sequence is unique and the order of the path e.g.:
* path_seq1/path_seq2/path_seq3...
*/
create table daysquare.path_data(
    id uuid primary key,
    request_id uuid not null references daysquare.request(id),
    data_type_id uuid not null references daysquare.data_type(id),
    sequence smallint not null,
    name text not null,

    unique(request_id, sequence)
);

/* Table to represent query data
* name is unique and represents query in url e.g.:
* ?name1=xxxx&name2=xxxxx&name3=xxxx
*/
create table daysquare.query_data(
    id uuid primary key,
    request_id uuid not null references daysquare.request(id),
    data_type_id uuid not null references daysquare.data_type(id),
    name text not null,
    is_vec boolean not null,

    unique(request_id, name)
);

/* Table to represent header data
* name is unique and represents header name e.g.:
* Authorization
*/
create table daysquare.header_data(
    id uuid primary key,
    request_id uuid not null references daysquare.request(id),
    data_type_id uuid not null references daysquare.data_type(id),
    name text not null,

    unique(request_id, name)
);
