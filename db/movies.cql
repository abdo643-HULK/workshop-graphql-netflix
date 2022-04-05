CREATE KEYSPACE movies WITH REPLICATION = { 
    'class': 'SimpleStrategy',
    'replication_factor': 2 
};

use movies;

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
    movieId UUID,

    releaseYear SMALLINT,
    duration BIGINT,
    genres SET<TEXT>,
    maturityRating TINYINT,
   
    PRIMARY KEY (movieId)
);

CREATE TABLE details (
    movieId UUID,

    synopsis TEXT,
    country SET<TEXT>,
    creatorId UUID,
    creatorName TEXT,

    PRIMARY KEY (movieId)
);

CREATE TABLE cast (
    movieId UUID,
    actorId UUID,
    name TEXT,

    PRIMARY KEY (movieId, actorId)
);

CREATE TABLE ratings (
    movieId UUID,
    userId UUID,
    rating FLOAT,
	timestamp BIGINT,

    PRIMARY KEY (movieId, userId)
);

CREATE TABLE related (
    movieId BIGINT,
    
    title TEXT,
    synopsis TEXT,
    releaseYear SMALLINT,
    maturityRating TINYINT,

    PRIMARY KEY (movieId, releaseYear)
);