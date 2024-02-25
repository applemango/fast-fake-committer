# About
高速で空のCommitをするプログラムです

[実際にこれを使用して生成されたリポジトリ](https://github.com/applemango/fake-24m-commits)

> [!CAUTION]
> 大量のCommitをすることは、利用規約で禁止されている行為ではありませんが、アカウントが停止されたりBanされる可能性があります


# How to use
1. `head.rs`の29-31行目と`commit.rs`の60-62行目を適当に変える
2. `cargo build --release`で`build`
3. 実行
```
(...)/fast-fake-committer -g (...)/git-test/.git -t 4b825dc642cb6eb9a060e54bf8d69288fbee4904
```
> [!NOTE]
> `-g`はリポジトリの`path`、`-t`は`tree`を表している
>
> `tree`は適当なCommitを`git cat-file -p commit_id`でみれる
>
> `commit_id`は`git log`でみれる


# Benchmark
|this|git|
|---|---|
|10M / hour|200K / hour|

> [!IMPORTANT]
> 厳密に測定したわけではなく、あくまでも目安です。

> [!NOTE]
> `git`は`go`言語の`os/exec`を使用し、下記のコマンドを実行し測定しました
> ``` 
> git commit --allow-empty --allow-empty-message --no-edit --date "2006-01-02 15:04:05 -0700"
> ```

# Time
commitは1時間に1千万程度できますが、pushにはそれ以上の時間を要します
具体的には、24M Commitを一度にpushした場合以下のような時間がかかります

1. 24M Commitの生成に2時間
2. それのpushに18時間
3. pushが完了し、profileのcontributionsに完全に表示されるまで推定70時間[^1]

[^1]: 経過時間をX, 実際に進んだ量をYとし、逆数回帰で回帰分析し、その計算結果を元に推計[^2]
[^2]: あまりに時間が長く、実際に計測するのが困難なため予測値 ( ただ単に待つのが嫌なだけ )