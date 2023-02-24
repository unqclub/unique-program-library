export type DelegationManager = {
  version: '0.1.0';
  name: 'delegation_manager';
  docs: ["Unique program library's Delegation Manager program."];
  instructions: [
    {
      name: 'initializeDelegate';
      docs: ['Initializes delegation account'];
      accounts: [
        {
          name: 'master';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'representative';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'delegation';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'confirmDelegate';
      docs: ['Confirms delegation'];
      accounts: [
        {
          name: 'representative';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'delegation';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'cancelDelegate';
      docs: ['Cancels delegation'];
      accounts: [
        {
          name: 'delegation';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    }
  ];
  accounts: [
    {
      name: 'delegation';
      docs: ['State account storing the delegation'];
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'master';
            docs: ['The creator of the delegation'];
            type: 'publicKey';
          },
          {
            name: 'representative';
            docs: ['The wallet who delegates'];
            type: 'publicKey';
          },
          {
            name: 'authorised';
            docs: ['Confirmation flag'];
            type: 'bool';
          }
        ];
      };
    }
  ];
  errors: [
    {
      code: 6000;
      name: 'WrongRepresentative';
      msg: 'Wrong representative!';
    },
    {
      code: 6001;
      name: 'WrongMaster';
      msg: 'Wrong authority!';
    },
    {
      code: 6002;
      name: 'WrongSigner';
      msg: 'Wrong signer!';
    },
    {
      code: 6003;
      name: 'AlreadyAuthorised';
      msg: 'Authorization already approved!';
    },
    {
      code: 6004;
      name: 'NotAuthorized';
      msg: 'The account provided has no authority!';
    }
  ];
};

export const IDL: DelegationManager = {
  version: '0.1.0',
  name: 'delegation_manager',
  docs: ["Unique program library's Delegation Manager program."],
  instructions: [
    {
      name: 'initializeDelegate',
      docs: ['Initializes delegation account'],
      accounts: [
        {
          name: 'master',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'representative',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'delegation',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: 'confirmDelegate',
      docs: ['Confirms delegation'],
      accounts: [
        {
          name: 'representative',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'delegation',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: 'cancelDelegate',
      docs: ['Cancels delegation'],
      accounts: [
        {
          name: 'delegation',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
  ],
  accounts: [
    {
      name: 'delegation',
      docs: ['State account storing the delegation'],
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'master',
            docs: ['The creator of the delegation'],
            type: 'publicKey',
          },
          {
            name: 'representative',
            docs: ['The wallet who delegates'],
            type: 'publicKey',
          },
          {
            name: 'authorised',
            docs: ['Confirmation flag'],
            type: 'bool',
          },
        ],
      },
    },
  ],
  errors: [
    {
      code: 6000,
      name: 'WrongRepresentative',
      msg: 'Wrong representative!',
    },
    {
      code: 6001,
      name: 'WrongMaster',
      msg: 'Wrong authority!',
    },
    {
      code: 6002,
      name: 'WrongSigner',
      msg: 'Wrong signer!',
    },
    {
      code: 6003,
      name: 'AlreadyAuthorised',
      msg: 'Authorization already approved!',
    },
    {
      code: 6004,
      name: 'NotAuthorized',
      msg: 'The account provided has no authority!',
    },
  ],
};
