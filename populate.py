#BIKES
from faker import Faker
from tqdm import tqdm
import random
from datetime import datetime

fake = Faker()

f = open("up1.sql", "w")

# Generate 1_000_000 rider ids
print("Generating 1_000_000 bike ids")
bike_ids = [fake.uuid4() for _ in range(1_000_000)]
print("Generating 1_000_000 helmet ids")
helmet_ids = [fake.uuid4() for _ in range(1_000_000)]
print("Generating 1_000_000 rider ids")
rider_ids = [fake.uuid4() for _ in range(1_000_000)]
print("Generating 1_000_000 event ids")
event_ids = [fake.uuid4() for _ in range(1_000_000)]
time = "2023-04-23 00:00:00"

used_bike_ids = []
# Generate 1 million bike records
print("Generating 1_000_000 bikes")
for i in tqdm(range(10000)):
    brand = fake.company()
    model = fake.word()
    wheelsize = fake.random_element(elements=(26.0, 27.5, 28.0, 29.0))
    size = fake.random_element(elements=('S', 'M', 'L', 'XL'))
    price = fake.pyfloat(left_digits=4, right_digits=2, positive=True, min_value=100.0, max_value=5000.0)
    created_at = datetime.now()
    updated_at = datetime.now()
    
    temp = ""
    temp += "INSERT INTO bikes (b_id, brand, model, wheelsize, size, price, created_at, updated_at) VALUES  \n "
    # Generate bike ID and check for duplicates
    
    for j in range(100):
        used_bike_ids.append(bike_ids[i * 100 + j])    
        temp += (
            "('"
            + bike_ids[i * 100 + j]
            + "', '"
            + brand
            + str(j)
            + "', '"
            + model
            + str(j)
            + "', '"
            + str(fake.random_element(elements=(26.0, 27.5, 28.0, 29.0)))
            + "', '"
            + fake.random_element(elements=('S', 'M', 'L', 'XL'))
            + "', '"
            + str(price)
            + "', '"
            + str(created_at)
            + "', '"
            + str(updated_at)
            + "'),\n"
        )

    # Remove the last 2 characters
    temp = temp[:-2]
    temp += ";\n"
    f.write(temp)


f.close()
f = open("up2.sql", "w")
    
used_helmet_ids = []
# Generate 1 million helmet records
print("Generating 1_000_000 helmets")
for i in tqdm(range(10000)):
    brand = fake.company()
    model = fake.word()
    h_type = fake.random_element(elements=('full face', 'enduro', 'trial', 'cross-country'))
    size = fake.random_element(elements=('S', 'M', 'L', 'XL'))
    price = fake.pyfloat(left_digits=4, right_digits=2, positive=True, min_value=50.0, max_value=1000.0)
    created_at = datetime.now()
    updated_at = datetime.now()

    temp = ""
    temp += "INSERT INTO helmets (h_id, brand, model, h_type, size, price, created_at, updated_at) VALUES  \n "
    for j in range(100):
        used_helmet_ids.append(helmet_ids[i * 100 + j])
        temp += (
            "('"
            + helmet_ids[i * 100 + j]
            + "', '"
            + brand
            + str(j)
            + "', '"
            + model
            + str(j)
            + "', '"
            + fake.random_element(elements=('full face', 'enduro', 'trial', 'cross-country'))
            + "', '"
            + fake.random_element(elements=('S', 'M', 'L', 'XL'))
            + "', '"
            + str(price)
            + "', '"
            + str(created_at)
            + "', '"
            + str(updated_at)
            + "'),\n"
        )

    # Remove the last 2 characters
    temp = temp[:-2]
    temp += ";\n"
    f.write(temp)


f.close()
f = open("up3.sql", "w")

