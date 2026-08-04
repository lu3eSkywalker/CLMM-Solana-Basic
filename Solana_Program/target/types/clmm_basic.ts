/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/clmm_basic.json`.
 */
export type ClmmBasic = {
  "address": "EzsJ9BEKsa161p4iYoWymzzYY8gnd5uWqAnprekyrMDt",
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
    }
  ]
};
