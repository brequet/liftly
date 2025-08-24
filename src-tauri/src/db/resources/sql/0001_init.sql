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
        exercise_id TEXT NOT NULL,
        reps INTEGER NOT NULL,
        weight REAL NOT NULL,
        created_at TEXT NOT NULL DEFAULT (strftime ('%Y-%m-%d %H:%M:%f', 'now')),
        FOREIGN KEY (workout_id) REFERENCES workouts (id) ON DELETE CASCADE,
        FOREIGN KEY (exercise_id) REFERENCES exercises (id)
    );