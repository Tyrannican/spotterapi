CREATE TABLE thing_type(
    id INTEGER NOT NULL,
    PRIMARY KEY(id),
    type VARCHAR(64) NOT NULL
);

CREATE TABLE things(
    id UUID NOT NULL,
    PRIMARY KEY(id),
    thing_type INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    lat FLOAT NOT NULL,
    lng FLOAT NOT NULL,
    count INTEGER NOT NULL,
    description TEXT,
    image BYTEA,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(thing_type) REFERENCES thing_type(id)
);


INSERT INTO thing_type (id, type) VALUES
    (0, 'thing'),
    (1, 'mammal'),
    (2, 'plant'),
    (3, 'fungi'),
    (4, 'insect'),
    (5, 'bird'),
    (6, 'reptile'),
    (7, 'marine')
