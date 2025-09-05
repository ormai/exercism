pub fn binarySearch(T: type, target: T, items: []const T) ?usize {
    var left: usize = 0;
    var right = items.len;
    while (left < right) {
        const mid = (left + right) / 2;
        if (items[mid] < target) {
            left = mid + 1;
        } else if (items[mid] > target) {
            right = mid;
        } else {
            return mid;
        }
    }
    return null;
}
