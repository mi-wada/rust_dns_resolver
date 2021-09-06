# DNS(Domain Name System)
## 概要
ドメイン名を受け取りIPアドレスを返す．
データベース全体の一貫性のあるコピーの収集は不可能．各部で分散管理するべき．

## 主要なセグメント
1. ドメイン名空間とリソースレコード
ツリー構造を持つ名前空間と，名前に関連付けられたデータに関する仕様

2. ネームサーバー
地震が管理する完全な情報と，他の情報にアクセスするためのポインタをもつ．

3. リゾルバー
クライアントのリクエストに答えてネームサーバーから除法を抽出するプログラム．

## 標準的メッセージフォーマット

- Header
  - ID(16 bit)
    - 送信元のプログラムが発行するID
  - QR(1 bit)
    - Question(0) or Response(1)
  - OPCODE(4 bit)
    - メッセージの種類(0 標準問い合わせ(QUERY), 1 逆問い合わせ(IQUERY), 2 サーバー状態要求(STATUS))
  - AA(1 bit)
    - 権威のある応答であること
  - TC(1 bit)
    - TrunCation
    - メッセージが切り捨てられたことを示す
  - RD(1 bit)
    - Recursion Desired
    - 問い合わせ時，再帰を要求するかどうか
  - RA(1 bit)
    - Recursion Available
    - 応答時，再帰が有効かどうか
  - Z(1 bit)
    - 予約(default: 0)
  - RCODE(4 bit)
    - 応答コード(0: エラーなし, 1: フォーマットエラー, 2: サーバー障害, 3: 名前エラー, 4: 未実装, 5: 拒否)
  - QDCOUNT
    - 符号なし16ビット整数，Questionセクション内のエントリ数
  - ANCOUNT
    - Anserセクション内のリソースレコード数
  - NSCOUNT
    - AUTHORITYセクション内のネームサーバーリソースレコード数
  - ARCOUNT
    - Additionalセク四内のリソースレコード数
- Question(問い合わせの際の質問内容)
  - QNAME
    - 問い合わせたいドメイン名．レングスオクテットのあとにその数だけオクテットが続く．ドメイン名はルートのヌルラベルを表す長さ0のオクテットで終了する
    - ex: 5google3com0
    - **圧縮について**
      - ドメイン名全体もしくはドメイン名の末尾のラベルのリストが前に同じ名前の出現した位置を指すポインタに置き換えられる
        - ポインタは必ず末尾に登場する
      - レングスオクテットの先頭ビットが11(0xC0)のとき，ポインタを表す
        - ラベルは63オクテット以下なので，長さは192(0xC0)にはなりえない
  - QTYPE
    - https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.2 & https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.3
  - QCLASS
- Answer
  - Name
    - このリソースレコードが所属するドメイン名
    - 圧縮される
      - 先頭2bitが11ならアドレス，2オクテット，メッセージの先頭からのオフセット(0ならID)
  - TYPE
    - RRのタイプコード一つを含む2オクテット．RDAAフィールド内の情報を表す
    - https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.2
  - CLASS
    - 2オクテット．RDATAフィールド内の情報のクラスを表す．
    - https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.4
    - IN: Internet, CS: CSNET, CH: CHAOS, HS: Hesiod
  - TTL
    - 符号なし32ビット．破棄されるまでにキャッシュして良い秒数．0はキャッシュすべきでないことをしめす．
  - RDLENGTH
    - 符号なし16ビット．RDATAフィールドの長さをオクテット単位で表す．
  - RDATA
    - このリソースを表す可変長のオクテット文字列．この情報のフォーマットはリソースレコードのTYPEとCLASSに依存する．TYPEがA, CLASSがINの場合，このフィールドは4オクテットのARPAインターネットアドレス．
- Authority
  - Answerに同じ
- Additional
  - Answerに同じ

### test
#### query_packet.txt
```
34 e7, id
0 0000 0 0 1, qr(1b), opcode(4b), aa(1b), tc(1b), rd(1b)
0 010 0000, ra(1b), z(3b), rcode(4b)
00 01 00 00 00 00 00 00
```

#### request_packet.txt
```
34 e7 id,
1 0000 0 0 1, qr(1b), opcode(4b), aa(1b), tc(1b), rd(1b)
1 000 0000, ra(1b), z(3b), rcode(4b)
00 01 00 01 00 00 00 00


00000000  34 e7 81 80 00 01 00 01  00 00 00 00 06 67 6f 6f  |4............goo|
00000010  67 6c 65 03 63 6f 6d 00  00 01 00 01 c0 0c 00 01  |gle.com.........|
00000020  00 01
00 00 00 2a ttl
00 04 rdlength
ac d9 1f ae              |.....*......|
```

12 oc
02 oc: id(16b)
01 oc: qr(1b), opcode(4b), aa(1b), tc(1b), rd(1b)
01 oc: ra(1b), z(3b), rcode(4b)
02 oc: qdc(16b)
02 oc: anc(16b)
02 oc: nsc(16b)
02 oc: arc(16b)
