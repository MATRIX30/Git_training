def print_alphabet():
    for i in range(97, 123):
        print(f"{chr(i)}", end="")
    print()  # newline


# Code inside this block ONLY runs if my_tools.py is run directly
if __name__ == "__main__":
    print("Executing directly!")
    print_alphabet()