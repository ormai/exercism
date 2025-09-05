local function aliquot_sum(n)
  local sum = 0;
  for f = 1, n - 1 do
    if (n % f == 0) then
      sum = sum + f
    end
  end
  return sum;
end

local function classify(n)
  local sum = aliquot_sum(n)
  return n == sum and 'perfect' or n < sum and 'abundant' or 'deficient'
end

return { aliquot_sum = aliquot_sum, classify = classify }
