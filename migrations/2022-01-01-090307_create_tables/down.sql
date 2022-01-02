DROP TRIGGER tenant_modified_datetime on tenants;
DROP TRIGGER client_modified_datetime on clients;
DROP TRIGGER refresh_tokens_modified_datetime on refresh_tokens;
DROP TRIGGER access_tokens_modified_datetime on access_tokens;

DROP TABLE access_tokens;
DROP TABLE refresh_tokens;
DROP TABLE clients;
DROP TABLE tenants;
