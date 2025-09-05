return function(n)
  local drops = (n % 3 == 0 and 'Pling' or '') ..
      (n % 5 == 0 and 'Plang' or '') .. (n % 7 == 0 and 'Plong' or '')
  return #drops > 0 and drops or tostring(n)
end
