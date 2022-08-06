def fact(n:int):
    ans = 1
    for i in range(1,n):
        ans*= i
    return ans

def digit_sum(n:int):
    return sum(map(lambda x: int(x),str(n)))

def main():
    large = fact(100)
    # print(large)
    print(digit_sum(large))

if __name__ == "__main__":
    main()




