import psycopg2

# Connect to the database
conn = psycopg2.connect("postgres://postgres:postgres@localhost:5432/postgres")

# Open a cursor to perform database operations
cur = conn.cursor()


cur.execute("SELECT h_id FROM helmets")
helmet_ids = cur.fetchall()
print(len(helmet_ids))


# Retrieve all the IDs from the riders table
cur.execute("SELECT r_id FROM riders")
rider_ids = cur.fetchall()

# Retrieve all the IDs from the events table
cur.execute("SELECT e_id FROM events")
event_ids = cur.fetchall()

# Combine the rider IDs and event IDs into a single list
all_ids = [id[0] for id in rider_ids] + [id[0] for id in event_ids]

# Write the IDs to a file
with open('ids.txt', 'w') as f:
    for id in all_ids:
        f.write(str(id) + '\n')

# Close the cursor and the database connection
cur.close()
conn.close()
