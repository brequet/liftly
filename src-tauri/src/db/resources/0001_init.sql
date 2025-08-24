CREATE TABLE
    IF NOT EXISTS workouts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        start_datetime TEXT NOT NULL,
        end_datetime TEXT, -- Nullable until workout is finished
        status TEXT NOT NULL CHECK (
            status IN ('in_progress', 'completed', 'cancelled')
        ) DEFAULT 'in_progress',
        notes TEXT
    );

CREATE TABLE
    IF NOT EXISTS workout_sets (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        workout_id INTEGER NOT NULL,
        exercise_name TEXT NOT NULL,
        reps INTEGER NOT NULL,
        weight REAL NOT NULL,
        created_at TEXT NOT NULL DEFAULT (strftime ('%Y-%m-%d %H:%M:%f', 'now')),
        FOREIGN KEY (workout_id) REFERENCES workouts (id) ON DELETE CASCADE
    );