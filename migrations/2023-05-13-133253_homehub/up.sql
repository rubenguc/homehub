-- Your SQL goes here
CREATE TABLE home (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    street VARCHAR(255) NOT NULL,
    number VARCHAR(255) NOT NULL,
    floor VARCHAR(255) NOT NULL,
    zipcode VARCHAR(255) NOT NULL,
    squaremeters VARCHAR(255) NOT NULL,
    number_of_bathrooms VARCHAR(255) NOT NULL,
    number_of_rooms VARCHAR(255) NOT NULL,
    home_type VARCHAR(255) NOT NULL
)