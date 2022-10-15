-- Add migration script here
create table ImagesData (
    id bigint not null auto_increment,
    content longblob not null,
    mime_type varchar(255) not null,
    primary key (id)
);

create table Users (
    id varchar(25) not null,
    avatar_id bigint,
    username varchar(255) not null,
    password varchar(96) not null,
    last_token varchar(255),
    foreign key (avatar_id) references ImagesData(id),
    primary key (id)
);
create unique index username_ix on Users(username);
create index last_token_ix on Users(last_token);

create table Images (
    id varchar(25) not null,
    user_id varchar(25) not null,
    image_id bigint not null,
    public bool not null default false,
    anonymous bool not null default false,
    public_list bool not null default false,
    foreign key (user_id) references Users(id),
    foreign key (image_id) references ImagesData(id),
    primary key (id)
);
create index user_id_ix on Images(user_id);
create index public_ix on Images(public);