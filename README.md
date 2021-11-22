Because dd doesn't respect bs and count on Linux*

use `make alpine` for an executable that will work on alpine.

Examples:

    $ truncate -s 1k file

    $ cargo run -q -- read -p file -o 0 -c 2
    read from block 0:
    00000000: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000010: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000020: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000030: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000040: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000050: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000060: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000070: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000080: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000090: 0000 0000 0000 0000 0000 0000 0000 0000 
    000000a0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000000b0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000000c0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000000d0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000000e0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000000f0: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000100: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000110: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000120: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000130: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000140: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000150: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000160: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000170: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000180: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000190: 0000 0000 0000 0000 0000 0000 0000 0000 
    000001a0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000001b0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000001c0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000001d0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000001e0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000001f0: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000200: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000210: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000220: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000230: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000240: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000250: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000260: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000270: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000280: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000290: 0000 0000 0000 0000 0000 0000 0000 0000 
    000002a0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000002b0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000002c0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000002d0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000002e0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000002f0: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000300: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000310: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000320: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000330: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000340: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000350: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000360: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000370: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000380: 0000 0000 0000 0000 0000 0000 0000 0000 
    00000390: 0000 0000 0000 0000 0000 0000 0000 0000 
    000003a0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000003b0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000003c0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000003d0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000003e0: 0000 0000 0000 0000 0000 0000 0000 0000 
    000003f0: 0000 0000 0000 0000 0000 0000 0000 0000

    $ cargo run -q -- write -p file -o 1 -c 1 -v 255
    write to block 1:
    00000200: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000210: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000220: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000230: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000240: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000250: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000260: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000270: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000280: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000290: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002a0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002b0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002c0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002d0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002e0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002f0: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000300: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000310: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000320: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000330: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000340: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000350: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000360: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000370: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000380: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000390: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003a0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003b0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003c0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003d0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003e0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003f0: ffff ffff ffff ffff ffff ffff ffff ffff

    $ cargo run -q -- write-random -p file -o 0 -c 1
    write to block 0:
    00000000: aed3 27e4 7609 37ca 89ce d3b1 e910 5527 
    00000010: 5881 ce93 b547 67ab a578 0ddd 3dfb 2f23 
    00000020: d417 9eca af4e 7d3e 1bba 6536 1932 e15a 
    00000030: 2cf5 684d ab3a 7862 c2fc 8c98 db7d 2cc4 
    00000040: ff7f 4a9b 3aef 1ab9 8543 8783 2e1e b16a 
    00000050: f56b 5384 6e52 f79f 382a a335 c459 a300 
    00000060: 2d47 34ea adbd 3b8a 3241 0981 9ef1 8c53 
    00000070: 20ee 49ff e3f0 e466 7a28 3c81 43ed 0b7c 
    00000080: dfbc e087 42b2 e665 84ad 8297 8d40 3fe3 
    00000090: c349 e424 87ea 36b8 3d55 4e87 421b 66b0 
    000000a0: a1d2 60e5 b177 3b3e 9f08 4787 b6cb b8f5 
    000000b0: 85cf 63ea 209e 3aa8 ed54 2c4c c854 5cf0 
    000000c0: 7e67 d393 5395 a2e4 a6f5 50d5 e810 e4a7 
    000000d0: f143 5675 df3a c10f c652 7364 479e 9a49 
    000000e0: be53 34ae c85b 1558 3dc1 aa8c d304 06e4 
    000000f0: de35 9e74 908b ff74 19af 0b51 4668 b8fa 
    00000100: 6499 de03 f1be c433 9eda 3f06 2682 98e2 
    00000110: 6e20 6d60 fe6b 0a71 8a37 2ed8 6148 6426 
    00000120: 67fa 0c01 4948 ba34 4c92 c21d 717a d6bb 
    00000130: 0194 5164 37e1 5199 a8d1 8c86 4c86 5139 
    00000140: d871 d332 d056 1879 e569 6070 7a25 8833 
    00000150: 1874 b4e3 c33f 8772 4786 53b1 5b50 5147 
    00000160: 24be 8860 93bd c98b 5861 d2d0 2d03 dc76 
    00000170: e32c d493 e56e b2e5 67d4 bcb1 00df 1deb 
    00000180: b5c3 284f 4856 ff23 e794 5d15 3cf7 07df 
    00000190: e18f 24b8 4c5a 0489 380c 4a1d a079 7564 
    000001a0: ca5b 0e74 2fc3 64fd 8212 ca57 6c92 7be5 
    000001b0: b647 8526 cb92 ff93 67b9 ba83 fe3d 9439 
    000001c0: e637 326d ee0d aa4a 8294 7b39 d5e7 4c72 
    000001d0: fb73 dec4 b02f 0faa 8ce4 5630 6609 7b46 
    000001e0: a55f 0b61 982d 5dfd 7142 cfdd 9d6b dd58 
    000001f0: 9aa3 8ebd 377c bec6 a2c3 e266 eb95 06d3

    $ cargo run -q -- read -p file -o 0 -c 2
    read from block 0:
    00000000: aed3 27e4 7609 37ca 89ce d3b1 e910 5527 
    00000010: 5881 ce93 b547 67ab a578 0ddd 3dfb 2f23 
    00000020: d417 9eca af4e 7d3e 1bba 6536 1932 e15a 
    00000030: 2cf5 684d ab3a 7862 c2fc 8c98 db7d 2cc4 
    00000040: ff7f 4a9b 3aef 1ab9 8543 8783 2e1e b16a 
    00000050: f56b 5384 6e52 f79f 382a a335 c459 a300 
    00000060: 2d47 34ea adbd 3b8a 3241 0981 9ef1 8c53 
    00000070: 20ee 49ff e3f0 e466 7a28 3c81 43ed 0b7c 
    00000080: dfbc e087 42b2 e665 84ad 8297 8d40 3fe3 
    00000090: c349 e424 87ea 36b8 3d55 4e87 421b 66b0 
    000000a0: a1d2 60e5 b177 3b3e 9f08 4787 b6cb b8f5 
    000000b0: 85cf 63ea 209e 3aa8 ed54 2c4c c854 5cf0 
    000000c0: 7e67 d393 5395 a2e4 a6f5 50d5 e810 e4a7 
    000000d0: f143 5675 df3a c10f c652 7364 479e 9a49 
    000000e0: be53 34ae c85b 1558 3dc1 aa8c d304 06e4 
    000000f0: de35 9e74 908b ff74 19af 0b51 4668 b8fa 
    00000100: 6499 de03 f1be c433 9eda 3f06 2682 98e2 
    00000110: 6e20 6d60 fe6b 0a71 8a37 2ed8 6148 6426 
    00000120: 67fa 0c01 4948 ba34 4c92 c21d 717a d6bb 
    00000130: 0194 5164 37e1 5199 a8d1 8c86 4c86 5139 
    00000140: d871 d332 d056 1879 e569 6070 7a25 8833 
    00000150: 1874 b4e3 c33f 8772 4786 53b1 5b50 5147 
    00000160: 24be 8860 93bd c98b 5861 d2d0 2d03 dc76 
    00000170: e32c d493 e56e b2e5 67d4 bcb1 00df 1deb 
    00000180: b5c3 284f 4856 ff23 e794 5d15 3cf7 07df 
    00000190: e18f 24b8 4c5a 0489 380c 4a1d a079 7564 
    000001a0: ca5b 0e74 2fc3 64fd 8212 ca57 6c92 7be5 
    000001b0: b647 8526 cb92 ff93 67b9 ba83 fe3d 9439 
    000001c0: e637 326d ee0d aa4a 8294 7b39 d5e7 4c72 
    000001d0: fb73 dec4 b02f 0faa 8ce4 5630 6609 7b46 
    000001e0: a55f 0b61 982d 5dfd 7142 cfdd 9d6b dd58 
    000001f0: 9aa3 8ebd 377c bec6 a2c3 e266 eb95 06d3 
    00000200: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000210: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000220: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000230: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000240: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000250: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000260: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000270: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000280: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000290: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002a0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002b0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002c0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002d0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002e0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000002f0: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000300: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000310: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000320: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000330: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000340: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000350: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000360: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000370: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000380: ffff ffff ffff ffff ffff ffff ffff ffff 
    00000390: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003a0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003b0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003c0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003d0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003e0: ffff ffff ffff ffff ffff ffff ffff ffff 
    000003f0: ffff ffff ffff ffff ffff ffff ffff ffff


*: unless you use [raw](https://man7.org/linux/man-pages/man8/raw.8.html)

