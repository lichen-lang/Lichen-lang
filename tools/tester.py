import itertools


def insert_spaces(s, n):
    # 文字列sの長さ - 1の位置にスペースを挿入できる
    positions = range(len(s) + 1)

    # スペースを挿入するn個の位置を選択
    for i, indices in enumerate(itertools.combinations(positions, n)):
        # 新しいリストを作成してスペースを挿入
        new_str = []
        last_index = 0
        for index in indices:
            new_str.append("".join(s[last_index:index]))
            new_str.append(" ")
            last_index = index
        new_str.append("".join(s[last_index:]))

        # print(new_str)
        # リストを結合して文字列として出力
        print(str(i).ljust(2, " ") + ":", "".join(new_str))


# テスト: "helloworld"に2つのスペースを挿入
insert_spaces(["!", "a", "&&", "!", "b"], 2)
