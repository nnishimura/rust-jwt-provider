SET client_encoding = 'UTF8';
CREATE SCHEMA IF NOT EXISTS extension;
CREATE EXTENSION IF NOT EXISTS "pgcrypto" WITH SCHEMA extension;

CREATE OR REPLACE FUNCTION update_modified_datetime()
    RETURNS TRIGGER
AS
$$
BEGIN
    NEW.modified_datetime = now();
    RETURN NEW;
END;
$$
    LANGUAGE 'plpgsql';