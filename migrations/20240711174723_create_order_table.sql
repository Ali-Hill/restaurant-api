-- Add migration script here
-- Create orders Table
CREATE TABLE orders(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   table_no integer NOT NULL,
   item TEXT NOT NULL,
   quantity integer NOT NULL,
   preparation_time integer NOT NULL,
   placed_at timestamptz NOT NULL
);
