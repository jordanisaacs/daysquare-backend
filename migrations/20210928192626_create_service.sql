-- Add migration script here
create schema if not exists daysquare;

/* Table that represents an entire service e.g:
* Spotify, Google, etc.
* Can have multiple services with same name but must
* have unique url (homepage)
*/
create table daysquare.service(
    id uuid primary key,
    title text not null,
    description text not null,
    url text not null,

    unique(url)
);

/* Table that represents an API to a service.
* A service can have multiple APIs. But must have
* unique versions.
* The api_url is used as the base of requests.
*/
create table daysquare.api(
    id uuid primary key,
    service_id uuid not null references daysquare.service(id),
    url text not null,
    vers text not null,

    unique(service_id, url, vers)
);

