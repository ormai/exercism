def commands(binary_str):
    operations = []
    if binary_str[4] == "1":
        operations.append("wink")
    if binary_str[3] == "1":
        operations.append("double blink")
    if binary_str[2] == "1":
        operations.append("close your eyes")
    if binary_str[1] == "1":
        operations.append("jump")
    if binary_str[0] == "1":
        operations.reverse()
    return operations
