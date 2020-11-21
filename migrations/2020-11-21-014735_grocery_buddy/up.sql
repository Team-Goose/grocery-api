-- Your SQL goes here

create table account (
	id 				serial primary key,
	username 		varchar(255) not null,
	pass 			varchar(255) not null,
	list 			varchar(255) not null default '[]', -- these last two are just strings that contain a serialized Json version of the lists they need to hold
	friends 		varchar(255) not null default '[]', -- this is because SQL needs only one value in each column
	isAdmin			boolean not null default false
);