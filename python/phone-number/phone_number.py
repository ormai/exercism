class PhoneNumber:
    def __init__(self, number: str):
        for c in number:
            if c.isalpha():
                raise ValueError("letters not permitted")
            if c in "!@:":
                raise ValueError("punctuations not permitted")

        number = "".join(filter(str.isdigit, number))

        if len(number) < 10:
            raise ValueError("must not be fewer than 10 digits")
        if len(number) > 11:
            raise ValueError("must not be greater than 11 digits")
        if len(number) == 11:
            if number[0] != "1":
                raise ValueError("11 digits must start with 1")
            number = number[1:]

        self.number = number
        self.area_code = number[:3]
        self.exchange_code = number[3:6]
        self.subscriber_number = number[6:]

        if self.exchange_code.startswith("0"):
            raise ValueError("exchange code cannot start with zero")
        if self.exchange_code.startswith("1"):
            raise ValueError("exchange code cannot start with one")
        if self.area_code.startswith("0"):
            raise ValueError("area code cannot start with zero")
        if self.area_code.startswith("1"):
            raise ValueError("area code cannot start with one")

    def pretty(self) -> str:
        return f"({self.area_code})-{self.exchange_code}-{self.subscriber_number}"
