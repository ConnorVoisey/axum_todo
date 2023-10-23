CREATE TABLE "todo"
(
    id    uuid primary key default uuid_generate_v1mc(),
    title       text collate "case_insensitive" not null,
    description text                            not null,
    completed   bool default false              not null
);
