-- Add migration script here
CREATE TYPE service_status AS ENUM ('operational', 'degraded', 'outage');

CREATE TABLE services (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    status service_status NOT NULL DEFAULT 'operational',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);