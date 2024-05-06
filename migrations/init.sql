--DROP TABLE IF EXISTS "todos";

CREATE TABLE "todos" (
    todo TEXT NOT NULL PRIMARY KEY
);

INSERT INTO "todos" (todo) VALUES ('Hausaufgaben');