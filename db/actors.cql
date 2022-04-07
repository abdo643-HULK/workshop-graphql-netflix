CREATE KEYSPACE actors WITH REPLICATION = { 
	'class': 'SimpleStrategy',
	'replication_factor': 1 
};

use actors;

CREATE TABLE series (
    actorId UUID,
	seriesId UUID,

	name varchar(50),
	timestamp bigint,

	PRIMARY KEY (actorId, seriesId)
);

CREATE TABLE movies (
    actorId UUID,
	movieId UUID,

    name varchar(50),
	timestamp bigint,

	PRIMARY KEY (actorId, movieId)
);
