CREATE TABLE thing_type(
    id INTEGER NOT NULL,
    PRIMARY KEY(id),
    type VARCHAR(255) NOT NULL
);

CREATE TABLE thing(
    id UUID NOT NULL,
    PRIMARY KEY(id),
    user_id UUID,
    type INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    lat FLOAT NOT NULL,
    lng FLOAT NOT NULL,
    count INTEGER NOT NULL,
    description TEXT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(type) REFERENCES thing_type(id)
);


INSERT INTO thing_type (id, type) VALUES
    (0, 'Object'),
    (1, 'Mammal'),
    (2, 'Plant'),
    (3, 'Fungi'),
    (4, 'Insect'),
    (5, 'Bird'),
    (6, 'Reptile'),
    (7, 'Marine')
