import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EscrowAnchor } from "../target/types/escrow_anchor";
import { SystemProgram, Transaction } from "@solana/web3.js"
import { 
    createMint, 
    getOrCreateAssociatedTokenAccount, 
    mintTo, 
    getAccount, 
    Account, 
    TOKEN_PROGRAM_ID 
} from "@solana/spl-token";
import { expect } from "chai";

describe("escrow-anchor", () => {
    const provider = anchor.AnchorProvider.env();
    const connection = provider.connection;
    anchor.setProvider(provider);

    const program = anchor.workspace.EscrowAnchor as Program<EscrowAnchor>;

    let mintA: anchor.web3.PublicKey;
    let mintB: anchor.web3.PublicKey;
    let initializerTokenAccountA: Account;
    let initializerTokenAccountB: Account;
    let takerTokenAccountA: Account;
    let takerTokenAccountB: Account;

    const [vault_account_pda, _vault_account_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("token_seed")],
        program.programId
    );

    const [escrow_account_pda, _escrow_account_bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("escrow")],
        program.programId
    );

    const takerAmount = 1000;
    const initializerAmount = 500;

    const payer = anchor.web3.Keypair.generate();
    const mintAuthority = anchor.web3.Keypair.generate();
    const initializerMainAccount = anchor.web3.Keypair.generate();
    const takerMainAccount = anchor.web3.Keypair.generate();

    it("Initialize program state", async () => {
        let airdropSignature = await connection.requestAirdrop(
            payer.publicKey,
            10000000000,
        );
        await connection.confirmTransaction(airdropSignature, "confirmed");

        const fundTx = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: payer.publicKey,
                toPubkey: initializerMainAccount.publicKey,
                lamports: 1000000000,
            }),
            SystemProgram.transfer({
                fromPubkey: payer.publicKey,
                toPubkey: takerMainAccount.publicKey,
                lamports: 1000000000,
            })
        );
        await anchor.web3.sendAndConfirmTransaction(connection, fundTx, [payer]);

        mintA = await createMint(
            connection,
            payer,
            mintAuthority.publicKey,
            null,
            0,
        );

        mintB = await createMint(
            connection,
            payer,
            mintAuthority.publicKey,
            null,
            0,
        );

        initializerTokenAccountA  = await getOrCreateAssociatedTokenAccount(
            connection,
            payer,
            mintA,
            initializerMainAccount.publicKey,
        );

        takerTokenAccountA = await getOrCreateAssociatedTokenAccount(
            connection,
            payer,
            mintA,
            takerMainAccount.publicKey,

        );

        initializerTokenAccountB = await getOrCreateAssociatedTokenAccount(
            connection,
            payer,
            mintB,
            initializerMainAccount.publicKey,
        );

        takerTokenAccountB = await getOrCreateAssociatedTokenAccount(
            connection,
            payer,
            mintB,
            takerMainAccount.publicKey,
        );

        await mintTo(
            connection,
            payer,
            mintA,
            initializerTokenAccountA.address,
            mintAuthority,
            initializerAmount,
        );

        await mintTo(
            connection,
            payer,
            mintB,
            takerTokenAccountB.address,
            mintAuthority,
            takerAmount,
        );

        const initializerAccountA = await getAccount(
            connection,
            initializerTokenAccountA.address,
        );

        const takerAccountB = await getAccount(
            connection,
            takerTokenAccountB.address,
        );

        expect(Number(initializerAccountA.amount)).to.equal(initializerAmount)
        expect(Number(takerAccountB.amount)).to.equal(takerAmount)
    })

    it("Initialize escrow", async () => {

        await program.methods
            .initialize(
                new anchor.BN(initializerAmount), 
                new anchor.BN(takerAmount)
            )
            .accounts({
                initializer: initializerMainAccount.publicKey,
                mint: mintA,
                // vaultAccount: vault_account_pda,
                initializerDepositTokenAccount: initializerTokenAccountA.address,
                initializerReceiveTokenAccount: initializerTokenAccountB.address,
                // escrowAccount: escrow_account_pda,
                // systemProgram: SystemProgram.programId,
                // rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                // tokenProgram: TOKEN_PROGRAM_ID
            })
            .signers([
                initializerMainAccount,
                // escrowAccount
            ])
            .rpc()

        const vault = await getAccount(
            connection,
            vault_account_pda
        );

        const escrowData = await program.account.escrowAccount.fetch(escrow_account_pda);

        expect(vault.owner.toString()).to.equal(escrow_account_pda.toString())
        expect(Number(vault.amount)).to.equal(initializerAmount);

        expect(Number(escrowData.initializerAmount)).to.equal(initializerAmount);
        expect(escrowData.initializerKey.toString()).to.equal(initializerMainAccount.publicKey.toString());
        expect(escrowData.initializerDepositTokenAccount.toString()).to.equal(initializerTokenAccountA.address.toString());
        expect(escrowData.initializerReceiveTokenAccount.toString()).to.equal(initializerTokenAccountB.address.toString());
        expect(Number(escrowData.takerAmount)).to.equal(takerAmount)
    });

    it("Exchange escrow state", async () => {
        // TODO
    });

    it("Initialize escrow and cancel escrow", async () => {
        // TODO
    });

});
