def max(list):
    if list:
        sub_max = max(list[1:])
        return list[0] if list[0] > sub_max else sub_max
    else:
        return 0


print("The result is", max([1, 3, 5, 6, 7, 2]))
