# Write out the code for `sum()` function by recursion;
def sum(list):
    if list:
        return list[0]+sum(list[1:])
    else:
        return 0


print("The result is %d",sum([1,2,3,4,5,6]))
