import {
  AccountInfo,
  Connection,
  ParsedAccountData,
  PublicKey,
  RpcResponseAndContext,
} from '@solana/web3.js';
import { authorizeSeed, programFactory } from './common/constants';
import { ITokenAccountInfo, IDelegation } from './common/interfaces';

export const checkNumberOfTokenAccounts = async (
  connection: Connection,
  representative: PublicKey,
  master: PublicKey,
  mint: PublicKey
): Promise<ITokenAccountInfo> => {
  try {
    let mastersTokenAccounts;
    const tokenAccountsForPublicKey = await connection.getParsedTokenAccountsByOwner(
      representative,
      {
        mint: mint,
      }
    );

    try {
      mastersTokenAccounts = (
        await getAllMastersTokenAccountsWithSpecifiedMint(connection, representative, master, mint)
      )?.value.length;
    } catch (error) {
      mastersTokenAccounts = undefined;
    }
    return {
      numberOfRepresentativeTokenAccounts: tokenAccountsForPublicKey.value.length,
      numberOfMasterTokenAccounts: mastersTokenAccounts,
    };
  } catch (error) {
    console.log(error);
    throw error;
  }
};

export const getAllMastersTokenAccountsWithSpecifiedMint = async (
  connection: Connection,
  representative: PublicKey,
  master: PublicKey,
  mint: PublicKey
): Promise<
  RpcResponseAndContext<
    {
      pubkey: PublicKey;
      account: AccountInfo<ParsedAccountData>;
    }[]
  >
> => {
  try {
    const program = programFactory(connection);

    const [delegation] = PublicKey.findProgramAddressSync(
      [authorizeSeed, master.toBuffer(), representative.toBuffer()],
      program.programId
    );

    const delegationInfo = await program.account.delegation.fetch(delegation);
    if (!delegationInfo) {
      throw new Error('Delegation does not exist');
    }
    if (!delegationInfo.authorised) {
      throw new Error('You need to confirm delegation in order to represent this master');
    }
    const tokenAccounts = await connection.getParsedTokenAccountsByOwner(master, {
      mint: mint,
    });
    return tokenAccounts;
  } catch (error) {
    console.log(error);
    throw error;
  }
};

export const getRepresentativeForPublicKey = async (
  publicKey: PublicKey,
  connection: Connection
): Promise<IDelegation[]> => {
  try {
    const representations: IDelegation[] = [];
    const program = programFactory(connection);
    const allDelegationsAccounts = await program.account.delegation.all([
      {
        memcmp: {
          offset: 8,
          bytes: publicKey.toBase58(),
        },
      },
    ]);
    allDelegationsAccounts.forEach((item) => {
      item.account.authorised &&
        representations.push({
          authorised: item.account.authorised,
          master: item.account.master,
          representative: item.account.representative,
        });
    });

    return representations;
  } catch (error) {
    console.log(error);
    throw error;
  }
};

export const getMastersForPublicKey = async (
  publicKey: PublicKey,
  connection: Connection
): Promise<IDelegation[]> => {
  try {
    const representations: IDelegation[] = [];
    const program = programFactory(connection);
    const allDelegationsAccounts = await program.account.delegation.all([
      {
        memcmp: {
          offset: 8 + 32,
          bytes: publicKey.toBase58(),
        },
      },
    ]);
    allDelegationsAccounts.forEach((item) => {
      item.account.authorised &&
        representations.push({
          authorised: item.account.authorised,
          master: item.account.master,
          representative: item.account.representative,
        });
    });

    return representations;
  } catch (error) {
    console.log(error);
    throw error;
  }
};

export const getAllMasters = async (connection: Connection): Promise<IDelegation[]> => {
  try {
    const representations: IDelegation[] = [];
    const program = programFactory(connection);
    const allDelegationsAccounts = await program.account.delegation.all();
    allDelegationsAccounts.forEach((item) => {
      item.account.authorised &&
        representations.push({
          authorised: item.account.authorised,
          master: item.account.master,
          representative: item.account.representative,
        });
    });

    return representations;
  } catch (error) {
    console.log(error);
    throw error;
  }
};

export const getAllRepresentative = async (connection: Connection): Promise<IDelegation[]> => {
  try {
    const representations: IDelegation[] = [];
    const program = programFactory(connection);
    const allDelegationsAccounts = await program.account.delegation.all();
    allDelegationsAccounts.forEach((item) => {
      item.account.authorised &&
        representations.push({
          authorised: item.account.authorised,
          master: item.account.master,
          representative: item.account.representative,
        });
    });

    return representations;
  } catch (error) {
    console.log(error);
    throw error;
  }
};

export const getDelegationAddress = (
  connection: Connection,
  master: PublicKey,
  representative: PublicKey
): PublicKey => {
  const program = programFactory(connection);
  const [delegation] = PublicKey.findProgramAddressSync(
    [authorizeSeed, master.toBuffer(), representative.toBuffer()],
    program.programId
  );
  return delegation;
};

export const checkIfDelegationExists = async (
  connection: Connection,
  master: PublicKey,
  representative: PublicKey
): Promise<IDelegation> => {
  try {
    const program = programFactory(connection);
    const delegationInfo = await program.account.delegation.fetch(
      getDelegationAddress(connection, master, representative)
    );
    if (!delegationInfo) {
      throw new Error('Delegation does not exist');
    }
    if (!delegationInfo.authorised) {
      throw new Error('Delegation not authorized');
    }
    return {
      authorised: delegationInfo.authorised,
      master: delegationInfo.master,
      representative: delegationInfo.representative,
    };
  } catch (error) {
    throw error;
  }
};

export default {
  getMastersForPublicKey,
  getRepresentativeForPublicKey,
  getAllMasters,
  getAllRepresentative,
  getAllMastersTokenAccountsWithSpecifiedMint,
  checkNumberOfTokenAccounts,
};
