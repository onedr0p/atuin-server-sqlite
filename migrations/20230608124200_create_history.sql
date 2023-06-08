create table history (
	id bigserial primary key,
	client_id text not null unique, -- the client-generated ID
	user_id bigserial not null,     -- allow multiple users
	hostname text not null,         -- a unique identifier from the client (can be hashed, random, whatever)
	timestamp timestamp not null,   -- one of the few non-encrypted metadatas

	data text not null,    -- store the actual history data, encrypted. I don't wanna know!

	created_at timestamp not null default current_timestamp,
	deleted_at timestamp
);

-- queries will all be selecting the ids of history for a user, that has been deleted
create unique index history_deleted_index on history(client_id, user_id, deleted_at);
