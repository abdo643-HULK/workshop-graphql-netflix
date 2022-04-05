CREATE KEYSPACE actors WITH REPLICATION = { 
    'class': 'SimpleStrategy',
    'replication_factor': 1 
};

use actors;

CREATE TABLE series (
    seriesId UUID,
    name varchar(50),
    timestamp bigint,

    PRIMARY KEY (seriesId)
);

CREATE TABLE movies (
    movieId UUID,

    PRIMARY KEY (movieId)
);
