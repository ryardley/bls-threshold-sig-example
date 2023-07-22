# BLS Signature example

This is an example of how to use BLS threshold signatures in rust for encryption and decryption.

```

We have three secret keys:

        SecretKeyShare(Scalar(0x3a3bdffe6a9c3e3c52acb0eb8a57214e6f0171be54b208be83c4354a364f88b7))
        SecretKeyShare(Scalar(0x230b229a6d9460fea747fb06e9eb71ed693e9a7f8bcb54883334e61de504736a))
        SecretKeyShare(Scalar(0x0bda6536708c83c0fbe34522497fc28c637bc340c2e4a051e2a596f193b95e1d))

That together form a public key:

        99c429c2a9515edcb47bae5a85f7525ec830955d807b5d29d8cf87d39b71039f82608f9f8499f431b491e599ded0e083

I create a message:

        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

Then encrypt it to the above public key:

        b85a4a3e25c80f54f330e4db4d3ccd1bf11bd362cfbf0d4051eb418be5baebfca811d462b3e98d06559963b72062f81a99a0f97d21b0ff4ce5044e27e3252a675f414291c97fb42ea19eb67dbd0678ee5d77392154013fb9887443a903a13fb90c675c6869b2e26ec6e5eb7d3ea6fcbed3f72b16343ddb22498d4863f7cec495bd3678f675d691868ed372fd8c1b4cb8a8eaab6ac1c0286803290cc3543624adde96880fec73488caaed48ce1720fb889c667ccd9f0a43ea01a7f0e3380009cfc0d5aa5a445948af44e862266b85988d44125c61f0330190a5791077e00077e3672bc87271502881f60c806cef6ab39e61b4bbc675bfea22f4f19fc4f2450645138813bf94b69c05a015196aa031caa14239effd8c0575510a3ed45786b490e8d7f7b796470dc3d167c391cd0ca7b7a2f2623390f3c1b4ac831ffc597842959fda63202849ec0305a405bd2e47cf82833ef09b22934b4c4ad9c8001aacd8f2f112c151df060741f6b5169ea4b9d7a3632e53c9b6b4e2c89f5a57d98b7cb3945b87e0b682dfd28868679361e36c1e66b1d7357ec6e44ddb5a25ba1739818195a84040e313f32e0e48b1855002b7ae9c393c9fd0972b72226f7c06fc4d54793223258f972376a18bf311685e4ad03963343bd54da9b39f78d00f659cbf7be1a2a66de58dd33b56a1e2f9176b00ec6e3c6267cddd0494e3023902df83bcda0fb9555e00efbed06823168ccde13ec678d9053910ef2f63069443dbdfc954b2382aea8377f82170f49a6547cb03a814ae025bd3aad7b9f76cfee79d66d90eea199d6c3cfca2eb879defe8804580c7e1

I decrypt the message with two of the secret keys giving me these pieces:

        b9e197424551e5772d822187a2d4689ac633cd5c670198a6b654afba3aaa52b8feaa309814cfb1f6571859e833ab1898
        a499f16fdedd8747ae2de77d54bdea11739243d2dbd6e9de28f068b5beb6ec14508f4794b3117356a851e1b18cdb4afe

Then reassemble the decryption pieces:

        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

I can also decrypt the message with one of the other secret keys giving me these pieces:

        b9e197424551e5772d822187a2d4689ac633cd5c670198a6b654afba3aaa52b8feaa309814cfb1f6571859e833ab1898
        8d9e059b5bfc10c8866d563b9f3917d3f31cae905be493aa4736e8b4e91b4393e1639107abb3f74b59cc524a78da6bb0

Then reassemble the decryption pieces:

        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

```
