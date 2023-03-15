# Unique Delegation Manager (UDM) Javascript SDK

Unique Delegation Manager (UDM) is a toolset built for managing a "master-delegate" relationship between 1-to-many wallets. Use UDM SDK to interact with UDM Solana program and accounts.

The main account to interact with is the `delegation account`. The delegation account is essentialy an on chain statement which confirms that the representative has the authority to execute smart contract actions that are otherwise reserved for the master. This account can also be used by projects to display asset ownership by proxy and give other logical priviledges. Two invovled parties that makes delegation are master and representative. The `master` is the one who initiated delegation account. The `representative` is the one who was invited to represent the master.

## Installation

### For use in Node.js or a web application

```
$ npm install --save @unique.vc/udm.js

```

## Main features

### Check if delegation between master and representative exists

`checkIfDelegationExists` - This SDK function checks if delegation account for passed master and representative public keys exists and if is confirmed.

### Get delegation address

`getDelegationAddress` - This SDK function returns address of the Delegation PDA for specific master and representative public keys.

### Get all masters for specific representative

`getMastersForPublicKey` - This SDK function returns all delegation accounts with master addresses that are confirmed and related to the passed representative public key.

### Get all representatives for specific master

`getRepresentativeForPublicKey` - This SDK function returns all delegation accounts with representative addresses that are confirmed and related to the passed master public key.

## Example

```tsx
import { useAnchorWallet } from "@solana/wallet-adapter-react";
import { Connection } from "@solana/web3.js";
import { getMastersForPublicKey, IDelegation } from "@unique.vc/udm.js";
import { FC, useEffect, useState } from "react";

export const SOLANA_ENDPOINT = "https://api.devnet.solana.com";
export const RPC_CONNECTION = new Connection(SOLANA_ENDPOINT, "confirmed");

const UdmExample: FC = () => {
  const [allDelegations, setAllDelegations] = useState<IDelegation[]>();
  const wallet = useAnchorWallet();

  useEffect(() => {
    void getAllDelegationsForRepresentative();
  }, [wallet]);

  const getAllDelegationsForRepresentative = async () => {
    if (wallet) {
      const allDelegationsForRepresentativeFromSDK =
        await getMastersForPublicKey(wallet.publicKey, RPC_CONNECTION);
      setAllDelegations(allDelegationsForRepresentativeFromSDK);
    }
  };

  return (
    <div>
      <p>Representative: {wallet?.publicKey.toString()}</p>
      <p>Masters:</p>
      {allDelegations?.map((item) => (
        <div>
          <p>{item.master.toString()}</p>
        </div>
      ))}
    </div>
  );
};

export default UdmExample;
```

## Live demo - UDM dapp

<a href="https://unique-delegation-manager.surge.sh/" target="_blank">react-day-picker-v7.netlify.app</a>
