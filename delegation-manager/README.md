# Unique Delegation Manager - UDM

An open-source, shared protocol for managing wallet delegation.

# About

Unique Delegation Manager is a toolset built for managing a "master-delegate" relationship between 1-to-many wallets. Protocols that implement it can allow safe execution of numerous actions for users without exposing their assets to any risks.

# Background

One of the core problems that the web3 industry is facing is a constant compromise between user experience and self-custody. While security is paramount, we cannot onboard the next billion users without UX that is human friendly. UDM was created with a goal to address this problem.

**UDM is open source and will always be open source. The goal is to also bring it as fast as possible to the point where the main program can be made immutable.**

UDM is a tool that allows users to create a "master-delegate" relationship between wallets. One master wallet can have multiple delegates, and one wallet can be a delegate to numerous master wallets.

**Delegates don't have, under any circumstances, an option to operate assets on the master account.**

From the user perspective, it's a simple interface that allows to manage such connections (create or delete). There is also a CLI version for advanced users, or an option to directly interact with the program using instructions.

From the app developer perspective, there are SDKs for programs and front-end that allow to validate the existence of master-delegate on-chain relationships and check master wallet assets.

That opens a number of use cases and possibilities, here are some examples:

1. If a user wallet is whitelisted for an NFT mint, he doesn't need to connect the main wallet to the minting site, but rather put funds for minting into a delegate wallet and only use that one.
2. If an app has claim functionality, it can allow delegates to claim reward, but send it to the master account.
3. Cross-device compatibility does not require copy-pasting the seed phrase or private keys any more. For instance, a mobile app can create a wallet for a new user via web3auth integration, and the user can add that new wallet as a delegate to his main wallet and have a frictionless mobile experience.

**User experience will become better with each app adopting the solution, so we encourage all developers to consider implementing it.**
