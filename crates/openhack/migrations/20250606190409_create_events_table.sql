-- Add migration script here
CREATE TYPE rsvp_status AS ENUM (
  'maybe',
  'yes',
  'no'
);

CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  creator_id INTEGER NOT NULL,
  scheduled_date TIMESTAMP WITH TIME ZONE NOT NULL,
  duration_in_mins INTEGER NOT NULL,
  name VARCHAR(100) NOT NULL,
  location VARCHAR(255) NOT NULL,
  details TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (creator_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE event_rsvps (
  event_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  extra_attendee_count INTEGER NOT NULL,
  status rsvp_status NOT NULL DEFAULT 'maybe',
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (event_id, user_id),
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);
