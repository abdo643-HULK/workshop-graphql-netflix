CREATE KEYSPACE IF NOT EXISTS movie WITH REPLICATION = { 
	'class': 'SimpleStrategy',
	'replication_factor': 2 
};

use movie;

CREATE TYPE IF NOT EXISTS MATURITYRATING (
    board TEXT,
    board_id INT,
    maturity_description TEXT,
    maturity_level TINYINT,
    rating_id INT,
    reasons LIST<TEXT>,
    specific_rating_reason TEXT,
    value TEXT,
);

CREATE TYPE IF NOT EXISTS METADATA (
	release_year SMALLINT,
	duration BIGINT,
	genres SET<TEXT>,
	maturity_rating frozen<MATURITYRATING>,--TINYINT,
);

CREATE TYPE IF NOT EXISTS BOXART (
	extension TEXT,
    image_key TEXT,
    is_smoky BOOLEAN,
    size TEXT,
    url TEXT,
    width SMALLINT,
    height SMALLINT,
    video_id UUID,
);

CREATE TYPE IF NOT EXISTS DELIVERY (
	is_3D BOOLEAN,
	is_51Audio BOOLEAN,
	is_dolby_atmos BOOLEAN,
	is_dolby_vision BOOLEAN,
	is_HD BOOLEAN,
	is_HDR BOOLEAN,
	is_UltraHD BOOLEAN,
	quality TEXT,
);

CREATE TYPE IF NOT EXISTS AVAILABILITY (
    is_playable BOOLEAN,
	availability_date TEXT,
    availability_start_time BIGINT,
);

CREATE TYPE IF NOT EXISTS HIGHTLIGHTIMAGE(
    image_key TEXT,
    url TEXT,
    width SMALLINT,
    height SMALLINT,
);

CREATE TYPE IF NOT EXISTS TRAILER (
	title TEXT,
    track_id BIGINT,
    highlight frozen<HIGHTLIGHTIMAGE>,
);

CREATE TYPE IF NOT EXISTS ACTOR (
    id UUID,
    name TEXT,
);

CREATE TYPE IF NOT EXISTS RELATEDMOVIE (
    id UUID,

    is_original BOOLEAN,
    title TEXT,
    box_art frozen<BOXART>,

    metadata frozen<METADATA>,
);

-- Columns
CREATE TABLE IF NOT EXISTS movies (
    id UUID PRIMARY KEY,

    is_original BOOLEAN,
    title TEXT,
    box_art frozen<BOXART>,
    synopsis TEXT,

    metadata frozen<METADATA>,
    available_deliveries frozen<DELIVERY>, 
 
    directors text,
    writers text,

    actors LIST<frozen<ACTOR>>,
    -- cast LIST<ACTOR>,

    trailers LIST<frozen<TRAILER>>,
    related LIST<frozen<RELATEDMOVIE>>,
);

CREATE TABLE IF NOT EXISTS movies_by_genre (
	movie_id UUID,

    genre TEXT,
    is_original BOOLEAN,
    title TEXT,
    box_art frozen<BOXART>,
    metadata frozen<METADATA>,
    
    PRIMARY KEY (genre, movie_id)
) WITH CLUSTERING ORDER BY (movie_id DESC);

CREATE TABLE IF NOT EXISTS movies_by_actor (
    movie_id UUID,

    actor_id UUID,
    is_original BOOLEAN,
    title TEXT,
    box_art frozen<BOXART>,
    metadata frozen<METADATA>,

	PRIMARY KEY (actor_id, movie_id)
) WITH CLUSTERING ORDER BY (movie_id DESC);

-- CREATE TABLE related (
-- 	movieId BIGINT,
	
-- 	title TEXT,
-- 	synopsis TEXT,
-- 	releaseYear SMALLINT,
-- 	maturityRating TINYINT,

-- 	PRIMARY KEY (movieId, releaseYear)
-- ) WITH CLUSTERING ORDER BY (releaseYear DESC);