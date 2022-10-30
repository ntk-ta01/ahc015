# 考察メモ

## 15時

推定系っぽい？まあ貪欲を考えるか

キャンディー1つを受け取って箱を傾けるので1ターン。どのキャンディが何個来るかなどは最初にわかる。位置がわからないのか。

**ランダムに傾ける**: seed0 10万点 提出28M

傾けたら何点になるか調べて一番高い向きに傾ける

**1ターン先読み**: seed0 35万点 提出69M

## 16時

ビジュアライザをよく見る。1ターン先読みの同点とかありそう

turn 8で来たイチゴを左に傾ければ、既にある連結成分と繋がる可能性が高いのに、下に傾けているのはもったいない

![turn 8](vis1.png) ![turn 9](vis2.png)

何も次のターン置かないとして2ターン傾けを先読みする？

**2ターン先読み(2ターン目は何も置かない)**: seed0 14万点 提出31M

さすがに2ターン目もどこかに置いた方がいいんじゃないか？？？

100ターン目はどこに傾けても変わらない（動かせないから）。って考えると重要なのは序盤なのかな？？？

**2ターン先読み(2ターン目はランダムに置く)**: seed0 29万点 提出90M

ここからの方針は

1. 2ターン目に置くやつをいろいろ試して精度を良くしようとする
2. 3,4,5ターン目と先読みターン数を増やす

の2通りに分かれる。とりあえず2.を試してみる。

**3ターン先読み(2,3ターン目はランダムに置く)**: seed0 47万点 提出93M

## 17時

**3ターン先読み(2,3ターン目はランダムに置く) 実装2**: seed0 63万点 提出87M

testerを用意して、100ケースローカル実行

**5ターン先読み(2ターン目以降はランダムに置く) 実装2**: seed0 32万点 local_score:92396398 提出90M 1366ms

ターン数を上げてもこのままじゃ伸びない！精度を上げる方をやろう

## 18時
