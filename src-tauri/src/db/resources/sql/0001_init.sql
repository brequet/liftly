CREATE TABLE
    IF NOT EXISTS exercises (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        predefined_id TEXT,
        name TEXT NOT NULL,
        title TEXT NOT NULL,
        description TEXT,
        type TEXT,
        equipment TEXT,
        primary_muscles TEXT,
        secondary_muscles TEXT,
        steps TEXT,
        tips TEXT
    );

CREATE VIRTUAL TABLE exercises_fts USING fts5 (
    title,
    content = 'exercises', -- The original table to draw content from
    content_rowid = 'id' -- The column that maps to the original table's rowid
);

-- Trigger to update the FTS table when a new exercise is inserted
CREATE TRIGGER exercises_ai AFTER INSERT ON exercises BEGIN
INSERT INTO
    exercises_fts (rowid, title)
VALUES
    (new.id, new.title);

END;

-- Trigger to update the FTS table when an exercise is deleted
CREATE TRIGGER exercises_ad AFTER DELETE ON exercises BEGIN
INSERT INTO
    exercises_fts (exercises_fts, rowid, title)
VALUES
    ('delete', old.id, old.title);

END;

-- Trigger to update the FTS table when an exercise is updated
CREATE TRIGGER exercises_au AFTER
UPDATE ON exercises BEGIN
INSERT INTO
    exercises_fts (exercises_fts, rowid, title)
VALUES
    ('delete', old.id, old.title);

INSERT INTO
    exercises_fts (rowid, title)
VALUES
    (new.id, new.title);

END;

CREATE TABLE
    IF NOT EXISTS workouts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        start_datetime TEXT NOT NULL,
        end_datetime TEXT,
        status TEXT NOT NULL CHECK (
            status IN ('in_progress', 'completed', 'cancelled')
        ) DEFAULT 'in_progress',
        notes TEXT
    );

CREATE TABLE
    IF NOT EXISTS workout_sets (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        workout_id INTEGER NOT NULL,
        exercise_id INTEGER NOT NULL,
        reps INTEGER NOT NULL,
        weight REAL NOT NULL,
        created_at TEXT NOT NULL DEFAULT (strftime ('%Y-%m-%d %H:%M:%f', 'now')),
        FOREIGN KEY (workout_id) REFERENCES workouts (id) ON DELETE CASCADE,
        FOREIGN KEY (exercise_id) REFERENCES exercises (id)
    );