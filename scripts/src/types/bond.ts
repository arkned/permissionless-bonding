export type Bond = {
  version: "0.1.0";
  name: "bond";
  instructions: [
    {
      name: "initNewProject";
      accounts: [
        {
          name: "initializer";
          isMut: true;
          isSigner: true;
        },
        {
          name: "tokenMint";
          isMut: false;
          isSigner: false;
        },
        {
          name: "tokenAccount";
          isMut: true;
          isSigner: false;
        },
        {
          name: "lpMint";
          isMut: false;
          isSigner: false;
        },
        {
          name: "lpTokenAccount";
          isMut: false;
          isSigner: false;
        },
        {
          name: "vaultAccount";
          isMut: true;
          isSigner: false;
        },
        {
          name: "projectInfo";
          isMut: true;
          isSigner: false;
        },
        {
          name: "rent";
          isMut: false;
          isSigner: false;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        },
        {
          name: "tokenProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "amount";
          type: "u64";
        },
        {
          name: "price";
          type: "u64";
        },
        {
          name: "discountSettings";
          type: {
            defined: "DiscountSettings";
          };
        },
        {
          name: "vestingSchedule";
          type: {
            defined: "VestingSchedule";
          };
        }
      ];
    },
    {
      name: "updateAuthority";
      accounts: [
        {
          name: "user";
          isMut: true;
          isSigner: true;
        },
        {
          name: "projectInfo";
          isMut: true;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "newAuthority";
          type: "publicKey";
        }
      ];
    }
  ];
  accounts: [
    {
      name: "projectInfo";
      type: {
        kind: "struct";
        fields: [
          {
            name: "projectOwner";
            type: "publicKey";
          },
          {
            name: "projectToken";
            type: "publicKey";
          },
          {
            name: "lpToken";
            type: "publicKey";
          },
          {
            name: "lpTokenAccount";
            type: "publicKey";
          },
          {
            name: "tokenAmount";
            type: "u64";
          },
          {
            name: "price";
            type: "u64";
          },
          {
            name: "minDiscout";
            type: "u64";
          },
          {
            name: "maxDiscount";
            type: "u64";
          },
          {
            name: "discountMode";
            type: "u64";
          },
          {
            name: "releaseInterval";
            type: "u64";
          },
          {
            name: "releaseRate";
            type: "u64";
          },
          {
            name: "instantUnlock";
            type: "u64";
          },
          {
            name: "initialUnlock";
            type: "u64";
          },
          {
            name: "lockPeriod";
            type: "u64";
          },
          {
            name: "vestingPeriod";
            type: "u64";
          },
          {
            name: "bondedLpAmount";
            type: "u64";
          },
          {
            name: "vestedAmount";
            type: "u64";
          }
        ];
      };
    },
    {
      name: "bondsInfo";
      type: {
        kind: "struct";
        fields: [
          {
            name: "totalBonds";
            type: "u64";
          }
        ];
      };
    },
    {
      name: "vestingInfo";
      type: {
        kind: "struct";
        fields: [
          {
            name: "totalAmount";
            type: "u64";
          },
          {
            name: "withdrawnAmount";
            type: "u64";
          },
          {
            name: "startTime";
            type: "u64";
          }
        ];
      };
    }
  ];
  types: [
    {
      name: "VestingSchedule";
      type: {
        kind: "struct";
        fields: [
          {
            name: "releaseInterval";
            type: "u64";
          },
          {
            name: "releaseRate";
            type: "u64";
          },
          {
            name: "instantUnlock";
            type: "u64";
          },
          {
            name: "initialUnlock";
            type: "u64";
          },
          {
            name: "lockPeriod";
            type: "u64";
          },
          {
            name: "vestingPeriod";
            type: "u64";
          }
        ];
      };
    },
    {
      name: "DiscountSettings";
      type: {
        kind: "struct";
        fields: [
          {
            name: "minDiscout";
            type: "u64";
          },
          {
            name: "maxDiscount";
            type: "u64";
          },
          {
            name: "discountMode";
            type: "u64";
          }
        ];
      };
    }
  ];
};

