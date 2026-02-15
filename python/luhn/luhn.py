class Luhn:
    def __init__(self, card_num: str):
        self.num = card_num

    def valid(self) -> bool:
        num = list(filter(lambda c: c != " ", self.num))

        if len(num) <= 1 or any(not d.isnumeric() for d in num):
            return False

        num = list(map(int, reversed(num)))

        def double(n: int) -> int:
            dup = n + n
            return dup - 9 if dup > 9 else dup

        return sum(num[::2] + list(map(double, num[1::2]))) % 10 == 0
