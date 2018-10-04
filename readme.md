rbkcrack
=======

Crack legacy zip encryption with Biham and Kocher's known plaintext attack.

(Fork from bkcrak.)

[![Linux build status](https://travis-ci.org/Aloxaf/rbkcrack.svg)](https://travis-ci.org/Aloxaf/rbkcrack)

Download
--------

Get the latest version from the [git repository](https://github.com/Aloxaf/rbkcrack).

Install
-------

Build and install it with [Cargo](https://doc.rust-lang.org/cargo).

```shell
cargo install --git https://github.com/Aloxaf/rbkcrack
```

Usage
-----

### Data required

The attack uses at least 12 bytes of contiguous plaintext.
The larger the known plaintext, the faster the attack.

#### From zip archives

Having a zip archive `encrypted.zip` with the entry `cipher` being the ciphertext and `plain.zip` with the entry `plain` as the known plaintext, bkcrack can be run like this:

    rbkcrack -C encrypted.zip -c cipher -P plain.zip -p plain

#### From files

Having a file `cipherfile` with the ciphertext (starting with the 12 bytes corresponding to the encryption header) and `plainfile` with the known plaintext, bkcrack can be run like this:

    rbkcrack -c cipherfile -p plainfile

#### Offset

If the plaintext corresponds to a part other than the beginning of the ciphertext, you can specify an offset.
It can be negative if the plaintext includes a part of the encryption header.

    rbkcrack -c cipherfile -p plainfile -o offset

### Decipher

If the attack is successful, the deciphered text can be saved:

    rbkcrack -c cipherfile -p plainfile -d decipheredfile

If the keys are known from a previous attack, it is possible to use bkcrack to decipher data:

    rbkcrack -c cipherfile -k 12345678 23456789 34567890 -d decipheredfile

### Decompress

The deciphered data might be compressed depending on whether compression was used or not when the zip file was created.
If deflate compression was used, a Python 3 script provided in the `tools` folder may be used to decompress data.

    tools/inflate.py < decipheredfile > decompressedfile

You can also use `-u` option to enable decompress

    rbkcrack -C encrypted.zip -c cipher -P plain.zip -p plain -d final -u

Learn
-----

For more information, have a look at the documentation and read the source.

Contribute
----------

Do not hesitate to suggest improvements or submit pull requests on [github](https://github.com/Aloxaf/rbkcrack).

License
-------

This project is provided under the terms of the [zlib/png license](http://opensource.org/licenses/Zlib).
