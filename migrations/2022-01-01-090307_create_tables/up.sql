--- namespace for jwt (e.g. companya-sso)
CREATE TABLE tenants (
  id uuid PRIMARY KEY DEFAULT extension.gen_random_uuid(), --- key to encode jwt. TODO: generate unique jwks per each tenant
  tenant_name character varying(100) NOT NULL, ---- user friendly unique name
  issuer character varying(100) UNIQUE NOT NULL,

  access_token_ttl integer NOT NULL,
  refresh_token_ttl integer NOT NULL,

  created_datetime  timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
  modified_datetime timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

---- app client (e.g. my.webapp.com)
CREATE TABLE clients (
  id uuid PRIMARY KEY DEFAULT extension.gen_random_uuid(), --- this will be used as audience claim as well
  tenant_id uuid REFERENCES tenants (id) NOT NULL,
  client_name character varying(100) UNIQUE NOT NULL, ---- user friendly unique client ID
  --   jwks_url TEXT, TODO

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

CREATE TRIGGER tenant_modified_datetime
    BEFORE UPDATE
    ON tenants
    FOR EACH ROW
EXECUTE PROCEDURE update_modified_datetime();

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