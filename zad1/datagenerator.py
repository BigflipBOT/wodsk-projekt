import random

def generate_pairs(num_pairs, increase_range, second_range):
    pairs = []
    first_number = 0
    
    for _ in range(num_pairs):
        second_number = random.randint(second_range[0], second_range[1])
        pairs.append(f"{first_number},{second_number}")
        first_number += random.randint(increase_range[0], increase_range[1])
    
    return "; ".join(pairs)

def save_to_file(data, file_name):
    try:
        with open(file_name, 'w') as file:
            file.write(data)
        print(f"Data successfully saved to {file_name}")
    except Exception as e:
        print(f"An error occurred while saving to the file: {e}")

def main():
    try:
        num_pairs = int(input("Enter the number of pairs to generate: "))
        increase_min = int(input("Enter the minimum value for increase range of the first number: "))
        increase_max = int(input("Enter the maximum value for increase range of the first number: "))
        second_min = int(input("Enter the minimum value for the second number range: "))
        second_max = int(input("Enter the maximum value for the second number range: "))
        file_name = input("Enter the name of the file to save the data: ")

        if increase_min > increase_max or second_min > second_max:
            print("Invalid ranges provided. Please ensure minimum values are less than or equal to maximum values.")
            return

        increase_range = (increase_min, increase_max)
        second_range = (second_min, second_max)

        result = generate_pairs(num_pairs, increase_range, second_range)
        print("Generated pairs")
        # print(result)
        
        save_to_file(result, file_name)
    
    except ValueError:
        print("Please enter valid integers for all inputs.")

if __name__ == "__main__":
    main()

