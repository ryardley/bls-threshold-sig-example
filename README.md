# BLS Signature example

This is an example of how to use BLS threshold signatures in rust for encryption and decryption.

```

We have three secret keys:

        SecretKeyShare(Scalar(0x3d0cb3b942adafb941b0b5b2f18dc88ae8d4041503ec77f2261f1012d84a1f2c))
        SecretKeyShare(Scalar(0x516cc06fccd0f51405c35f6cac42de34b5862772e2495369703572cdb6493d41))
        SecretKeyShare(Scalar(0x65cccd2656f43a6ec9d6092666f7f3de82384ad0c0a62ee0ba4bd58894485b56))

That together form a public key:

        8654a8f9424c9f24fd90889828632c1aa6649db3b93ae4dbef3b4eec844150bb8af1cdf30ec8215aec254947376be891

I create a message:

        Hello universe!

Then encrypt it to the above public key:

        aab27e58b23b1a1f8b813ffcb7b160ae70d1453ca16c81c7865892c12a3eb843b68b8fb13acd75e6daf78a53df6c5b1c8de0b70e8b82810574e03c6c00af1efc2e8a6b91d025da91301c5a0ba16a996c329d8311b8738543286f13a20f1881910ea991489a1d4170494152c8174d28358b8037518b3ca721b738b62088fb909ca7f139cbe225f719758f414a5231e34245c7c24efa245d5c479bf47276d542

I decrypt the message with two of the secret keys giving me these pieces:

        ab38e29d0bdb836989f0dff77e847c04adad5f43b4c9d5f367264287c5a719bba733fe22aa68c15168e937aab7c93977
        b9a6ad97e4654de9b9cc7c88317a6c466ec6f0c0fe5a0da79956e7103ef26d7963997b6de2f9cc04d78f5bd2db01f04e

Then reassemble the decryption pieces:

        Hello universe!

I can also decrypt the message with one of the other secret keys giving me these pieces:

        ab38e29d0bdb836989f0dff77e847c04adad5f43b4c9d5f367264287c5a719bba733fe22aa68c15168e937aab7c93977
        b5255f73ea5c873be3ffdcb520b9a51a76f34b217830c7a308e11a91341e02a383f88db8072e941f320f3e9c8701d369

Then reassemble the decryption pieces:

        Hello universe!
```
