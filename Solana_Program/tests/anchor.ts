import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { ClmmBasic } from "../target/types/clmm_basic";
import { BN } from "@coral-xyz/anchor";
import assert from "assert";

describe("Create Pool Test", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ClmmBasic as anchor.Program<ClmmBasic>;

  const tokenA_mint_address = new web3.PublicKey(
    "2yUWpgR1cX453hmxnh7RJEspfoBt2JVzWyUKniXzVheY"
  );
  const tokenB_mint_address = new web3.PublicKey(
    "3fKqu7JLynEkyooG6TvBmq4djU6jsHuPivTcLgpKfhQQ"
  );

  const [pool_state_pda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("pool"),
      tokenA_mint_address.toBuffer(),
      tokenB_mint_address.toBuffer(),
    ],
    program.programId
  );

  const token_vault_0_keypair = web3.Keypair.generate();
  const token_vault_1_keypair = web3.Keypair.generate();

  const INITIAL_SQRT_PRICE = new BN(2).pow(new BN(64));
  const TICK_SPACING = 1;

  it("creates a liquidity pool of token A and token B", async () => {
    const poolInfo = await program.provider.connection.getAccountInfo(
      pool_state_pda
    );

    if (!poolInfo) {
      const ix = await program.methods
        .createPool(
          tokenA_mint_address,
          tokenB_mint_address,
          INITIAL_SQRT_PRICE,
          TICK_SPACING
        )
        .accounts({
          payer: program.provider.publicKey,
          pool: pool_state_pda,
          token0Vault: token_vault_0_keypair.publicKey,
          token1Vault: token_vault_1_keypair.publicKey,
          token0Mint: tokenA_mint_address,
          token1Mint: tokenB_mint_address,
          systemProgram: web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: web3.SYSVAR_RENT_PUBKEY,
        })
        .instruction();

      // Add vault keypairs as additional signers
      ix.keys.push(
        {
          pubkey: token_vault_0_keypair.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: token_vault_1_keypair.publicKey,
          isSigner: true,
          isWritable: true,
        }
      );

      const tx = new web3.Transaction().add(ix);
      const txHash = await program.provider.sendAndConfirm(tx, [
        token_vault_0_keypair,
        token_vault_1_keypair,
      ]);

      console.log(
        `Pool created. Use 'solana confirm -v ${txHash}' to see the logs`
      );
    } else {
      console.log("Pool already exists, skipping creation");
    }

    const poolState = await program.account.pool.fetch(pool_state_pda);

    assert.ok(
      poolState.token0Mint.equals(tokenA_mint_address),
      "Token 0 mint mismatch"
    );
    assert.ok(
      poolState.token1Mint.equals(tokenB_mint_address),
      "Token 1 mint mismatch"
    );
    assert.ok(
      poolState.sqrtPrice.eq(INITIAL_SQRT_PRICE),
      "Initial sqrt price mismatch"
    );
    assert.strictEqual(poolState.currentTick, 0, "Initial tick should be 0");
    assert.ok(
      poolState.liquidity.eq(new BN(0)),
      "Initial liquidity should be 0"
    );
    assert.strictEqual(
      poolState.tickSpacing,
      TICK_SPACING,
      "Tick spacing mismatch"
    );

    assert.ok(
      poolState.token0Vault.equals(token_vault_0_keypair.publicKey),
      "Token 0 vault mismatch"
    );
    assert.ok(
      poolState.token1Vault.equals(token_vault_1_keypair.publicKey),
      "Token 1 vault mismatch"
    );

    console.log("Pool state verified:", {
      token0Mint: poolState.token0Mint.toString(),
      token1Mint: poolState.token1Mint.toString(),
      token0Vault: poolState.token0Vault.toString(),
      token1Vault: poolState.token1Vault.toString(),
      sqrtPrice: poolState.sqrtPrice.toString(),
      currentTick: poolState.currentTick,
      liquidity: poolState.liquidity.toString(),
      tickSpacing: poolState.tickSpacing,
      bump: poolState.bump,
    });
  });
});
