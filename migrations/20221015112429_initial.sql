-- Add migration script here
create table images (
    id bigint not null auto_increment,
    content longblob not null,
    mime_type varchar(255) not null,
    primary key (id)
);

create table users (
    id varchar(25) not null,
    username varchar(255) not null,
    password varchar(96) not null,
    avatar_id bigint,
    last_token varchar(255),
    foreign key (avatar_id) references images(id),
    primary key (id)
);
create unique index username_ix on users(username);
create index last_token_ix on users(last_token);

create table images_info (
    id varchar(25) not null,
    user_id varchar(25) not null,
    image_id bigint not null,
    public bool not null default false,
    public_list bool not null default false,
    foreign key (user_id) references users(id),
    foreign key (image_id) references images(id),
    primary key (id)
);
create index user_id_ix on images_info(user_id);
create index public_ix on images_info(public);