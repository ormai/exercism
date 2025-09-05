return function(array, target)
  local lo = 1
  local hi = #array + 1
  while lo < hi do
    local mid = math.floor((lo + hi) / 2)
    if array[mid] > target then
      hi = mid
    elseif array[mid] < target then
      lo = mid + 1
    else
      return mid
    end
  end
  return -1
end
