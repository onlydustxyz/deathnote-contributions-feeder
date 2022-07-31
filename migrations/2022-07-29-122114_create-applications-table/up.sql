CREATE TABLE applications(
    id UUID PRIMARY KEY,
    contribution_id UUID NOT NULL REFERENCES contributions(id),
    contributor_id VARCHAR NOT NULL,
    CONSTRAINT unique_application UNIQUE(contribution_id, contributor_id)
)
