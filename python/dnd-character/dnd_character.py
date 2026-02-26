from random import randint, choice


class Character:
    def __init__(self):
        self.strength = randint(3, 18)
        self.dexterity = randint(3, 18)
        self.constitution = randint(3, 18)
        self.intelligence = randint(3, 18)
        self.wisdom = randint(3, 18)
        self.charisma = randint(3, 18)
        self.hitpoints = 10 + modifier(self.constitution)

    def ability(self) -> int:
        return choice(
            [
                self.strength,
                self.dexterity,
                self.constitution,
                self.intelligence,
                self.wisdom,
                self.charisma,
            ]
        )


def modifier(value: int) -> int:
    return (value - 10) // 2
