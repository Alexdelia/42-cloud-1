CREATE TABLE stats (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_uuid UUID NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    correct_key INT NOT NULL CHECK (correct_key >= 0),
    wrong_key INT NOT NULL CHECK (wrong_key >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()

    CONSTRAINT end_after_start CHECK (end_time > start_time)
    CONSTRAINT positive_key_count CHECK (correct_key + wrong_key > 0)
);
CREATE INDEX idx_stats_user_uuid ON stats (user_uuid);
CREATE INDEX idx_stats_created_at ON stats (created_at);
