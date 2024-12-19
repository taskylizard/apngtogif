from apngtogif import convert

with open("stuff/a.png", "rb") as f:
    apng_bytes = f.read()

output = convert(apng_bytes, 1)

with open("stuff/output.gif", "wb") as f:
    f.write(output)
    print("Done!")
