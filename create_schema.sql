create table
	if not exists tenant (
		id text primary key,
		name text not null unique,
		image text not null,
		description text not null
	);

create table
	if not exists characters (
		id text primary key,
		tenant_id text references tenant (id),
		name text not null,
		description text,
		image text not null,
		guide_available text
	);

create table
	if not exists attributes (
		id text primary key,
		tenant_id text references tenant (id),
		name text not null unique,
		parent_attribute_id text,
		preference integer,
		foreign key (parent_attribute_id) references attributes (id)
	);

create table
	if not exists attribute_values (
		id text primary key,
		attribute_id text not null,
		tenant_id text not null,
		value text not null,
		description text,
		foreign key (attribute_id) references attributes (id),
		foreign key (tenant_id) references tenant (id)
	);

create table
	if not exists character_attributes (
		tenant_id text references tenant (id),
		character_id text references characters (id),
		attribute_id text references attributes (id),
		attribute_value_id text references attribute_values (id),
		preference integer
	);

create table
	if not exists guides (
		id text primary key,
		tenant_id text references tenant (id),
		name text not null,
		description text not null
	);

create table
	if not exists character_guides (
		id text primary key,
		tenant_id text references tenant (id),
		character_id text references characters (id),
		preference integer,
		guide_id text references guides (id)
	);

create table
	if not exists guide_attributes (
		id primary key,
		guide_id text references guides (id),
		tenant_id text references tenant (id),
		attribute_name_id text references attributes (id),
		attribute_value_id text references attribute_values (id),
		preference integer
	);

create table
	if not exists moves (
		id text primary key,
		tenant_id text,
		name text not null unique,
		parent_move_id text,
		preference integer,
		foreign key (parent_move_id) references moves (id)
	);

create table
	if not exists moves_values (
		id text primary key,
		move_id text not null references moves (id),
		tenant_id text not null references tenant (id),
		value text not null,
		description text
	);

create table
	if not exists character_moves (
		tenant_id text references tenant (id),
		character_id text references characters (id),
		move_id text references moves (id),
		move_value_id text references moves_values (id),
		preference integer
	);

create table
	if not exists guide_moves (
		id primary key,
		guide_id text references guides (id),
		tenant_id text references tenant (id),
		move_name_id text references moves (id),
		move_value_id text references moves_values (id),
		preference integer
	);