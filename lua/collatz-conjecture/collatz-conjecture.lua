return function(t)
  local function collatz(n)
    if (n <= 0) then
      error("Only positive numbers are allowed")
    elseif (n == 1) then
      return 0
    end
    return 1 + collatz(n % 2 == 0 and n / 2 or 3 * n + 1)
  end
  return collatz(t)
end
