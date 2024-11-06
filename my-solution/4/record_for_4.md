# Exercise

## Q&A

>4.1 Write out the code for the earlier sum function.

```python
# Write out the code for `sum()` function by recursion;
def sum(list):
    if list:
        return list[0]+sum(list[1:])
    else:
        return 0

  print("The result id %d",sum([1,2,3,4,5,6]))
```

>4.2 Write a recursive function to count the number of items in a list.

```python
def count(list, i):
    if list:
        i += 1
        return count(list[1:], i)
    else:
        return i


print("The result is %d", count([1, 2, 3, 4, 5, 6], 0))
```

>4.3 Find the maximum number in a list.

```python
def max(list):
    if list:
        sub_max = max(list[1:])
        return list[0] if list[0] > sub_max else sub_max
    else:
        return 0


print("The result is", max([1, 3, 5, 6, 7, 2]))
```

>4.4 Remember binary search from chapter 1? It’s a divide-and-conquer algorithm, too. Can you come up with the base case and recursive case for binary search?
