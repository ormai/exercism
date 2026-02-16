def proverb(*args: list[str], qualifier: None | str) -> list[str]:
    if not args:
        return []
    return [
        f"For want of a {args[i]} the {args[i + 1]} was lost."
        for i in range(len(args) - 1)
    ] + [f"And all for the want of a {qualifier + ' ' if qualifier else ''}{args[0]}."]
