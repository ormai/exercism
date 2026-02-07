class SpaceAge:
    SECONDS_IN_A_YEAR_ON_EARTH = 60 * 60 * 24 * 365.25

    def __init__(self, seconds):
        self.seconds = seconds

    def __convert(self, factor: float = 1.0) -> float:
        return round(self.seconds / (factor * self.SECONDS_IN_A_YEAR_ON_EARTH), 2)

    def on_mercury(self):
        return self.__convert(0.2408467)

    def on_venus(self):
        return self.__convert(0.61519726)

    def on_earth(self):
        return self.__convert()

    def on_mars(self):
        return self.__convert(1.8808158)

    def on_jupiter(self):
        return self.__convert(11.862615)

    def on_saturn(self):
        return self.__convert(29.447498)

    def on_uranus(self):
        return self.__convert(84.016846)

    def on_neptune(self):
        return self.__convert(164.79132)