pre_used_rider_ids = []
used_rider_gear_ids = []
#RIDERS
print("Generating 1_000_000 riders")
for i in tqdm(range(10000)):
    helmet_id = random.choice(used_helmet_ids)
    bike_id = random.choice(used_bike_ids)
    used_rider_gear_ids.append([bike_id, helmet_id])
    r_name = fake.name()
    height = fake.pyfloat(left_digits=1, right_digits=2, positive=True, min_value=1.0, max_value=2.5)
    r_weight = fake.pyfloat(left_digits=2, right_digits=1, positive=True, min_value=30.0, max_value=99.0)
    specialization = fake.random_element(elements=('Freeride', 'Road', 'Trail', 'Enduro', 'Downhill', 'Cross-country', 'Dirt', 'BMX', 'Other'))
    email = fake.email()
    phone = fake.numerify(text='##########')
    created_at = datetime.now()
    updated_at = datetime.now()

    temp = ""
    temp += "INSERT INTO riders (r_id, helmet_id, bike_id, r_name, height, r_weight, specialization, email, phone, created_at, updated_at) VALUES  \n "
    for j in range(100):
        pre_used_rider_ids.append(rider_ids[i * 100 + j])
        temp += (
            "('"
            + rider_ids[i * 100 + j]
            + "', '"
            + helmet_id
            + "', '"
            + bike_id
            + "', '"
            + r_name
            + str(j)
            + "', '"
            + str(height)
            + "', '"
            + str(r_weight)
            + "', '"
            + specialization
            + "', '"
            + email
            + "', '"
            + str(phone)
            + "', '"
            + str(created_at)
            + "', '"
            + str(updated_at)
            + "'),\n"
        )

    # Remove the last 2 characters
    temp = temp[:-2]
    temp += ";\n"
    f.write(temp)


f.close()
f = open("up4.sql", "w")

pre_used_event_ids = []
#EVENTS
print("Generating 1_000_000 events")
for i in tqdm(range(10000)):
    e_name = fake.company()
    e_date = fake.date_time_between(start_date='-1y', end_date='now')
    specialization = fake.random_element(elements=('Freeride', 'Road', 'Trail', 'Enduro', 'Downhill', 'Cross-country', 'Dirt', 'BMX', 'Other'))
    created_at = datetime.now()
    updated_at = datetime.now()

    temp = ""
    temp += "INSERT INTO events (e_id, e_name, e_date, specialization, created_at, updated_at) VALUES  \n "

    for j in range(100):
        pre_used_event_ids.append(event_ids[i * 100 + j])
        temp += (
            "('"
            + event_ids[i * 100 + j]
            + "', '"
            + e_name
            + str(j)
            + "', '"
            + str(e_date)
            + "', '"
            + specialization
            + "', '"
            + str(created_at)
            + "', '"
            + str(updated_at)
            + "'),\n"
        )

    # Remove the last 2 characters
    temp = temp[:-2]
    temp += ";\n"
    f.write(temp)


f.close()
f = open("up5.sql", "w")


def get_unused_id(id_list, used_id_list):
    id = random.choice(id_list)
    while id in used_id_list:
        id = random.choice(id_list)
    
    used_id_list.append(id)
    return id

used_event_ids = []
eventrider_ids = []
#EVENTRIDER
print("Generating 10_000_000 eventrider")
for i in tqdm(range(10000)):
    e_id = get_unused_id(pre_used_event_ids, used_event_ids)

    er_type = fake.random_element(elements=('Participant', 'Organizer', 'Spectator'))
    er_specialization = fake.random_element(elements=('Freeride', 'Road', 'Trail', 'Enduro', 'Downhill', 'Cross-country', 'Dirt', 'BMX', 'Other'))
    created_at = datetime.now()
    updated_at = datetime.now()
    used_rider_ids = []
    temp = ""
    temp += "INSERT INTO eventrider (e_id, r_id, er_type, er_specialization, created_at, updated_at) VALUES  \n "

    for j in range(100):
        
        r_id=get_unused_id(pre_used_rider_ids, used_rider_ids)
        eventrider_ids.append([e_id,r_id])
        temp += (
            "('"
            + e_id
            + "', '"
            + r_id
            + "', '"
            + fake.random_element(elements=('Participant', 'Organizer', 'Spectator'))
            + "', '"
            + fake.random_element(elements=('Freeride', 'Road', 'Trail', 'Enduro', 'Downhill', 'Cross-country', 'Dirt', 'BMX', 'Other'))
            + "', '"
            + str(created_at)
            + "', '"
            + str(updated_at)
            + "'),\n"
        )

    # Remove the last 2 characters
    temp = temp[:-2]
    temp += ";\n"
    f.write(temp)


f.close()


#check if all eventrider tuple ids are unique
print("Checking if all eventrider tuple ids are unique")
if len(eventrider_ids) == len(set(map(tuple, eventrider_ids))):
    print("All eventrider tuple ids are unique")

#check if all used_rider_ids are from helmet_ids and bike_ids
print("Checking if all used_rider_ids are from helmet_ids and bike_ids")
bike_column = []
helmet_column = []

# Loop through each row in the 2D matrix
for row in used_rider_gear_ids:
    # Append the first and second elements of each row to the respective columns
    bike_column.append(row[0])
    helmet_column.append(row[1])
if set(bike_column ).issubset(set(bike_ids)) and set(helmet_column).issubset(set(helmet_ids)):
    print("All used_rider_ids are from helmet_ids and bike_ids")
