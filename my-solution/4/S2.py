def count(list, i):
    if list:
        i += 1
        return count(list[1:], i)
    else:
        return i


print("The result is %d", count([1, 2, 3, 4, 5, 6], 0))
