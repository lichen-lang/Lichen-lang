def combinations(elements: list[str], r: int) -> list[list[str]]:
    # 組み合わせの要素数が0なら、空リストを返す
    if r == 0:
        return [[]]
    # リストが空で、要素数が0より大きい場合は空リストを返す
    if len(elements) < r:
        return []

    # 最初の要素を含む組み合わせを生成
    with_first = combinations(elements[1:], r - 1)
    with_first = [[elements[0]] + comb for comb in with_first]

    # 最初の要素を含まない組み合わせを生成
    without_first = combinations(elements[1:], r)

    # 両方を結合して返す
    return with_first + without_first


# 使用例
elements = ["Hello", "World", "Tom"]
r = 3
combs = combinations(elements, r)
for comb in combs:
    print(comb)


# if __name__ == "__main__":
#     for i in itertools.combinations(["hello", "world", "Tom"], 2):
#         print(i)
