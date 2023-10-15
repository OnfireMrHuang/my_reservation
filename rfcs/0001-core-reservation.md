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

## Reference-level explanation



## Drawbacks



## Rationale and alternatives



## Prior art



## Unresolved questions



## Future possibilities



