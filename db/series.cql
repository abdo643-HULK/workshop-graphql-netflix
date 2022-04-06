CREATE KEYSPACE series WITH REPLICATION = { 
	'class': 'SimpleStrategy',
	'replication_factor': 2 
};

use series;

CREATE TYPE DELIVERY (
	3D BOOLEAN,
	51Audio BOOLEAN,
	DolbyAtmos BOOLEAN,
	DolbyVision BOOLEAN,
	HD BOOLEAN,
	HDR BOOLEAN,
	UltraHD BOOLEAN,
	quality TEXT,
)

CREATE TABLE metadata (
	movieId BIGINT,

	releaseYear SMALLINT,
	delivery DELIVERY,

	PRIMARY KEY (movieId)
);

CREATE TABLE details (
	movieId BIGINT,

	synopsis TEXT,

	PRIMARY KEY (movieId)
);

CREATE TABLE cast (
	movieId BIGINT,
	actorId BIGINT,

	PRIMARY KEY (movieId)
);

CREATE TABLE season (
	seasonId TINYINT,
	
	name TEXT,
	shortName TEXT,
	length TINYINT,

	timestamp BIGINT,
	PRIMARY KEY (seasonId)
);

CREATE TABLE episode (
	seasonId TINYINT,
	episodeId TINYINT,

	title TEXT,
	synopsis TEXT,

	name TEXT,
	timestamp BIGINT,
	PRIMARY KEY (movieId)
);



