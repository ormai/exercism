def line_up(name, number):
    ordinal = str(number) + (
        "th"
        if number % 100 in (11, 12, 13)
        else {1: "st", 2: "nd", 3: "rd"}.get(number % 10, "th")
    )
    return f"{name}, you are the {ordinal} customer we serve today. Thank you!"
