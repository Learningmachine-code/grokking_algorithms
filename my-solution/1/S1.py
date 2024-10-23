def binary_search(list,item):
    low=0
    high=len(list)-1
    i=0

    while low<=high:
        i+=1
        # Let the result of `x/y` be integer by `//`
        mid= (low+high)//2
        guess=list[mid]
        if guess==item:
            print("the item is %d"%i)
            return mid
        if guess>item:
            high =mid-1
        else:
            low=mid+1

    print("the item is %d"%i)
    return None

my_list=[1,3,5,7,9]

print (binary_search(my_list,3))
print (binary_search(my_list,-1))
