INSERT INTO users ( id, username, email, password_hash )
VALUES(
  42,
  'jdong',
  'jdong@hotmail.com',
  'not-even-valid-bro'
);

INSERT INTO users ( id, username, email, password_hash )
VALUES(
  7,
  'ltrain',
  'ltrain@aol.com',
  'not-even-valid-bro'
);

INSERT INTO events (
  id, creator_id, scheduled_date, duration_in_mins, name, location, details
)
VALUES(
  73,
  42,
  CURRENT_TIMESTAMP AT TIME ZONE 'UTC' + INTERVAL '1 day',
  90,
  'Jdongs Big Day',
  'Jdongs House',
  'This one is for Jdong!'
);

INSERT INTO event_rsvps (
  event_id, user_id, extra_attendee_count, status
)
VALUES(
  73, 7, 2, 'maybe'
);
