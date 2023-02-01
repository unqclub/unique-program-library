<div align="center">
  <h1>UDM - Basic Example</h1>

  <p>
    <strong>Unique Delegation Manager</strong>
  </p>
</div>

This program shows an example of using the Unique Delegation Manager in a smart contract. It contains a single instruction, 'increment_counter'. The first time it's invoked it creates a Counter PDA account, and sets its authority to the one who signed the transaction. Each consecutive time it's invoked, it checks if its invoked by the one who created the Counter account. If the signer isn't the one who created it, it checks if the authoriti was delegated to the signer of the transaction, so that he can increment the counter in the name of the one who created it. If the Delegation account exists, the payer was authorised to represent the original authority of the Counter, an he has accepted the Delegation, the counter is incremented.
