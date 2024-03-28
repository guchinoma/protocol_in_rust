#!/bin/bash

#
#
# 注意！ 仮想環境以外でこれを実行させるな。面倒臭いことになる
#
#

echo "注意! 仮想環境以外で動かすな"
echo "仮想環境でこれ実行させると、ネットワーク設定を直す必要がある。"
echo "仮想環境だと確認していますか？"
echo "確認しているならyと打つ"

echo "Caution! Do not run this sh in non-virtual environment."
echo "If you run this, you need to fix network setting of your laptop, which is troublesome."
echo "Do you recognize your env is now virtual environment?"
echo "If you do, say \"y\""

read -p ":" INPUT_STR

case "$INPUT_STR" in
  [yY]) echo "go on" ;;
  *) exit 1;;
esac

sudo ip tuntap add mode tap user $USER name tap0
sudo ip addr add 192.0.2.1/24 dev tap0
sudo ip link set tap0 up
