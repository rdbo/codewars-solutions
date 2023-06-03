# Challenge: https://www.codewars.com/kata/551f23362ff852e2ab000037/train/python

def longest_slide_down(pyramid):
    sums = pyramid[-1]
    for i in reversed(range(len(pyramid) - 1)):
        for j in range(len(pyramid[i])):
            sums[j] = max(sums[j], sums[j + 1]) + pyramid[i][j]
    return sums[0]
