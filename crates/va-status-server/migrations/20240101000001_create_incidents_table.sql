-- Add migration script here
CREATE TABLE incidents (
    id UUID PRIMARY KEY NOT NULL,
    service_id UUID NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    status service_status NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index to speed up incident lookups by service
CREATE INDEX incidents_service_id_idx ON incidents (service_id);