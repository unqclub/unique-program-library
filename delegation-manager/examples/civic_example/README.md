<div align="center">
  <h1>UDM - Civic Example</h1>

  <p>
    <strong>Unique Delegation Manager</strong>
  </p>
</div>

This is an example program, showing a way to use Unique Delegation Manager together with Civic. It contains a single instruction, 'increment_counter'. Specifically for this example, the master wallet should be the one invoking the instruction first, as it sets the authority on the Counter PDA account to be that wallet and increments the counter by 1. After that, a Representative wallet can invoke that same instruction, and it will check if the Master wallet is KYC verified, not the wallet signing, and the Unique Delegation Manager program will ensure that the Master wallet that was KYC checked truly is the master wallet of the signer. This way, both the KYC verified wallet and all it's representative wallets are able to increment the counter.
