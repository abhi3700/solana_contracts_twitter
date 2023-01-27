# Write Instruction

[SOURCE](./2_Our%20first%20instruction%20_%20Create%20a%20Solana%20dApp%20from%20scratch%20_%20Loris.pdf)

## Notes

In Solana, **programs in Solana are stateless**.

Because of that, sending an **instruction** to a **program** requires providing all the necessary context for it to run successfully.

Similarly to how we defined our `Tweet` account, contexts are implemented using a `struct`. Within that `struct`, we should list all the `accounts` that are necessary for the `instruction` to do its job.

In your `lib.rs` file, just above the `Tweet` struct we defined in the previous episode, you should see an empty `Initialize` context.
