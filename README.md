# About the project

![alt text](https://github.com/Nelis-sol/pricelocker-program/blob/main/pricelocker.png "Why.fun logo")


## Pricelocker
With pricelocker, users can lock up their tokens until they reach a certain price. Once the token price exceeds the price set on the locker, the tokens get unlocked and can be withdrawn. 
The goal of the pricelocker is aligning incentives between project teams, influencers, developers, AI agents and the community. Only when one driven value to the token price, can their tokens be unlocked.winning callers). This includes prompting the AI agent and instructing it on how to judge winning and losing attempts. Gamecall also allows calls through the browser.

**Functionalities of the pricelocker:**
  * **create locker** (creates PDA and a token account that will serve as the vault for the tokens)
  * **deposit funds** (deposit a token that you want to lock up)
  * **create price lock** (determine the price at which the tokens unlock)
  * **create time lock** (instead of a price lock, users can also choose for a time lock - or do both: e.g. unlock at X price AND after 3 months)
  * **unlock price lock** (unlock tokens)
  * **unlock time lock** (unlock tokens)
  * **withdraw funds** (withdraw unlocked funds from the locker to the user wallet)

<br />

## Devnet and published IDL
This program is deployed on Solana devnet with program id: `BPVWMMj1eEALBvsR3TQZC2Zb3vt8jQkvzBYeJsoaKum7`.<br />
The IDL is published onchain:
[Solscan - Devnet]([https://solscan.io/account/BPVWMMj1eEALBvsR3TQZC2Zb3vt8jQkvzBYeJsoaKum7?cluster=devnet#anchorProgramIdl](https://solscan.io/account/BPVWMMj1eEALBvsR3TQZC2Zb3vt8jQkvzBYeJsoaKum7?cluster=devnet#anchorProgramIdl))

<br />

## Built with

- [x] **Rust**
- [x] **Anchor**  
- [x] **Pyth** (https://pyth.network)
- [x] **React**

<br />

____

<br />

## Install & run

### 1. Install Rust, Cargo
```
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
```

If you have Rust already installed, make sure to update to the latest stable version.
```
$ rustup update
```
<br />

### 2. Install Anchor, AVM
```
$ cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
$ avm install 0.29.0
$ avm use 0.29.0
```
<br />

### 3. Deploy program on devnet
Update the program id if necessary (in the `lib.rs` and `anchor.toml` files).

```
$ cd price-locker-ui
$ npm i
```
<br />

### 4. Install npm dependencies
```
$ cd price-locker-ui
$ npm i
```
<br />

### 5. Run UI
```
$ npm run dev
```

<br />

# Contact
Contact me via email (nelis.sol@protonmail.com) or on X (@nelis-sol)

<br /><br />

