-- Your SQL goes here

create table account (
	id 				serial primary key,
	username 	varchar(255) not null,
	pass 			varchar(255) not null,
	list 			int [] not null,
	friends 	int [] not null
);