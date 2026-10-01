/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/clmm_basic.json`.
 */
export type ClmmBasic = {
  "address": "BrZPVRu7HgWe9yZM3XiDx9qRdbXwR8RBFW4JiyanQm75",
  "metadata": {
    "name": "clmmBasic",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "createPool",
      "discriminator": [
        233,
        146,
        209,
        142,
        207,
        104,
        64,
        188
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "token0Mint"
              },
              {
                "kind": "arg",
                "path": "token1Mint"
              }
            ]
          }
        },
        {
          "name": "token0Vault",
          "writable": true
        },
        {
          "name": "token1Vault",
          "writable": true
        },
        {
          "name": "token0Mint"
        },
        {
          "name": "token1Mint"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "token0Mint",
          "type": "pubkey"
        },
        {
          "name": "token1Mint",
          "type": "pubkey"
        },
        {
          "name": "initialSqrtPrice",
          "type": "u128"
        },
        {
          "name": "tickSpacing",
          "type": "u16"
        }
      ]
    },
    {
      "name": "decreaseLiquidity",
      "discriminator": [
        160,
        38,
        208,
        111,
        104,
        91,
        44,
        1
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "position",
          "writable": true
        },
        {
          "name": "pool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.token0_mint",
                "account": "pool"
              },
              {
                "kind": "account",
                "path": "pool.token1_mint",
                "account": "pool"
              }
            ]
          }
        },
        {
          "name": "lowerTickArray",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  105,
                  99,
                  107,
                  95,
                  97,
                  114,
                  114,
                  97,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              },
              {
                "kind": "arg",
                "path": "lowerTickArrayStart"
              }
            ]
          }
        },
        {
          "name": "upperTickArray",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  105,
                  99,
                  107,
                  95,
                  97,
                  114,
                  114,
                  97,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              },
              {
                "kind": "arg",
                "path": "upperTickArrayStart"
              }
            ]
          }
        },
        {
          "name": "userToken0",
          "writable": true
        },
        {
          "name": "userToken1",
          "writable": true
        },
        {
          "name": "poolToken0Vault",
          "writable": true
        },
        {
          "name": "poolToken1Vault",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "liquidityToRemove",
          "type": "u128"
        },
        {
          "name": "amount0Min",
          "type": "u64"
        },
        {
          "name": "amount1Min",
          "type": "u64"
        },
        {
          "name": "lowerTick",
          "type": "i32"
        },
        {
          "name": "upperTick",
          "type": "i32"
        },
        {
          "name": "lowerTickArrayStart",
          "type": "i32"
        },
        {
          "name": "upperTickArrayStart",
          "type": "i32"
        }
      ]
    },
    {
      "name": "increaseLiquidity",
      "discriminator": [
        46,
        156,
        243,
        118,
        13,
        205,
        251,
        178
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "position",
          "writable": true
        },
        {
          "name": "pool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.token0_mint",
                "account": "pool"
              },
              {
                "kind": "account",
                "path": "pool.token1_mint",
                "account": "pool"
              }
            ]
          }
        },
        {
          "name": "lowerTickArray",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  105,
                  99,
                  107,
                  95,
                  97,
                  114,
                  114,
                  97,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              },
              {
                "kind": "arg",
                "path": "lowerTickArrayStart"
              }
            ]
          }
        },
        {
          "name": "upperTickArray",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  105,
                  99,
                  107,
                  95,
                  97,
                  114,
                  114,
                  97,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              },
              {
                "kind": "arg",
                "path": "upperTickArrayStart"
              }
            ]
          }
        },
        {
          "name": "userToken0",
          "writable": true
        },
        {
          "name": "userToken1",
          "writable": true
        },
        {
          "name": "poolToken0Vault",
          "writable": true
        },
        {
          "name": "poolToken1Vault",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount0Max",
          "type": "u64"
        },
        {
          "name": "amount1Max",
          "type": "u64"
        },
        {
          "name": "lowerTick",
          "type": "i32"
        },
        {
          "name": "upperTick",
          "type": "i32"
        },
        {
          "name": "lowerTickArrayStart",
          "type": "i32"
        },
        {
          "name": "upperTickArrayStart",
          "type": "i32"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "pool",
      "discriminator": [
        241,
        154,
        109,
        4,
        17,
        177,
        109,
        188
      ]
    },
    {
      "name": "position",
      "discriminator": [
        170,
        188,
        143,
        228,
        122,
        64,
        247,
        208
      ]
    },
    {
      "name": "tickArray",
      "discriminator": [
        69,
        97,
        189,
        190,
        110,
        7,
        66,
        187
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "identicalTokenMints",
      "msg": "Token mints must be different"
    },
    {
      "code": 6001,
      "name": "invalidInitialPrice",
      "msg": "Initial sqrt price must be greater than 0"
    },
    {
      "code": 6002,
      "name": "invalidTickSpacing",
      "msg": "Tick spacing must be greater than 0"
    },
    {
      "code": 6003,
      "name": "invalidInitialTick",
      "msg": "Initial tick is not aligned with tick spacing"
    },
    {
      "code": 6004,
      "name": "invalidTick",
      "msg": "Tick out of bounds"
    },
    {
      "code": 6005,
      "name": "invalidSqrtPrice",
      "msg": "Invalid sqrt price"
    },
    {
      "code": 6006,
      "name": "invalidPriceRange",
      "msg": "Invalid price range"
    },
    {
      "code": 6007,
      "name": "mathOverflow",
      "msg": "Math overflow"
    },
    {
      "code": 6008,
      "name": "tickAlreadyInitialized",
      "msg": "Tick already initialized"
    },
    {
      "code": 6009,
      "name": "tickNotInitialized",
      "msg": "Tick not initialized"
    },
    {
      "code": 6010,
      "name": "liquidityOverflow",
      "msg": "Liquidity overflow"
    },
    {
      "code": 6011,
      "name": "liquidityUnderflow",
      "msg": "Liquidity underflow"
    },
    {
      "code": 6012,
      "name": "tickStillInUse",
      "msg": "Tick still in use"
    },
    {
      "code": 6013,
      "name": "tickArrayNotInitialized",
      "msg": "Tick array not initialized"
    },
    {
      "code": 6014,
      "name": "tickOutOfRange",
      "msg": "Tick out of range"
    },
    {
      "code": 6015,
      "name": "tickNotAligned",
      "msg": "Tick not aligned with spacing"
    },
    {
      "code": 6016,
      "name": "invalidStartTick",
      "msg": "Invalid start tick"
    },
    {
      "code": 6017,
      "name": "arithmeticOverflow",
      "msg": "Arithmetic overflow"
    },
    {
      "code": 6018,
      "name": "tickArrayAlreadyInitialized",
      "msg": "Tick array already initialized"
    },
    {
      "code": 6019,
      "name": "invalidLiquidity",
      "msg": "Invalid liquidity amount"
    },
    {
      "code": 6020,
      "name": "liquiditySubValueError",
      "msg": "Liquidity subtraction value error"
    },
    {
      "code": 6021,
      "name": "liquidityAddValueError",
      "msg": "Liquidity addition value error"
    },
    {
      "code": 6022,
      "name": "maxTokenOverflow",
      "msg": "Max token overflow"
    },
    {
      "code": 6023,
      "name": "positionAlreadyInitialized",
      "msg": "Position already initialized"
    },
    {
      "code": 6024,
      "name": "invalidTickArrayPool",
      "msg": "Tick array belongs to a different pool"
    },
    {
      "code": 6025,
      "name": "slippageExceeded",
      "msg": "Slippage exceeded"
    },
    {
      "code": 6026,
      "name": "positionNotInitialized",
      "msg": "Position not initialized"
    },
    {
      "code": 6027,
      "name": "positionPoolMismatch",
      "msg": "Position does not belong to pool"
    },
    {
      "code": 6028,
      "name": "positionOwnerMismatch",
      "msg": "Position owner mismatch"
    },
    {
      "code": 6029,
      "name": "tokenAccountMintMismatch",
      "msg": "Token account mint mismatch"
    }
  ],
  "types": [
    {
      "name": "pool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "token0Mint",
            "type": "pubkey"
          },
          {
            "name": "token1Mint",
            "type": "pubkey"
          },
          {
            "name": "token0Vault",
            "type": "pubkey"
          },
          {
            "name": "token1Vault",
            "type": "pubkey"
          },
          {
            "name": "sqrtPrice",
            "type": "u128"
          },
          {
            "name": "currentTick",
            "type": "i32"
          },
          {
            "name": "liquidity",
            "type": "u128"
          },
          {
            "name": "tickSpacing",
            "type": "u16"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "position",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "pool",
            "type": "pubkey"
          },
          {
            "name": "lowerTick",
            "type": "i32"
          },
          {
            "name": "upperTick",
            "type": "i32"
          },
          {
            "name": "liquidity",
            "type": "u128"
          },
          {
            "name": "amount0",
            "type": "u64"
          },
          {
            "name": "amount1",
            "type": "u64"
          },
          {
            "name": "initialized",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "tickArray",
      "serialization": "bytemuck",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "pubkey"
          },
          {
            "name": "startTickIndex",
            "type": "i32"
          },
          {
            "name": "tickSpacing",
            "type": "u16"
          },
          {
            "name": "initialized",
            "type": "u8"
          },
          {
            "name": "padding",
            "type": "u8"
          },
          {
            "name": "ticks",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "tick"
                  }
                },
                88
              ]
            }
          }
        ]
      }
    }
  ]
};
