# My Reservation Service

- Feature Name: my-reservation
- Start Date: 2023-10-15

## Summary

my reservation service that solves the problem of reserving a resource for a period of time. 

## Motivation

We need a common solution for various reservation requirements: 1) calendar booking; 2) hotel/room booking; 3) meeting room booking; 4) parking lot booking; 5) etc. Repeatedly building features for these requirements is a waste of time and resources. We should have a common solution that can be used by all teams.

## Guide-level explanation

Basic architecture:

![base-architecture](./images/arch1.jpg)

Service interface

we whould use grpc as a service interface.below is the proto definition:

```proto3

enum ReservationStatus {
	UNKNOWN = 0;
	PENDING = 1;
	CONFIRMED = 2;
	BLOCKED = 3;
}

message Reservation {
	string id = 1;
	string user_id=2;
 	ReservationStatus status =3;
	string resource_id=4;
	google.protobuf.Timestamp start=5;
	google.protobuf.Timestamp end=6;
	string note = 7;
}

message ReserveRequest {
	Reservation reservation = 1; 
}

message ReserveResponse {
	Reservation reservation = 1; 
}

message ConfirmRequest {
	string id = 1;
}

message ConfirmResponse {
	Reservation reservation = 1; 
}

message UpdateRequest {
	string id =1;
	ReservationStatus status =2;
	string note = 3;
}

message UpdateResponse {
	Reservation reservation = 1; 
}

message CancelRequest {
	string id = 1;
}

message CancelResponse {
	Reservation reservation = 1; 
}

message GetRequest {
	string id = 1;
}

message GetResponse {
	Reservation reservation = 1; 
}

message QueryRequest {
	string user_id = 1;
	string resource_id = 2;
	ReservationStatus status = 3;
	google.protobuf.Timestamp start = 4;
	google.protobuf.Timestamp end = 5;
}

service ReservationService {
	rpc reserve(ReserveRequest) returns (ReserveResponse);
	rpc confirm(ConfirmRequest) returns (ConfirmResponse);
 	rpc update(UpdateRequest) returns (UpdateResponse);
	rpc cancel(CancelRequest) returns (CancelResponse);
	rpc get(GetRequest) returns (GetResponse);
	rpc Query(QueryRequest) returns (stream Reservation);
}

```

## Database schema

we use postgres as the database. below is the schema:

```sql
-- 创建一个schema, schema在关系型数据库中往往表示的是数据库。
create schema revp;
-- 创建预定状态的枚举类型
CREATE TYPE rsvp.reservation_status AS ENUM ('unknown', 'pending', 'confirmed', 'blocked');
-- 创建预定更新枚举类型
CREATE TYPE rsvp.reservation_update_type AS ENUM ('unknown', 'create', 'update', 'delete');

-- 创建预定表
create table revp.reservations (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	user_id VARCHAR(64) NOT NULL,
	status rsvp.reservation_status NOT NULL DEFAULT 'pending',
	resource_id VARCHAR(64) NOT NULL,
	timespan TSTZRANGE NOT NULL,
	note TEXT,

 	CONSTRAINT reservations_pkey PRIMARY KEY (id),
	CONSTRAINT reservatinos_conflict EXCLUDE USING gist (resource_id WITH =, timespan WITH &&)
);

-- 创建索引
CREATE INDEX reservations_resource_id_idx ON revp.reservations (resource_id);
CREATE INDEX reservations_user_id_idx ON revp.reservations (user_id);

-- 创建一个查询函数
-- if user_id is null, find all reservatinos within during for the resource
-- if resource_id is null, find all reservations within during for the user
-- if both are null, find all reservations within during
-- if both set, find all reservations within during for the user and resource
CREATE OR REPLACE FUNCTION rsvp.query(
	uid text,
	rid text,
	during TSTZRANGE,
) RETURNS TABLE rsvp.reservations as $$ $$ LANGUAGE plpgsql;

-- reservation change queue
CREATE TABLE rsvp.reservation_changes (
	id SERIAL NOT NULL,
	reservation_id uuid NOT NULL,
	op rsvp.reservation_update_type NOT NULL,
);

-- trigger for add/update/delete a reservation
CREATE OR REPLACE FUNCTION rsvp.reservations_trigger() RETURNS TRIGGER AS $$
BEGIN
	IF TG_OP = 'INSERT' THEN
		-- update reservation_changes
  		INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'create');
	ELSIF TG_OP = 'UPDATE' THEN
  		-- if status changed, update reservation_changes
     	 IF OLD.status <> NEW.status THEN
            INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'update');
        END IF;
	ELESIF TG_OP = 'DELETE' THEN
   		-- update reservation_changes
        INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (OLD.id, 'delete');
	END IF;
  	-- notify a channel called reservation_update
    NOTIFY reservation_update;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器
CREATE TRIGGER reservations_trigger
    AFTER INSERT OR UPDATE OR DELETE ON rsvp.reservations
    FOR EACH ROW EXECUTE PROCEDURE rsvp.reservations_trigger();
```

## Reference-level explanation


## Drawbacks


## Rationale and alternatives


## Prior art


## Unresolved questions


## Future possibilities
