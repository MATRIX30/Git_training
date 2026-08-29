import random


def random_int_gen():
    #this function randomly generates an integer
    return random.randint(-9999,9999)


number = random_int_gen()

if number > 0:
    print(f"{number} is positive",)
elif number == 0:
    print(f"{number} is zero")
else:
    print(f"{number} is negative")
    