export const IDL: Bond = {
  version: "0.1.0",
  name: "bond",
  instructions: [
    {
      name: "initNewProject",
      accounts: [
        {
          name: "initializer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "tokenMint",
          isMut: false,
          isSigner: false,
        },
        {
          name: "tokenAccount",
          isMut: true,
          isSigner: false,
        },
        {
          name: "lpMint",
          isMut: false,
          isSigner: false,
        },
        {
          name: "lpTokenAccount",
          isMut: false,
          isSigner: false,
        },
        {
          name: "vaultAccount",
          isMut: true,
          isSigner: false,
        },
        {
          name: "projectInfo",
          isMut: true,
          isSigner: false,
        },
        {
          name: "rent",
          isMut: false,
          isSigner: false,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
        {
          name: "tokenProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "amount",
          type: "u64",
        },
        {
          name: "price",
          type: "u64",
        },
        {
          name: "discountSettings",
          type: {
            defined: "DiscountSettings",
          },
        },
        {
          name: "vestingSchedule",
          type: {
            defined: "VestingSchedule",
          },
        },
      ],
    },
    {
      name: "updateAuthority",
      accounts: [
        {
          name: "user",
          isMut: true,
          isSigner: true,
        },
        {
          name: "projectInfo",
          isMut: true,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "newAuthority",
          type: "publicKey",
        },
      ],
    },
  ],
  accounts: [
    {
      name: "projectInfo",
      type: {
        kind: "struct",
        fields: [
          {
            name: "projectOwner",
            type: "publicKey",
          },
          {
            name: "projectToken",
            type: "publicKey",
          },
          {
            name: "lpToken",
            type: "publicKey",
          },
          {
            name: "lpTokenAccount",
            type: "publicKey",
          },
          {
            name: "tokenAmount",
            type: "u64",
          },
          {
            name: "price",
            type: "u64",
          },
          {
            name: "minDiscout",
            type: "u64",
          },
          {
            name: "maxDiscount",
            type: "u64",
          },
          {
            name: "discountMode",
            type: "u64",
          },
          {
            name: "releaseInterval",
            type: "u64",
          },
          {
            name: "releaseRate",
            type: "u64",
          },
          {
            name: "instantUnlock",
            type: "u64",
          },
          {
            name: "initialUnlock",
            type: "u64",
          },
          {
            name: "lockPeriod",
            type: "u64",
          },
          {
            name: "vestingPeriod",
            type: "u64",
          },
          {
            name: "bondedLpAmount",
            type: "u64",
          },
          {
            name: "vestedAmount",
            type: "u64",
          },
        ],
      },
    },
    {
      name: "bondsInfo",
      type: {
        kind: "struct",
        fields: [
          {
            name: "totalBonds",
            type: "u64",
          },
        ],
      },
    },
    {
      name: "vestingInfo",
      type: {
        kind: "struct",
        fields: [
          {
            name: "totalAmount",
            type: "u64",
          },
          {
            name: "withdrawnAmount",
            type: "u64",
          },
          {
            name: "startTime",
            type: "u64",
          },
        ],
      },
    },
  ],
  types: [
    {
      name: "VestingSchedule",
      type: {
        kind: "struct",
        fields: [
          {
            name: "releaseInterval",
            type: "u64",
          },
          {
            name: "releaseRate",
            type: "u64",
          },
          {
            name: "instantUnlock",
            type: "u64",
          },
          {
            name: "initialUnlock",
            type: "u64",
          },
          {
            name: "lockPeriod",
            type: "u64",
          },
          {
            name: "vestingPeriod",
            type: "u64",
          },
        ],
      },
    },
    {
      name: "DiscountSettings",
      type: {
        kind: "struct",
        fields: [
          {
            name: "minDiscout",
            type: "u64",
          },
          {
            name: "maxDiscount",
            type: "u64",
          },
          {
            name: "discountMode",
            type: "u64",
          },
        ],
      },
    },
  ],
};
