# sol_program

1. This is a sample program with anchor for Solana.
2. You need Rust, Solana, Anchor already installed on your machine, in case you don't have this, please go through my video - https://www.youtube.com/watch?v=g6-OHQA2r98&t=19s
3. In the description of that video, there is a document, that'll make it very easy for you to setup Solana.
4. You also need to create a solana key in case you don't have it, `solana-keygen new`
5. Once you have your key, please make sure you change the Anchor.toml file, the wallet field with your system's path for the `json` file.
6. Get your Solana validator running `solana-test-validator` - this starts a solana node
7. Airdrop yourself some sol - `solana airdrop 2` - you need some amount to deploy your program, if you have none, you will get an error.
8. We are deploying to our local net right now. Make sure you check the Anchor.toml file and make changes in case you want to deploy to devnet or mainet.
9. Run `Anchor test`to see if all tests pass (I made sure all tests pass before committing)
10. Run `Anchor build` to build the code and create the signed binary
11. Run `Anchor deploy` to deploy to localnet.
12. Needless to say, your validator node needs to be running when you're performing all these operations.
