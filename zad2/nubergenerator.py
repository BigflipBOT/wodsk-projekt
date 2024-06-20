import random

def generate_random_string(length, max_int, no_repeats):
    if no_repeats:
        random_string = []
        last_int = None
        for _ in range(length):
            rand_int = random.randint(1, max_int)
            while rand_int == last_int:
                rand_int = random.randint(1, max_int)
            random_string.append(rand_int)
            last_int = rand_int
        return ','.join(map(str, random_string))
    else:
        return ','.join(str(random.randint(1, max_int)) for _ in range(length))

def main():
    # Get user inputs
    length = int(input("Enter the length of the random string: "))
    max_int = int(input("Enter the maximum integer value: "))
    filename = input("Enter the filename to save the string: ")
    no_repeats = input("Ensure no consecutive integers are the same? (yes/no): ").strip().lower() == 'yes'

    # Generate random string
    random_string = generate_random_string(length, max_int, no_repeats)

    # Write the string to the file
    with open(filename, 'w') as file:
        file.write(random_string)

    print(f"Random string saved to {filename}")

if __name__ == "__main__":
    main()

