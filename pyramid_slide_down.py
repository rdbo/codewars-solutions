def climb(pyramid, i, j):
    if i == 0:
        return []
    
    higher_nums = {}
    if j < len(pyramid[i - 1]):
        higher_nums[pyramid[i - 1][j]] = [i - 1, j]
    
    if j - 1 >= 0:
        higher_nums[pyramid[i - 1][j - 1]] = [i - 1, j - 1]
    
    return higher_nums

def climb_top(pyramid, combs, prev, i, j):
    if i == 0 and j == 0:
        combs.append(prev)
        return combs
    
    higher_nums = climb(pyramid, i, j)
    
    for n in higher_nums:
        p = prev + [n]
        pos = higher_nums[n]
        combs = climb_top(pyramid, combs, p, pos[0], pos[1])
    return combs
    
def longest_slide_down(pyramid):
    sums = []
    for i in range(len(pyramid)):
        paths = climb_top(pyramid, [], [pyramid[-1][i]], len(pyramid) - 1, i)
        sums.append(max([sum(p) for p in paths]))

    return max(sums)
