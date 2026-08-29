# Loop through the ASCII codes for lowercase English letters (97 to 122 inclusive)

# ASCII 97 = 'a', ASCII 122 = 'z'
for i in range(97, 123):
    print("{:c}".format(i), end="")
# and print each corresponding character without space or line break