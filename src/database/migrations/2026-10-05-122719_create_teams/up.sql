-- Your SQL goes here

CREATE TABLE leagues (
    id INTEGER PRIMARY KEY,
    ext_id INTEGER NOT NULL,
    name TEXT NOT NULL
);


CREATE TABLE partitions (
    id INTEGER PRIMARY KEY,
    ext_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    parent_id INTEGER NOT NULL,
    league_id INTEGER NOT NULL,

    CONSTRAINT fk_partitions_parent
        FOREIGN KEY (parent_id) REFERENCES partitions(id),

    CONSTRAINT fk_partitions_league
        FOREIGN KEY (league_id) REFERENCES leagues(id)
);


CREATE TABLE teams (
    id INTEGER PRIMARY KEY,
    ext_id INTEGER NOT NULL,
    version INTEGER NOT NULL,
    create_time TEXT NOT NULL,
    delete_time TEXT,
    name TEXT NOT NULL,
    city TEXT NOT NULL,
    partition_id INTEGER NOT NULL,

    CONSTRAINT fk_teams_partition
        FOREIGN KEY (partition_id) REFERENCES partitions(id),

    CONSTRAINT fk_teams_version
        FOREIGN KEY (version) REFERENCES versions(version)
);


CREATE TABLE stats (
    id INTEGER PRIMARY KEY,
    team_id INTEGER NOT NULL,
    year TEXT NOT NULL,
    win INTEGER NOT NULL,
    lose INTEGER NOT NULL,
    draw INTEGER NOT NULL,

    CONSTRAINT fk_stats_team
        FOREIGN KEY (team_id) REFERENCES teams(id),

    CONSTRAINT uq_stats_team_year
        UNIQUE (team_id, year)
);


CREATE TABLE trophies (
    id INTEGER PRIMARY KEY,
    team_id INTEGER NOT NULL,
    name TEXT NOT NULL,

    CONSTRAINT fk_trophies_team
        FOREIGN KEY (team_id) REFERENCES teams(id)
);
