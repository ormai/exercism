def convert(number):
    drops = ""
    if number % 3 == 0:
        drops += "Pling"
    if number % 5 == 0:
        drops += "Plang"
    if number % 7 == 0:
        drops += "Plong"
    if drops:
        return drops
    return str(number)
