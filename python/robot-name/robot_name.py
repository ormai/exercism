from random import randint, seed
from time import time


class Robot:
    def __init__(self):
        self.name: str = self.__random_name()

    def reset(self):
        """Sets the name to a new randomly generated name."""
        self.name = self.__random_name()

    @staticmethod
    def __random_name() -> str:
        """Generats a totally random name"""

        seed(time())

        def random_letter():
            """Generates a random uppercase letter"""
            return chr(randint(ord("A"), ord("Z")))

        return f"{random_letter()}{random_letter()}{randint(100, 999)}"
