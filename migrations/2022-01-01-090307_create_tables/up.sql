CREATE TABLE clients (
  id uuid PRIMARY KEY DEFAULT extension.gen_random_uuid(),
  client_name character varying(100) UNIQUE NOT NULL, ---- user friendly unique client ID
  audience jsonb NOT NULL, --- array of expected audience. [applicationClientId1, applicationClientId2]
  jwks jsonb,
--   jwks_url TEXT,

  created_datetime  timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
  modified_datetime timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE access_tokens (
  id uuid PRIMARY KEY DEFAULT extension.gen_random_uuid(),
  client_id uuid REFERENCES clients (id) NOT NULL,
  active boolean NOT NULL,
  attributes jsonb,
  created_datetime  timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
  modified_datetime timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE refresh_tokens (
  id uuid PRIMARY KEY DEFAULT extension.gen_random_uuid(),
  client_id uuid REFERENCES clients (id) NOT NULL,
  active boolean NOT NULL,
  created_datetime  timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
  modified_datetime timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER client_modified_datetime
    BEFORE UPDATE
    ON clients
    FOR EACH ROW
EXECUTE PROCEDURE update_modified_datetime();

CREATE TRIGGER refresh_tokens_modified_datetime
    BEFORE UPDATE
    ON refresh_tokens
    FOR EACH ROW
EXECUTE PROCEDURE update_modified_datetime();

CREATE TRIGGER access_tokens_modified_datetime
    BEFORE UPDATE
    ON access_tokens
    FOR EACH ROW
EXECUTE PROCEDURE update_modified_datetime();